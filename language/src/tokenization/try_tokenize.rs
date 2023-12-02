use std::iter::Peekable;
use std::str::CharIndices;

use super::byte_index::ByteIndex;
use super::byte_size::ByteSize;
use super::character_position::CharacterPosition;
use super::constants::{
    ARROW, ASSIGNMENT, ASTERISK, CLOSE_BRACE, CLOSE_PARENTHESES, COLON, COMMA,
    COMMENT_BLOCK_END, COMMENT_BLOCK_START, COMMENT_LINE_START, DOT, EQUAL_TO,
    F32_NAME, F64_NAME, GREATER_THAN, GREATER_THAN_OR_EQUAL_TO, I16_NAME,
    I32_NAME, I64_NAME, I8_NAME, ISIZE_NAME, LOWER_THAN,
    LOWER_THAN_OR_EQUAL_TO, MINUS, NEWLINE,
    NON_STARTING_IDENTIFIER_CHARACTERS, NOT_EQUAL_TO, NUMERIC_CHARACTERS,
    OPEN_BRACE, OPEN_PARENTHESES, PLUS, SEMICOLON, SLASH,
    STARTING_IDENTIFIER_CHARACTERS, TYPE_SUFFIX_SEPARATOR,
    TYPE_SUFFIX_SEPARATOR_STRING, U16_NAME, U32_NAME, U64_NAME, U8_NAME,
    USIZE_NAME,
};
use super::lexical_error::{
    CommentBlockIncomplete, IdentifierIncomplete,
    IdentifierUnexpectedCharacter, LexicalError,
    NumericLiteralTypeSuffixIncomplete,
    NumericLiteralTypeSuffixUnexpectedCharacter,
    NumericLiteralTypeSuffixUnknown, NumericLiteralValueIncomplete,
    NumericLiteralValueTypeSuffixConflict,
    NumericLiteralValueUnexpectedCharacter, UnexpectedCharacter,
};
use super::numeric_literal_type::NumericLiteralType;
use super::numeric_literal_value_kind::NumericLiteralValueKind;
use super::substring_position::SubstringPosition;
use super::token::Token;
use super::token_content::TokenContent;
use super::utf_8_index::Utf8Index;
use super::utf_8_size::Utf8Size;

pub trait TryTokenize<StringType> {
    fn try_tokenize(
        self,
    ) -> Result<Vec<Token<StringType>>, LexicalError<StringType>>;
}

impl<'a, StringType: From<&'a str>> TryTokenize<StringType> for &'a str {
    fn try_tokenize(
        self,
    ) -> Result<Vec<Token<StringType>>, LexicalError<StringType>> {
        Tokens::try_from(self).map(move |tokens| tokens.0)
    }
}

struct Tokens<StringType>(Vec<Token<StringType>>);

impl<'a, StringType: From<&'a str>> TryFrom<&'a str> for Tokens<StringType> {
    type Error = LexicalError<StringType>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut enumerated_lines = value.split_inclusive(NEWLINE).enumerate();
        let mut tokens = Vec::new();
        'lines: while let Some((mut line_index, mut line)) =
            enumerated_lines.next()
        {
            let mut positioned_characters =
                PositionedCharacters::from(line.char_indices()).peekable();
            'characters: while let Some((
                mut character_position,
                mut character,
            )) = positioned_characters.next()
            {
                if is_non_newline_whitespace(character) {
                    let whitespace_start_character_position =
                        character_position;
                    loop {
                        if let Some((candidate_position, candidate)) =
                            positioned_characters.next()
                        {
                            if !is_non_newline_whitespace(candidate) {
                                tokens.push(Token {
                                    position: SubstringPosition {
                                        start_line: line_index,
                                        end_line: line_index,
                                        start_character: whitespace_start_character_position,
                                        end_character: candidate_position,
                                    },
                                    content: TokenContent::Whitespace(
                                        StringType::from(slice_string_by_positions(
                                            line,
                                            whitespace_start_character_position,
                                            candidate_position,
                                        )),
                                    ),
                                });
                                (character_position, character) =
                                    (candidate_position, candidate);
                                break;
                            }
                        } else {
                            tokens.push(Token {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    end_line: line_index,
                                    start_character: whitespace_start_character_position,
                                    end_character: to_line_end_position(line),
                                },
                                content: TokenContent::Whitespace(
                                    StringType::from(tail_string_from_position(line, whitespace_start_character_position)),
                                ),
                            });
                            break 'characters;
                        }
                    }
                }
                debug_assert!(!is_non_newline_whitespace(character));
                if character == SLASH {
                    if let Some((_, '/')) = positioned_characters.peek() {
                        let _ = positioned_characters.next();
                        debug_assert_eq!(
                            slice_string_by_byte_indices(
                                line,
                                character_position.byte,
                                character_position.byte
                                    + COMMENT_LINE_START.byte_size(),
                            ),
                            COMMENT_LINE_START
                        );
                        tokens.push(Token {
                            position: SubstringPosition {
                                start_line: line_index,
                                start_character: character_position,
                                end_line: line_index,
                                end_character: to_line_end_position(line),
                            },
                            content: TokenContent::CommentLine(
                                StringType::from(tail_string_from_position(
                                    line,
                                    character_position,
                                )),
                            ),
                        });
                        continue 'lines;
                    } else if let Some((_, '*')) = positioned_characters.peek()
                    {
                        let _ = positioned_characters.next();
                        let comment_block_line_start = line_index;
                        let comment_block_start_character_position =
                            character_position;
                        let comment_block_first_non_start_byte_index =
                            comment_block_start_character_position.byte
                                + COMMENT_BLOCK_START.byte_size();
                        if let Some(
                            candidate_comment_block_end_first_byte_index,
                        ) = tail_string_from_byte_index(
                            line,
                            comment_block_first_non_start_byte_index,
                        )
                        .find(COMMENT_BLOCK_END)
                        .map(ByteIndex::from)
                        {
                            let comment_block_end_byte_index =
                                comment_block_first_non_start_byte_index
                                    + candidate_comment_block_end_first_byte_index
                                    + COMMENT_BLOCK_END.byte_size();
                            let non_start_comment_block_characters_count =
                                slice_string_by_byte_indices(
                                    line,
                                    comment_block_first_non_start_byte_index,
                                    comment_block_end_byte_index,
                                )
                                .utf_8_size();
                            for _ in 0usize
                                ..non_start_comment_block_characters_count
                                    .into()
                            {
                                let _ = positioned_characters.next();
                            }
                            let comment_block_column_end = positioned_characters
                                .peek()
                                .map(|(position, _)| *position)
                                .unwrap_or_else(|| CharacterPosition {
                                    byte: line.byte_size(),
                                    utf_8: comment_block_start_character_position.utf_8
                                        + COMMENT_BLOCK_START.utf_8_size()
                                        + non_start_comment_block_characters_count,
                                });
                            tokens.push(Token {
                                position: SubstringPosition {
                                    start_line: comment_block_line_start,
                                    start_character:
                                    comment_block_start_character_position,
                                    end_line: comment_block_line_start,
                                    end_character: comment_block_column_end,
                                },
                                content: TokenContent::CommentBlock(vec![
                                    StringType::from(
                                        slice_string_by_positions(
                                            line,
                                            comment_block_start_character_position,
                                            comment_block_column_end,
                                        ),
                                    ),
                                ]),
                            });
                            continue;
                        } else {
                            let mut comment_block_lines =
                                vec![StringType::from(
                                    tail_string_from_position(
                                        line,
                                        comment_block_start_character_position,
                                    ),
                                )];
                            loop {
                                if let Some((next_line_index, next_line)) =
                                    enumerated_lines.next()
                                {
                                    line = next_line;
                                    line_index = next_line_index;
                                    if let Some(
                                        comment_block_column_byte_offset,
                                    ) = line
                                        .find(COMMENT_BLOCK_END)
                                        .map(ByteIndex::from)
                                    {
                                        let comment_block_column_end_byte_index =
                                            comment_block_column_byte_offset
                                                + COMMENT_BLOCK_END.byte_size();
                                        let comment_block_ending = behead_string_until_byte_index(
                                            line,
                                            comment_block_column_end_byte_index,
                                        );
                                        comment_block_lines.push(
                                            StringType::from(
                                                comment_block_ending,
                                            ),
                                        );
                                        positioned_characters =
                                            PositionedCharacters::from(
                                                line.char_indices(),
                                            )
                                            .peekable();
                                        for _ in 0usize
                                            ..comment_block_ending
                                                .utf_8_size()
                                                .into()
                                        {
                                            let _ =
                                                positioned_characters.next();
                                        }
                                        let comment_block_column_end =
                                            positioned_characters
                                                .peek()
                                                .map(|(value, _)| *value)
                                                .unwrap_or_else(|| {
                                                    to_line_end_position(line)
                                                });
                                        tokens.push(Token {
                                            position: SubstringPosition {
                                                start_line: comment_block_line_start,
                                                start_character:
                                                comment_block_start_character_position,
                                                end_line: line_index,
                                                end_character: comment_block_column_end,
                                            },
                                            content: TokenContent::CommentBlock(comment_block_lines),
                                        });
                                        continue 'characters;
                                    } else {
                                        comment_block_lines
                                            .push(StringType::from(line));
                                    }
                                } else {
                                    return Err(LexicalError::CommentBlockIncomplete(
                                        CommentBlockIncomplete {
                                            strings: comment_block_lines,
                                            position: SubstringPosition {
                                                start_line: comment_block_line_start,
                                                start_character: comment_block_start_character_position,
                                                end_line: line_index,
                                                end_character: to_line_end_position(line),
                                            },
                                        },
                                    ));
                                }
                            }
                        }
                    } else {
                        tokens.push(Token {
                            position: SubstringPosition {
                                start_line: line_index,
                                start_character: character_position,
                                end_line: line_index,
                                end_character: CharacterPosition {
                                    byte: character_position.byte
                                        + SLASH.byte_size(),
                                    utf_8: character_position.utf_8
                                        + SLASH.utf_8_size(),
                                },
                            },
                            content: TokenContent::Slash,
                        });
                        continue;
                    }
                }
                let token = match character {
                    ASSIGNMENT => {
                        if let Some((_, '=')) = positioned_characters.peek() {
                            let _ = positioned_characters.next();
                            Token {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + EQUAL_TO.byte_size(),
                                        utf_8: character_position.utf_8
                                            + EQUAL_TO.utf_8_size(),
                                    },
                                },
                                content: TokenContent::EqualTo,
                            }
                        } else {
                            Token {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + ASSIGNMENT.byte_size(),
                                        utf_8: character_position.utf_8
                                            + ASSIGNMENT.utf_8_size(),
                                    },
                                },
                                content: TokenContent::Assignment,
                            }
                        }
                    }
                    ASTERISK => Token {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + ASTERISK.byte_size(),
                                utf_8: character_position.utf_8
                                    + ASTERISK.utf_8_size(),
                            },
                        },
                        content: TokenContent::Asterisk,
                    },
                    CLOSE_BRACE => Token {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + CLOSE_BRACE.byte_size(),
                                utf_8: character_position.utf_8
                                    + CLOSE_BRACE.utf_8_size(),
                            },
                        },
                        content: TokenContent::CloseBrace,
                    },
                    CLOSE_PARENTHESES => Token {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + CLOSE_PARENTHESES.byte_size(),
                                utf_8: character_position.utf_8
                                    + CLOSE_PARENTHESES.utf_8_size(),
                            },
                        },
                        content: TokenContent::CloseParenthesis,
                    },
                    COLON => Token {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + COLON.byte_size(),
                                utf_8: character_position.utf_8
                                    + COLON.utf_8_size(),
                            },
                        },
                        content: TokenContent::Colon,
                    },
                    COMMA => Token {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + COMMA.byte_size(),
                                utf_8: character_position.utf_8
                                    + COMMA.utf_8_size(),
                            },
                        },
                        content: TokenContent::Comma,
                    },
                    DOT => {
                        if matches!(
                            positioned_characters.peek(),
                            Some((_, candidate)) if is_numeric_character(*candidate)
                        ) {
                            // "floating-point numeric literal starting with dot" case
                            let _ = positioned_characters.next();
                            parse_floating_point_literal_starting_with_dot(
                                &mut positioned_characters,
                                line_index,
                                line,
                                character_position,
                            )?
                        } else {
                            // "dot operator" case
                            Token {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    end_line: line_index,
                                    start_character: character_position,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + DOT.byte_size(),
                                        utf_8: character_position.utf_8
                                            + DOT.utf_8_size(),
                                    },
                                },
                                content: TokenContent::Dot,
                            }
                        }
                    }
                    GREATER_THAN => {
                        if let Some((_, '=')) = positioned_characters.peek() {
                            let _ = positioned_characters.next();
                            Token {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + GREATER_THAN_OR_EQUAL_TO
                                                .byte_size(),
                                        utf_8: character_position.utf_8
                                            + GREATER_THAN_OR_EQUAL_TO
                                                .utf_8_size(),
                                    },
                                },
                                content: TokenContent::GreaterThanOrEqualTo,
                            }
                        } else {
                            Token {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + GREATER_THAN.byte_size(),
                                        utf_8: character_position.utf_8
                                            + GREATER_THAN.utf_8_size(),
                                    },
                                },
                                content: TokenContent::GreaterThan,
                            }
                        }
                    }
                    LOWER_THAN => {
                        if let Some((_, '=')) = positioned_characters.peek() {
                            let _ = positioned_characters.next();
                            Token {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + LOWER_THAN_OR_EQUAL_TO
                                                .byte_size(),
                                        utf_8: character_position.utf_8
                                            + LOWER_THAN_OR_EQUAL_TO
                                                .utf_8_size(),
                                    },
                                },
                                content: TokenContent::LowerThanOrEqualTo,
                            }
                        } else {
                            Token {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + LOWER_THAN.byte_size(),
                                        utf_8: character_position.utf_8
                                            + LOWER_THAN.utf_8_size(),
                                    },
                                },
                                content: TokenContent::LowerThan,
                            }
                        }
                    }
                    MINUS => {
                        if matches!(
                            positioned_characters.peek(),
                            Some((_, '>'))
                        ) {
                            let _ = positioned_characters.next();
                            Token {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + ARROW.byte_size(),
                                        utf_8: character_position.utf_8
                                            + ARROW.utf_8_size(),
                                    },
                                },
                                content: TokenContent::Arrow,
                            }
                        } else {
                            Token {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + MINUS.byte_size(),
                                        utf_8: character_position.utf_8
                                            + MINUS.utf_8_size(),
                                    },
                                },
                                content: TokenContent::Minus,
                            }
                        }
                    }
                    NEWLINE => Token {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + NEWLINE.byte_size(),
                                utf_8: character_position.utf_8
                                    + NEWLINE.utf_8_size(),
                            },
                        },
                        content: TokenContent::Newline,
                    },
                    OPEN_BRACE => Token {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + OPEN_BRACE.byte_size(),
                                utf_8: character_position.utf_8
                                    + OPEN_BRACE.utf_8_size(),
                            },
                        },
                        content: TokenContent::OpenBrace,
                    },
                    OPEN_PARENTHESES => Token {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + OPEN_PARENTHESES.byte_size(),
                                utf_8: character_position.utf_8
                                    + OPEN_PARENTHESES.utf_8_size(),
                            },
                        },
                        content: TokenContent::OpenParenthesis,
                    },
                    PLUS => Token {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + PLUS.byte_size(),
                                utf_8: character_position.utf_8
                                    + PLUS.utf_8_size(),
                            },
                        },
                        content: TokenContent::Plus,
                    },
                    SEMICOLON => Token {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + SEMICOLON.byte_size(),
                                utf_8: character_position.utf_8
                                    + SEMICOLON.utf_8_size(),
                            },
                        },
                        content: TokenContent::Semicolon,
                    },
                    '!' => {
                        if let Some((_, '=')) = positioned_characters.peek() {
                            let _ = positioned_characters.next();
                            Token {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + NOT_EQUAL_TO.byte_size(),
                                        utf_8: character_position.utf_8
                                            + NOT_EQUAL_TO.utf_8_size(),
                                    },
                                },
                                content: TokenContent::NotEqualTo,
                            }
                        } else {
                            return Err(LexicalError::UnexpectedCharacter(
                                UnexpectedCharacter {
                                    character,
                                    position: SubstringPosition {
                                        start_line: line_index,
                                        end_line: line_index,
                                        start_character: character_position,
                                        end_character: to_line_end_position(
                                            line,
                                        ),
                                    },
                                    string: StringType::from(
                                        tail_string_from_position(
                                            line,
                                            character_position,
                                        ),
                                    ),
                                },
                            ));
                        }
                    }
                    _ if is_numeric_character(character) => {
                        parse_numeric_literal(
                            &mut positioned_characters,
                            line_index,
                            line,
                            character_position,
                        )?
                    }
                    _ if is_starting_identifier_character(character) => {
                        let (
                            identifier_start_character_position,
                            identifier_start_character,
                        ) = (character_position, character);
                        let identifier_end_character_position =
                            parse_non_starting_identifier_characters(
                                &mut positioned_characters,
                                identifier_start_character_position,
                                identifier_start_character,
                            );
                        Token {
                            position: SubstringPosition {
                                start_line: line_index,
                                end_line: line_index,
                                start_character:
                                    identifier_start_character_position,
                                end_character:
                                    identifier_end_character_position,
                            },
                            content: TokenContent::Identifier(
                                StringType::from(slice_string_by_positions(
                                    line,
                                    identifier_start_character_position,
                                    identifier_end_character_position,
                                )),
                            ),
                        }
                    }
                    _ => {
                        return Err(LexicalError::UnexpectedCharacter(
                            UnexpectedCharacter {
                                character,
                                position: SubstringPosition {
                                    start_line: line_index,
                                    end_line: line_index,
                                    start_character: character_position,
                                    end_character: to_line_end_position(line),
                                },
                                string: StringType::from(
                                    tail_string_from_position(
                                        line,
                                        character_position,
                                    ),
                                ),
                            },
                        ));
                    }
                };
                tokens.push(token);
                continue;
            }
        }
        Ok(Self(tokens))
    }
}

fn is_non_newline_whitespace(character: char) -> bool {
    character != NEWLINE && character.is_whitespace()
}

fn parse_numeric_literal<'a, StringType: From<&'a str>>(
    positioned_characters: &mut Peekable<PositionedCharacters<'a>>,
    line_index: usize,
    line: &'a str,
    start_character_position: CharacterPosition,
) -> Result<Token<StringType>, LexicalError<StringType>> {
    parse_digits(positioned_characters).ok_or_else(|| {
        LexicalError::NumericLiteralValueIncomplete(
            NumericLiteralValueIncomplete {
                kind: NumericLiteralValueKind::Integer,
                string: StringType::from(tail_string_from_position(
                    line,
                    start_character_position,
                )),
                position: SubstringPosition {
                    start_line: line_index,
                    start_character: start_character_position,
                    end_line: line_index,
                    end_character: to_line_end_position(line),
                },
            },
        )
    })?;
    let (mut value_end_character_position, mut character) = {
        positioned_characters.next().ok_or_else(|| {
            LexicalError::NumericLiteralValueIncomplete(
                NumericLiteralValueIncomplete {
                    kind: NumericLiteralValueKind::Integer,
                    string: StringType::from(tail_string_from_position(
                        line,
                        start_character_position,
                    )),
                    position: SubstringPosition {
                        start_line: line_index,
                        start_character: start_character_position,
                        end_line: line_index,
                        end_character: to_line_end_position(line),
                    },
                },
            )
        })?
    };
    let mut is_floating_point = false;
    if character == '.' {
        is_floating_point = true;
        parse_digits(positioned_characters).ok_or_else(|| {
            LexicalError::NumericLiteralTypeSuffixIncomplete(
                NumericLiteralTypeSuffixIncomplete {
                    position: SubstringPosition {
                        start_line: line_index,
                        start_character: start_character_position,
                        end_line: line_index,
                        end_character: to_line_end_position(line),
                    },
                    string: StringType::from(tail_string_from_position(
                        line,
                        start_character_position,
                    )),
                    value: StringType::from(tail_string_from_position(
                        line,
                        start_character_position,
                    )),
                    value_kind: NumericLiteralValueKind::FloatingPoint,
                },
            )
        })?;
        (value_end_character_position, character) =
            positioned_characters.next().ok_or_else(|| {
                LexicalError::NumericLiteralTypeSuffixIncomplete(
                    NumericLiteralTypeSuffixIncomplete {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: start_character_position,
                            end_line: line_index,
                            end_character: to_line_end_position(line),
                        },
                        string: StringType::from(tail_string_from_position(
                            line,
                            start_character_position,
                        )),
                        value: StringType::from(tail_string_from_position(
                            line,
                            start_character_position,
                        )),
                        value_kind: NumericLiteralValueKind::FloatingPoint,
                    },
                )
            })?;
    }
    if character == 'e' || character == 'E' {
        is_floating_point = true;
        parse_floating_point_numeric_literal_exponent(
            positioned_characters,
            line,
            line_index,
            start_character_position,
        )?;
        (value_end_character_position, character) =
            positioned_characters.next().ok_or_else(|| {
                LexicalError::NumericLiteralTypeSuffixIncomplete(
                    NumericLiteralTypeSuffixIncomplete {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: start_character_position,
                            end_line: line_index,
                            end_character: to_line_end_position(line),
                        },
                        string: StringType::from(tail_string_from_position(
                            line,
                            start_character_position,
                        )),
                        value: StringType::from(tail_string_from_position(
                            line,
                            start_character_position,
                        )),
                        value_kind: NumericLiteralValueKind::FloatingPoint,
                    },
                )
            })?;
    }
    let value = StringType::from(slice_string_by_positions(
        line,
        start_character_position,
        value_end_character_position,
    ));
    if character != TYPE_SUFFIX_SEPARATOR {
        return Err(
            LexicalError::NumericLiteralTypeSuffixUnexpectedCharacter(
                NumericLiteralTypeSuffixUnexpectedCharacter {
                    character,
                    expected: StringType::from(TYPE_SUFFIX_SEPARATOR_STRING),
                    position: SubstringPosition {
                        start_line: line_index,
                        start_character: start_character_position,
                        end_line: line_index,
                        end_character: to_line_end_position(line),
                    },
                    string: StringType::from(tail_string_from_position(
                        line,
                        start_character_position,
                    )),
                    value,
                    value_kind: if is_floating_point {
                        NumericLiteralValueKind::FloatingPoint
                    } else {
                        NumericLiteralValueKind::Integer
                    },
                },
            ),
        );
    }
    let type_suffix_start_character_position = CharacterPosition {
        byte: value_end_character_position.byte
            + TYPE_SUFFIX_SEPARATOR.byte_size(),
        utf_8: value_end_character_position.utf_8
            + TYPE_SUFFIX_SEPARATOR.utf_8_size(),
    };
    let (end_character_position, value) = parse_numeric_literal_type_suffix(
        positioned_characters,
        line,
        line_index,
        type_suffix_start_character_position,
        value,
        is_floating_point,
    )?;
    let string = StringType::from(slice_string_by_positions(
        line,
        start_character_position,
        end_character_position,
    ));
    let type_suffix = slice_string_by_positions(
        line,
        type_suffix_start_character_position,
        end_character_position,
    );
    let token_position = SubstringPosition {
        start_line: line_index,
        start_character: start_character_position,
        end_line: line_index,
        end_character: end_character_position,
    };
    let type_ = match type_suffix {
        F32_NAME => NumericLiteralType::F32,
        F64_NAME => NumericLiteralType::F64,
        I8_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict(
                        NumericLiteralValueTypeSuffixConflict {
                            string,
                            type_suffix: StringType::from(type_suffix),
                            value,
                            value_kind: NumericLiteralValueKind::FloatingPoint,
                            position: token_position,
                        },
                    ),
                );
            }
            NumericLiteralType::I8
        }
        I16_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict(
                        NumericLiteralValueTypeSuffixConflict {
                            string,
                            type_suffix: StringType::from(type_suffix),
                            value,
                            position: token_position,
                            value_kind: NumericLiteralValueKind::FloatingPoint,
                        },
                    ),
                );
            }
            NumericLiteralType::I16
        }
        I32_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict(
                        NumericLiteralValueTypeSuffixConflict {
                            string,
                            type_suffix: StringType::from(type_suffix),
                            value,
                            position: token_position,
                            value_kind: NumericLiteralValueKind::FloatingPoint,
                        },
                    ),
                );
            }
            NumericLiteralType::I32
        }
        I64_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict(
                        NumericLiteralValueTypeSuffixConflict {
                            string,
                            type_suffix: StringType::from(type_suffix),
                            value,
                            position: token_position,
                            value_kind: NumericLiteralValueKind::FloatingPoint,
                        },
                    ),
                );
            }
            NumericLiteralType::I64
        }
        ISIZE_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict(
                        NumericLiteralValueTypeSuffixConflict {
                            string,
                            type_suffix: StringType::from(type_suffix),
                            value,
                            position: token_position,
                            value_kind: NumericLiteralValueKind::FloatingPoint,
                        },
                    ),
                );
            }
            NumericLiteralType::ISize
        }
        U8_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict(
                        NumericLiteralValueTypeSuffixConflict {
                            string,
                            type_suffix: StringType::from(type_suffix),
                            value,
                            position: token_position,
                            value_kind: NumericLiteralValueKind::FloatingPoint,
                        },
                    ),
                );
            }
            NumericLiteralType::U8
        }
        U16_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict(
                        NumericLiteralValueTypeSuffixConflict {
                            string,
                            type_suffix: StringType::from(type_suffix),
                            value,
                            position: token_position,
                            value_kind: NumericLiteralValueKind::FloatingPoint,
                        },
                    ),
                );
            }
            NumericLiteralType::U16
        }
        U32_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict(
                        NumericLiteralValueTypeSuffixConflict {
                            string,
                            type_suffix: StringType::from(type_suffix),
                            value,
                            position: token_position,
                            value_kind: NumericLiteralValueKind::FloatingPoint,
                        },
                    ),
                );
            }
            NumericLiteralType::U32
        }
        U64_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict(
                        NumericLiteralValueTypeSuffixConflict {
                            position: token_position,
                            type_suffix: StringType::from(type_suffix),
                            string,
                            value,
                            value_kind: NumericLiteralValueKind::FloatingPoint,
                        },
                    ),
                );
            }
            NumericLiteralType::U64
        }
        USIZE_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict(
                        NumericLiteralValueTypeSuffixConflict {
                            position: token_position,
                            type_suffix: StringType::from(type_suffix),
                            string,
                            value,
                            value_kind: NumericLiteralValueKind::FloatingPoint,
                        },
                    ),
                );
            }
            NumericLiteralType::USize
        }
        _ => {
            return Err(LexicalError::NumericLiteralTypeSuffixUnknown(
                NumericLiteralTypeSuffixUnknown {
                    position: token_position,
                    string,
                    type_suffix: StringType::from(type_suffix),
                    value,
                    value_kind: if is_floating_point {
                        NumericLiteralValueKind::FloatingPoint
                    } else {
                        NumericLiteralValueKind::Integer
                    },
                },
            ));
        }
    };
    Ok(Token {
        position: token_position,
        content: TokenContent::NumericLiteral { value, type_ },
    })
}

fn parse_floating_point_literal_starting_with_dot<
    'a,
    StringType: From<&'a str>,
>(
    positioned_characters: &mut Peekable<PositionedCharacters<'a>>,
    line_index: usize,
    line: &'a str,
    start_character_position: CharacterPosition,
) -> Result<Token<StringType>, LexicalError<StringType>> {
    parse_digits(positioned_characters).ok_or_else(|| {
        LexicalError::NumericLiteralTypeSuffixIncomplete(
            NumericLiteralTypeSuffixIncomplete {
                string: StringType::from(tail_string_from_position(
                    line,
                    start_character_position,
                )),
                position: SubstringPosition {
                    start_line: line_index,
                    start_character: start_character_position,
                    end_line: line_index,
                    end_character: to_line_end_position(line),
                },
                value: StringType::from(tail_string_from_position(
                    line,
                    start_character_position,
                )),
                value_kind: NumericLiteralValueKind::FloatingPoint,
            },
        )
    })?;
    let (mut value_end_character_position, mut character) =
        positioned_characters.next().ok_or_else(|| {
            LexicalError::NumericLiteralTypeSuffixIncomplete(
                NumericLiteralTypeSuffixIncomplete {
                    string: StringType::from(tail_string_from_position(
                        line,
                        start_character_position,
                    )),
                    position: SubstringPosition {
                        start_line: line_index,
                        start_character: start_character_position,
                        end_line: line_index,
                        end_character: to_line_end_position(line),
                    },
                    value: StringType::from(tail_string_from_position(
                        line,
                        start_character_position,
                    )),
                    value_kind: NumericLiteralValueKind::FloatingPoint,
                },
            )
        })?;
    if character == 'e' || character == 'E' {
        parse_floating_point_numeric_literal_exponent(
            positioned_characters,
            line,
            line_index,
            start_character_position,
        )?;
        (value_end_character_position, character) =
            positioned_characters.next().ok_or_else(|| {
                LexicalError::NumericLiteralTypeSuffixIncomplete(
                    NumericLiteralTypeSuffixIncomplete {
                        string: StringType::from(tail_string_from_position(
                            line,
                            start_character_position,
                        )),
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: start_character_position,
                            end_line: line_index,
                            end_character: to_line_end_position(line),
                        },
                        value: StringType::from(tail_string_from_position(
                            line,
                            start_character_position,
                        )),
                        value_kind: NumericLiteralValueKind::FloatingPoint,
                    },
                )
            })?;
    }
    let value = StringType::from(slice_string_by_positions(
        line,
        start_character_position,
        value_end_character_position,
    ));
    if character != TYPE_SUFFIX_SEPARATOR {
        return Err(
            LexicalError::NumericLiteralTypeSuffixUnexpectedCharacter(
                NumericLiteralTypeSuffixUnexpectedCharacter {
                    character,
                    expected: StringType::from(TYPE_SUFFIX_SEPARATOR_STRING),
                    position: SubstringPosition {
                        start_line: line_index,
                        start_character: start_character_position,
                        end_line: line_index,
                        end_character: to_line_end_position(line),
                    },
                    string: StringType::from(tail_string_from_position(
                        line,
                        start_character_position,
                    )),
                    value,
                    value_kind: NumericLiteralValueKind::FloatingPoint,
                },
            ),
        );
    }
    let type_suffix_start_character_position = CharacterPosition {
        byte: value_end_character_position.byte
            + TYPE_SUFFIX_SEPARATOR.byte_size(),
        utf_8: value_end_character_position.utf_8
            + TYPE_SUFFIX_SEPARATOR.utf_8_size(),
    };
    let (end_character_position, value) = parse_numeric_literal_type_suffix(
        positioned_characters,
        line,
        line_index,
        type_suffix_start_character_position,
        value,
        true,
    )?;
    let token_position = SubstringPosition {
        start_line: line_index,
        start_character: start_character_position,
        end_line: line_index,
        end_character: end_character_position,
    };
    let type_suffix = slice_string_by_positions(
        line,
        type_suffix_start_character_position,
        end_character_position,
    );
    let type_ = match type_suffix {
        F32_NAME => NumericLiteralType::F32,
        F64_NAME => NumericLiteralType::F64,
        I8_NAME | I16_NAME | I32_NAME | I64_NAME | ISIZE_NAME | U8_NAME
        | U16_NAME | U32_NAME | U64_NAME | USIZE_NAME => {
            return Err(LexicalError::NumericLiteralValueTypeSuffixConflict(
                NumericLiteralValueTypeSuffixConflict {
                    position: token_position,
                    type_suffix: StringType::from(type_suffix),
                    string: StringType::from(slice_string_by_positions(
                        line,
                        start_character_position,
                        end_character_position,
                    )),
                    value,
                    value_kind: NumericLiteralValueKind::FloatingPoint,
                },
            ));
        }
        _ => {
            return Err(LexicalError::NumericLiteralTypeSuffixUnknown(
                NumericLiteralTypeSuffixUnknown {
                    position: token_position,
                    type_suffix: StringType::from(type_suffix),
                    string: StringType::from(slice_string_by_positions(
                        line,
                        start_character_position,
                        end_character_position,
                    )),
                    value,
                    value_kind: NumericLiteralValueKind::FloatingPoint,
                },
            ));
        }
    };
    Ok(Token {
        position: token_position,
        content: TokenContent::NumericLiteral { value, type_ },
    })
}

struct PositionedCharacters<'a> {
    indexed_characters: CharIndices<'a>,
    utf8_index: Utf8Index,
}

impl<'a> From<CharIndices<'a>> for PositionedCharacters<'a> {
    fn from(iter: CharIndices<'a>) -> Self {
        Self {
            indexed_characters: iter,
            utf8_index: Default::default(),
        }
    }
}

impl<'a> Iterator for PositionedCharacters<'a> {
    type Item = (CharacterPosition, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.indexed_characters.next().map(
            |(character_byte_index, character)| {
                let position = CharacterPosition {
                    byte: ByteIndex::from(character_byte_index),
                    utf_8: self.utf8_index,
                };
                self.utf8_index += character.utf_8_size();
                (position, character)
            },
        )
    }
}

fn is_non_starting_identifier_character(candidate: char) -> bool {
    NON_STARTING_IDENTIFIER_CHARACTERS.contains(candidate)
}

fn is_numeric_character(character: char) -> bool {
    NUMERIC_CHARACTERS.contains(character)
}

fn is_starting_identifier_character(character: char) -> bool {
    STARTING_IDENTIFIER_CHARACTERS.contains(character)
}

fn parse_numeric_literal_type_suffix<'a, StringType: From<&'a str>>(
    positioned_characters: &mut Peekable<PositionedCharacters<'a>>,
    line: &'a str,
    line_index: usize,
    type_suffix_start_character_position: CharacterPosition,
    value: StringType,
    is_floating_point: bool,
) -> Result<(CharacterPosition, StringType), LexicalError<StringType>> {
    match parse_identifier(
        positioned_characters,
        line,
        line_index,
        type_suffix_start_character_position,
        TYPE_SUFFIX_SEPARATOR,
    ) {
        Ok(position) => Ok((position, value)),
        Err(error) => Err(match error {
            LexicalError::IdentifierIncomplete(IdentifierIncomplete {
                position,
                string,
            }) => LexicalError::NumericLiteralTypeSuffixIncomplete(
                NumericLiteralTypeSuffixIncomplete {
                    position,
                    string,
                    value,
                    value_kind: if is_floating_point {
                        NumericLiteralValueKind::FloatingPoint
                    } else {
                        NumericLiteralValueKind::Integer
                    },
                },
            ),
            LexicalError::IdentifierUnexpectedCharacter(
                IdentifierUnexpectedCharacter {
                    character,
                    expected,
                    position,
                    string,
                },
            ) => LexicalError::NumericLiteralTypeSuffixUnexpectedCharacter(
                NumericLiteralTypeSuffixUnexpectedCharacter {
                    character,
                    expected,
                    position,
                    string,
                    value,
                    value_kind: if is_floating_point {
                        NumericLiteralValueKind::FloatingPoint
                    } else {
                        NumericLiteralValueKind::Integer
                    },
                },
            ),
            _ => unreachable!(),
        }),
    }
}

fn parse_digits(
    positioned_characters: &mut Peekable<PositionedCharacters<'_>>,
) -> Option<()> {
    loop {
        if let Some((_, candidate)) = positioned_characters.peek() {
            if !is_numeric_character(*candidate) {
                break;
            }
            let _ = positioned_characters.next();
            continue;
        } else {
            return None;
        }
    }
    Some(())
}

fn parse_floating_point_numeric_literal_exponent<
    'a,
    StringType: From<&'a str>,
>(
    positioned_characters: &mut Peekable<PositionedCharacters<'a>>,
    line: &'a str,
    line_index: usize,
    start_character_position: CharacterPosition,
) -> Result<(), LexicalError<StringType>> {
    if let Some((_, candidate)) = positioned_characters.peek() {
        if *candidate == '+' || *candidate == '-' {
            let _ = positioned_characters.next();
        }
    }
    if let Some((_, candidate)) = positioned_characters.next() {
        if !is_numeric_character(candidate) {
            let unexpected_character_end_position = positioned_characters
                .next()
                .map(|(position, _)| position)
                .unwrap_or_else(|| to_line_end_position(line));
            return Err(LexicalError::NumericLiteralValueUnexpectedCharacter(
                NumericLiteralValueUnexpectedCharacter {
                    string: StringType::from(slice_string_by_positions(
                        line,
                        start_character_position,
                        unexpected_character_end_position,
                    )),
                    expected: StringType::from(NUMERIC_CHARACTERS),
                    position: SubstringPosition {
                        start_line: line_index,
                        end_line: line_index,
                        start_character: start_character_position,
                        end_character: unexpected_character_end_position,
                    },
                    character: candidate,
                    kind: NumericLiteralValueKind::FloatingPoint,
                },
            ));
        }
    } else {
        return Err(LexicalError::NumericLiteralValueIncomplete(
            NumericLiteralValueIncomplete {
                string: StringType::from(tail_string_from_position(
                    line,
                    start_character_position,
                )),
                position: SubstringPosition {
                    start_line: line_index,
                    end_line: line_index,
                    start_character: start_character_position,
                    end_character: to_line_end_position(line),
                },
                kind: NumericLiteralValueKind::FloatingPoint,
            },
        ));
    };
    loop {
        if let Some((_, candidate)) = positioned_characters.peek() {
            if is_numeric_character(*candidate) {
                let _ = positioned_characters.next();
            } else {
                break;
            }
        } else {
            return Err(LexicalError::NumericLiteralTypeSuffixIncomplete(
                NumericLiteralTypeSuffixIncomplete {
                    string: StringType::from(tail_string_from_position(
                        line,
                        start_character_position,
                    )),
                    position: SubstringPosition {
                        start_line: line_index,
                        end_line: line_index,
                        start_character: start_character_position,
                        end_character: to_line_end_position(line),
                    },
                    value: StringType::from(tail_string_from_position(
                        line,
                        start_character_position,
                    )),
                    value_kind: NumericLiteralValueKind::FloatingPoint,
                },
            ));
        }
    }
    Ok(())
}

fn parse_identifier<'a, StringType: From<&'a str>>(
    positioned_characters: &mut Peekable<PositionedCharacters<'a>>,
    line: &'a str,
    line_index: usize,
    start_character_position: CharacterPosition,
    start_character: char,
) -> Result<CharacterPosition, LexicalError<StringType>> {
    if let Some((_, candidate)) = positioned_characters.next() {
        if !STARTING_IDENTIFIER_CHARACTERS.contains(candidate) {
            return Err(LexicalError::IdentifierUnexpectedCharacter(
                IdentifierUnexpectedCharacter {
                    character: candidate,
                    expected: StringType::from(STARTING_IDENTIFIER_CHARACTERS),
                    position: SubstringPosition {
                        start_line: line_index,
                        end_line: line_index,
                        start_character: start_character_position,
                        end_character: to_line_end_position(line),
                    },
                    string: StringType::from(tail_string_from_position(
                        line,
                        start_character_position,
                    )),
                },
            ));
        }
    } else {
        return Err(LexicalError::IdentifierIncomplete(
            IdentifierIncomplete {
                string: StringType::from(tail_string_from_position(
                    line,
                    start_character_position,
                )),
                position: SubstringPosition {
                    start_line: line_index,
                    end_line: line_index,
                    start_character: start_character_position,
                    end_character: to_line_end_position(line),
                },
            },
        ));
    };
    Ok(parse_non_starting_identifier_characters(
        positioned_characters,
        start_character_position,
        start_character,
    ))
}

fn parse_non_starting_identifier_characters(
    positioned_characters: &mut Peekable<PositionedCharacters<'_>>,
    identifier_start_character_position: CharacterPosition,
    identifier_start_character: char,
) -> CharacterPosition {
    let (mut character_position, mut character) = (
        identifier_start_character_position,
        identifier_start_character,
    );
    loop {
        if let Some((candidate_position, candidate)) =
            positioned_characters.peek()
        {
            if !is_non_starting_identifier_character(*candidate) {
                return *candidate_position;
            }
            (character_position, character) =
                unsafe { positioned_characters.next().unwrap_unchecked() };
        } else {
            return CharacterPosition {
                byte: character_position.byte + character.byte_size(),
                utf_8: character_position.utf_8 + character.utf_8_size(),
            };
        }
    }
}

fn behead_string_until_byte_index(line: &str, byte_index: ByteIndex) -> &str {
    &line[..byte_index.into()]
}

fn slice_string_by_byte_indices(
    line: &str,
    start_byte_index: ByteIndex,
    end_byte_index: ByteIndex,
) -> &str {
    &line[start_byte_index.into()..end_byte_index.into()]
}

fn slice_string_by_positions(
    line: &str,
    start_position: CharacterPosition,
    end_position: CharacterPosition,
) -> &str {
    slice_string_by_byte_indices(line, start_position.byte, end_position.byte)
}

fn tail_string_from_position(line: &str, position: CharacterPosition) -> &str {
    tail_string_from_byte_index(line, position.byte)
}

fn tail_string_from_byte_index(line: &str, byte_index: ByteIndex) -> &str {
    &line[byte_index.into()..]
}

fn to_line_end_position(line: &str) -> CharacterPosition {
    CharacterPosition {
        byte: line.byte_size(),
        utf_8: line.utf_8_size(),
    }
}
