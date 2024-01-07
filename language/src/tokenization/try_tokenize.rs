use std::iter::Peekable;
use std::str::CharIndices;

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
use super::numeric_literal_type::NumericLiteralType;
use super::positioned_token::PositionedToken;
use super::token::Token;
use super::types::LexicalError;
use super::{CharacterPosition, NumericLiteralValueKind, SubstringPosition};

pub trait TryTokenize<'a> {
    fn try_tokenize(
        self,
    ) -> Result<Vec<PositionedToken<'a>>, LexicalError<'a>>;
}

impl<'a> TryTokenize<'a> for &'a str {
    fn try_tokenize(
        self,
    ) -> Result<Vec<PositionedToken<'a>>, LexicalError<'a>> {
        Tokens::try_from(self).map(move |tokens| tokens.0)
    }
}

struct Tokens<'a>(Vec<PositionedToken<'a>>);

impl<'a> TryFrom<&'a str> for Tokens<'a> {
    type Error = LexicalError<'a>;

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
                                tokens.push(PositionedToken {
                                    position: SubstringPosition {
                                        start_line: line_index,
                                        end_line: line_index,
                                        start_character: whitespace_start_character_position,
                                        end_character: candidate_position,
                                    },
                                    token: Token::Whitespace(
                                        &line[whitespace_start_character_position.byte
                                            ..candidate_position.byte],
                                    ),
                                });
                                (character_position, character) =
                                    (candidate_position, candidate);
                                break;
                            }
                        } else {
                            tokens.push(PositionedToken {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    end_line: line_index,
                                    start_character: whitespace_start_character_position,
                                    end_character: character_position,
                                },
                                token: Token::Whitespace(
                                    &line[whitespace_start_character_position.byte..],
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
                            &line[character_position.byte
                                ..character_position.byte
                                    + COMMENT_LINE_START.len()],
                            COMMENT_LINE_START
                        );
                        tokens.push(PositionedToken {
                            position: SubstringPosition {
                                start_line: line_index,
                                start_character: character_position,
                                end_line: line_index,
                                end_character: to_line_end_position(line),
                            },
                            token: Token::CommentLine(
                                &line[character_position.byte..],
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
                                + COMMENT_BLOCK_START.len();
                        if let Some(
                            candidate_comment_block_end_first_byte_index,
                        ) = line[comment_block_first_non_start_byte_index..]
                            .find(COMMENT_BLOCK_END)
                        {
                            let comment_block_end_byte_index =
                                comment_block_first_non_start_byte_index
                                    + candidate_comment_block_end_first_byte_index
                                    + COMMENT_BLOCK_END.len();
                            let non_start_comment_block_characters_count =
                                line[comment_block_first_non_start_byte_index
                                    ..comment_block_end_byte_index]
                                    .chars()
                                    .count();
                            for _ in 0usize
                                ..non_start_comment_block_characters_count
                            {
                                let _ = positioned_characters.next();
                            }
                            let comment_block_column_end = positioned_characters
                                .peek()
                                .map(|(position, _)| *position)
                                .unwrap_or_else(|| CharacterPosition {
                                    byte: line.len(),
                                    utf_8: comment_block_start_character_position.utf_8
                                        + COMMENT_BLOCK_START.chars().count()
                                        + non_start_comment_block_characters_count,
                                });
                            tokens.push(PositionedToken {
                                position: SubstringPosition {
                                    start_line: comment_block_line_start,
                                    start_character:
                                        comment_block_start_character_position,
                                    end_line: comment_block_line_start,
                                    end_character: comment_block_column_end,
                                },
                                token: Token::CommentBlock(vec![
                                    &line
                                        [comment_block_start_character_position
                                            .byte
                                            ..comment_block_column_end.byte],
                                ]),
                            });
                            continue;
                        } else {
                            let mut comment_block_lines = vec![
                                &line
                                    [comment_block_start_character_position
                                        .byte..],
                            ];
                            loop {
                                if let Some((next_line_index, next_line)) =
                                    enumerated_lines.next()
                                {
                                    line = next_line;
                                    line_index = next_line_index;
                                    if let Some(
                                        comment_block_column_byte_offset,
                                    ) = line.find(COMMENT_BLOCK_END)
                                    {
                                        let comment_block_column_end_byte_index =
                                            comment_block_column_byte_offset
                                                + COMMENT_BLOCK_END.len();
                                        comment_block_lines
                                            .push(&line[..comment_block_column_end_byte_index]);
                                        positioned_characters =
                                            PositionedCharacters::from(
                                                line.char_indices(),
                                            )
                                            .peekable();
                                        for _ in 0usize
                                            ..(line[..comment_block_column_end_byte_index]
                                                .chars()
                                                .count()
                                                - 1usize)
                                        {
                                            let _ = positioned_characters.next();
                                        }
                                        let (comment_block_column_end, _) = unsafe {
                                            positioned_characters
                                                .next()
                                                .unwrap_unchecked()
                                        };
                                        tokens.push(PositionedToken {
                                            position: SubstringPosition {
                                                start_line: comment_block_line_start,
                                                start_character:
                                                    comment_block_start_character_position,
                                                end_line: line_index,
                                                end_character: comment_block_column_end,
                                            },
                                            token: Token::CommentBlock(comment_block_lines),
                                        });
                                        continue 'characters;
                                    } else {
                                        comment_block_lines.push(line);
                                    }
                                } else {
                                    return Err(LexicalError::CommentBlockIncomplete {
                                        strings: comment_block_lines,
                                        position: SubstringPosition {
                                            start_line: comment_block_line_start,
                                            start_character: comment_block_start_character_position,
                                            end_line: line_index,
                                            end_character: to_line_end_position(line),
                                        },
                                    });
                                }
                            }
                        }
                    } else {
                        tokens.push(PositionedToken {
                            position: SubstringPosition {
                                start_line: line_index,
                                start_character: character_position,
                                end_line: line_index,
                                end_character: CharacterPosition {
                                    byte: character_position.byte
                                        + SLASH.len_utf8(),
                                    utf_8: character_position.utf_8 + 1usize,
                                },
                            },
                            token: Token::Slash,
                        });
                        continue;
                    }
                }
                let token = match character {
                    ASSIGNMENT => {
                        if let Some((_, '=')) = positioned_characters.peek() {
                            let _ = positioned_characters.next();
                            PositionedToken {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + EQUAL_TO.len(),
                                        utf_8: character_position.utf_8
                                            + EQUAL_TO.chars().count(),
                                    },
                                },
                                token: Token::EqualTo,
                            }
                        } else {
                            PositionedToken {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + ASSIGNMENT.len_utf8(),
                                        utf_8: character_position.utf_8
                                            + 1usize,
                                    },
                                },
                                token: Token::Assignment,
                            }
                        }
                    }
                    ASTERISK => PositionedToken {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + ASTERISK.len_utf8(),
                                utf_8: character_position.utf_8 + 1usize,
                            },
                        },
                        token: Token::Asterisk,
                    },
                    CLOSE_BRACE => PositionedToken {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + CLOSE_BRACE.len_utf8(),
                                utf_8: character_position.utf_8 + 1usize,
                            },
                        },
                        token: Token::CloseBrace,
                    },
                    CLOSE_PARENTHESES => PositionedToken {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + CLOSE_PARENTHESES.len_utf8(),
                                utf_8: character_position.utf_8 + 1usize,
                            },
                        },
                        token: Token::CloseParenthesis,
                    },
                    COLON => PositionedToken {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + COLON.len_utf8(),
                                utf_8: character_position.utf_8 + 1usize,
                            },
                        },
                        token: Token::Colon,
                    },
                    COMMA => PositionedToken {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + COMMA.len_utf8(),
                                utf_8: character_position.utf_8 + 1usize,
                            },
                        },
                        token: Token::Comma,
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
                            PositionedToken {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    end_line: line_index,
                                    start_character: character_position,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + DOT.len_utf8(),
                                        utf_8: character_position.utf_8
                                            + 1usize,
                                    },
                                },
                                token: Token::Dot,
                            }
                        }
                    }
                    GREATER_THAN => {
                        if let Some((_, '=')) = positioned_characters.peek() {
                            let _ = positioned_characters.next();
                            PositionedToken {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + GREATER_THAN_OR_EQUAL_TO.len(),
                                        utf_8: character_position.utf_8
                                            + GREATER_THAN_OR_EQUAL_TO
                                                .chars()
                                                .count(),
                                    },
                                },
                                token: Token::GreaterThanOrEqualTo,
                            }
                        } else {
                            PositionedToken {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + GREATER_THAN.len_utf8(),
                                        utf_8: character_position.utf_8
                                            + 1usize,
                                    },
                                },
                                token: Token::GreaterThan,
                            }
                        }
                    }
                    LOWER_THAN => {
                        if let Some((_, '=')) = positioned_characters.peek() {
                            let _ = positioned_characters.next();
                            PositionedToken {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + LOWER_THAN_OR_EQUAL_TO.len(),
                                        utf_8: character_position.utf_8
                                            + LOWER_THAN_OR_EQUAL_TO
                                                .chars()
                                                .count(),
                                    },
                                },
                                token: Token::LowerThanOrEqualTo,
                            }
                        } else {
                            PositionedToken {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + LOWER_THAN.len_utf8(),
                                        utf_8: character_position.utf_8
                                            + 1usize,
                                    },
                                },
                                token: Token::LowerThan,
                            }
                        }
                    }
                    MINUS => {
                        if matches!(
                            positioned_characters.peek(),
                            Some((_, '>'))
                        ) {
                            let _ = positioned_characters.next();
                            PositionedToken {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + ARROW.len(),
                                        utf_8: character_position.utf_8
                                            + 1usize,
                                    },
                                },
                                token: Token::Arrow,
                            }
                        } else {
                            PositionedToken {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + MINUS.len_utf8(),
                                        utf_8: character_position.utf_8
                                            + 1usize,
                                    },
                                },
                                token: Token::Minus,
                            }
                        }
                    }
                    NEWLINE => PositionedToken {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + NEWLINE.len_utf8(),
                                utf_8: character_position.utf_8 + 1usize,
                            },
                        },
                        token: Token::Newline,
                    },
                    OPEN_BRACE => PositionedToken {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + OPEN_BRACE.len_utf8(),
                                utf_8: character_position.utf_8 + 1usize,
                            },
                        },
                        token: Token::OpenBrace,
                    },
                    OPEN_PARENTHESES => PositionedToken {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + OPEN_PARENTHESES.len_utf8(),
                                utf_8: character_position.utf_8 + 1usize,
                            },
                        },
                        token: Token::OpenParenthesis,
                    },
                    PLUS => PositionedToken {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + PLUS.len_utf8(),
                                utf_8: character_position.utf_8 + 1usize,
                            },
                        },
                        token: Token::Plus,
                    },
                    SEMICOLON => PositionedToken {
                        position: SubstringPosition {
                            start_line: line_index,
                            start_character: character_position,
                            end_line: line_index,
                            end_character: CharacterPosition {
                                byte: character_position.byte
                                    + SEMICOLON.len_utf8(),
                                utf_8: character_position.utf_8 + 1usize,
                            },
                        },
                        token: Token::Semicolon,
                    },
                    '!' => {
                        if let Some((_, '=')) = positioned_characters.peek() {
                            let _ = positioned_characters.next();
                            PositionedToken {
                                position: SubstringPosition {
                                    start_line: line_index,
                                    start_character: character_position,
                                    end_line: line_index,
                                    end_character: CharacterPosition {
                                        byte: character_position.byte
                                            + NOT_EQUAL_TO.len(),
                                        utf_8: character_position.utf_8
                                            + NOT_EQUAL_TO.chars().count(),
                                    },
                                },
                                token: Token::NotEqualTo,
                            }
                        } else {
                            return Err(LexicalError::UnexpectedCharacter {
                                character,
                                position: SubstringPosition {
                                    start_line: line_index,
                                    end_line: line_index,
                                    start_character: character_position,
                                    end_character: to_line_end_position(line),
                                },
                                string: &line[character_position.byte..],
                            });
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
                        let identifier_start_character_position =
                            character_position;
                        let identifier_end_character_position =
                            parse_non_starting_identifier_characters(
                                &mut positioned_characters,
                                identifier_start_character_position,
                            );
                        PositionedToken {
                            position: SubstringPosition {
                                start_line: line_index,
                                end_line: line_index,
                                start_character:
                                    identifier_start_character_position,
                                end_character:
                                    identifier_end_character_position,
                            },
                            token: Token::Identifier(
                                &line[identifier_start_character_position.byte
                                    ..identifier_end_character_position.byte],
                            ),
                        }
                    }
                    _ => {
                        return Err(LexicalError::UnexpectedCharacter {
                            character,
                            position: SubstringPosition {
                                start_line: line_index,
                                end_line: line_index,
                                start_character: character_position,
                                end_character: to_line_end_position(line),
                            },
                            string: &line[character_position.byte..],
                        });
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

fn parse_numeric_literal<'a>(
    positioned_characters: &mut Peekable<PositionedCharacters<'a>>,
    line_index: usize,
    line: &'a str,
    start_character_position: CharacterPosition,
) -> Result<PositionedToken<'a>, LexicalError<'a>> {
    parse_digits(positioned_characters).ok_or_else(|| {
        LexicalError::NumericLiteralValueIncomplete {
            kind: NumericLiteralValueKind::Integer,
            string: &line[start_character_position.byte..],
            position: SubstringPosition {
                start_line: line_index,
                start_character: start_character_position,
                end_line: line_index,
                end_character: to_line_end_position(line),
            },
        }
    })?;
    let (mut value_end_character_position, mut character) = {
        positioned_characters.next().ok_or_else(|| {
            LexicalError::NumericLiteralValueIncomplete {
                kind: NumericLiteralValueKind::Integer,
                string: &line[start_character_position.byte..],
                position: SubstringPosition {
                    start_line: line_index,
                    start_character: start_character_position,
                    end_line: line_index,
                    end_character: to_line_end_position(line),
                },
            }
        })?
    };
    let mut is_floating_point = false;
    if character == '.' {
        is_floating_point = true;
        parse_digits(positioned_characters).ok_or_else(|| {
            LexicalError::NumericLiteralTypeSuffixIncomplete {
                position: SubstringPosition {
                    start_line: line_index,
                    start_character: start_character_position,
                    end_line: line_index,
                    end_character: to_line_end_position(line),
                },
                string: &line[start_character_position.byte..],
                value: &line[start_character_position.byte..],
                value_kind: NumericLiteralValueKind::FloatingPoint,
            }
        })?;
        (value_end_character_position, character) =
            positioned_characters.next().ok_or_else(|| {
                LexicalError::NumericLiteralTypeSuffixIncomplete {
                    position: SubstringPosition {
                        start_line: line_index,
                        start_character: start_character_position,
                        end_line: line_index,
                        end_character: to_line_end_position(line),
                    },
                    string: &line[start_character_position.byte..],
                    value: &line[start_character_position.byte..],
                    value_kind: NumericLiteralValueKind::FloatingPoint,
                }
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
                LexicalError::NumericLiteralTypeSuffixIncomplete {
                    position: SubstringPosition {
                        start_line: line_index,
                        start_character: start_character_position,
                        end_line: line_index,
                        end_character: to_line_end_position(line),
                    },
                    string: &line[start_character_position.byte..],
                    value: &line[start_character_position.byte..],
                    value_kind: NumericLiteralValueKind::FloatingPoint,
                }
            })?;
    }
    let value = &line
        [start_character_position.byte..value_end_character_position.byte];
    if character != TYPE_SUFFIX_SEPARATOR {
        return Err(
            LexicalError::NumericLiteralTypeSuffixUnexpectedCharacter {
                character,
                expected: TYPE_SUFFIX_SEPARATOR_STRING,
                position: SubstringPosition {
                    start_line: line_index,
                    start_character: start_character_position,
                    end_line: line_index,
                    end_character: to_line_end_position(line),
                },
                string: &line[start_character_position.byte..],
                value,
                value_kind: if is_floating_point {
                    NumericLiteralValueKind::FloatingPoint
                } else {
                    NumericLiteralValueKind::Integer
                },
            },
        );
    }
    let type_suffix_start_character_position = CharacterPosition {
        byte: value_end_character_position.byte
            + TYPE_SUFFIX_SEPARATOR.len_utf8(),
        utf_8: value_end_character_position.utf_8 + 1usize,
    };
    let end_character_position = parse_numeric_literal_type_suffix(
        positioned_characters,
        line,
        line_index,
        type_suffix_start_character_position,
        value,
        is_floating_point,
    )?;
    let string =
        &line[start_character_position.byte..end_character_position.byte];
    let type_suffix = &line[type_suffix_start_character_position.byte
        ..end_character_position.byte];
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
                    LexicalError::NumericLiteralValueTypeSuffixConflict {
                        string,
                        type_suffix,
                        value,
                        value_kind: NumericLiteralValueKind::FloatingPoint,
                        position: token_position,
                    },
                );
            }
            NumericLiteralType::I8
        }
        I16_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict {
                        string,
                        type_suffix,
                        value,
                        position: token_position,
                        value_kind: NumericLiteralValueKind::FloatingPoint,
                    },
                );
            }
            NumericLiteralType::I16
        }
        I32_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict {
                        string,
                        type_suffix,
                        value,
                        position: token_position,
                        value_kind: NumericLiteralValueKind::FloatingPoint,
                    },
                );
            }
            NumericLiteralType::I32
        }
        I64_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict {
                        string,
                        type_suffix,
                        value,
                        position: token_position,
                        value_kind: NumericLiteralValueKind::FloatingPoint,
                    },
                );
            }
            NumericLiteralType::I64
        }
        ISIZE_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict {
                        string,
                        type_suffix,
                        value,
                        position: token_position,
                        value_kind: NumericLiteralValueKind::FloatingPoint,
                    },
                );
            }
            NumericLiteralType::ISize
        }
        U8_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict {
                        string,
                        type_suffix,
                        value,
                        position: token_position,
                        value_kind: NumericLiteralValueKind::FloatingPoint,
                    },
                );
            }
            NumericLiteralType::U8
        }
        U16_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict {
                        string,
                        type_suffix,
                        value,
                        position: token_position,
                        value_kind: NumericLiteralValueKind::FloatingPoint,
                    },
                );
            }
            NumericLiteralType::U16
        }
        U32_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict {
                        string,
                        type_suffix,
                        value,
                        position: token_position,
                        value_kind: NumericLiteralValueKind::FloatingPoint,
                    },
                );
            }
            NumericLiteralType::U32
        }
        U64_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict {
                        position: token_position,
                        type_suffix,
                        string,
                        value,
                        value_kind: NumericLiteralValueKind::FloatingPoint,
                    },
                );
            }
            NumericLiteralType::U64
        }
        USIZE_NAME => {
            if is_floating_point {
                return Err(
                    LexicalError::NumericLiteralValueTypeSuffixConflict {
                        position: token_position,
                        type_suffix,
                        string,
                        value,
                        value_kind: NumericLiteralValueKind::FloatingPoint,
                    },
                );
            }
            NumericLiteralType::USize
        }
        _ => {
            return Err(LexicalError::NumericLiteralTypeSuffixUnknown {
                position: token_position,
                string,
                type_suffix,
                value,
                value_kind: if is_floating_point {
                    NumericLiteralValueKind::FloatingPoint
                } else {
                    NumericLiteralValueKind::Integer
                },
            });
        }
    };
    Ok(PositionedToken {
        position: token_position,
        token: Token::NumericLiteral { value, type_ },
    })
}

fn parse_floating_point_literal_starting_with_dot<'a>(
    positioned_characters: &mut Peekable<PositionedCharacters<'a>>,
    line_index: usize,
    line: &'a str,
    start_character_position: CharacterPosition,
) -> Result<PositionedToken<'a>, LexicalError<'a>> {
    parse_digits(positioned_characters).ok_or_else(|| {
        LexicalError::NumericLiteralTypeSuffixIncomplete {
            string: &line[start_character_position.byte..],
            position: SubstringPosition {
                start_line: line_index,
                start_character: start_character_position,
                end_line: line_index,
                end_character: to_line_end_position(line),
            },
            value: &line[start_character_position.byte..],
            value_kind: NumericLiteralValueKind::FloatingPoint,
        }
    })?;
    let (mut value_end_character_position, mut character) =
        positioned_characters.next().ok_or_else(|| {
            LexicalError::NumericLiteralTypeSuffixIncomplete {
                string: &line[start_character_position.byte..],
                position: SubstringPosition {
                    start_line: line_index,
                    start_character: start_character_position,
                    end_line: line_index,
                    end_character: to_line_end_position(line),
                },
                value: &line[start_character_position.byte..],
                value_kind: NumericLiteralValueKind::FloatingPoint,
            }
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
                LexicalError::NumericLiteralTypeSuffixIncomplete {
                    string: &line[start_character_position.byte..],
                    position: SubstringPosition {
                        start_line: line_index,
                        start_character: start_character_position,
                        end_line: line_index,
                        end_character: to_line_end_position(line),
                    },
                    value: &line[start_character_position.byte..],
                    value_kind: NumericLiteralValueKind::FloatingPoint,
                }
            })?;
    }
    let value = &line
        [start_character_position.byte..value_end_character_position.byte];
    if character != TYPE_SUFFIX_SEPARATOR {
        return Err(
            LexicalError::NumericLiteralTypeSuffixUnexpectedCharacter {
                character,
                expected: TYPE_SUFFIX_SEPARATOR_STRING,
                position: SubstringPosition {
                    start_line: line_index,
                    start_character: start_character_position,
                    end_line: line_index,
                    end_character: to_line_end_position(line),
                },
                string: &line[start_character_position.byte..],
                value,
                value_kind: NumericLiteralValueKind::FloatingPoint,
            },
        );
    }
    let type_suffix_start_character_position = CharacterPosition {
        byte: value_end_character_position.byte
            + TYPE_SUFFIX_SEPARATOR.len_utf8(),
        utf_8: value_end_character_position.utf_8 + 1usize,
    };
    let end_character_position = parse_numeric_literal_type_suffix(
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
    let token_string =
        &line[start_character_position.byte..end_character_position.byte];
    let type_suffix = &line[type_suffix_start_character_position.byte
        ..end_character_position.byte];
    let type_ = match type_suffix {
        F32_NAME => NumericLiteralType::F32,
        F64_NAME => NumericLiteralType::F64,
        I8_NAME | I16_NAME | I32_NAME | I64_NAME | ISIZE_NAME | U8_NAME
        | U16_NAME | U32_NAME | U64_NAME | USIZE_NAME => {
            return Err(LexicalError::NumericLiteralValueTypeSuffixConflict {
                position: token_position,
                type_suffix,
                string: token_string,
                value,
                value_kind: NumericLiteralValueKind::FloatingPoint,
            });
        }
        _ => {
            return Err(LexicalError::NumericLiteralTypeSuffixUnknown {
                position: token_position,
                type_suffix,
                string: token_string,
                value,
                value_kind: NumericLiteralValueKind::FloatingPoint,
            });
        }
    };
    Ok(PositionedToken {
        position: token_position,
        token: Token::NumericLiteral { value, type_ },
    })
}

struct PositionedCharacters<'a> {
    indexed_characters: CharIndices<'a>,
    byte_index: usize,
}

impl<'a> From<CharIndices<'a>> for PositionedCharacters<'a> {
    fn from(iter: CharIndices<'a>) -> Self {
        Self {
            indexed_characters: iter,
            byte_index: 0,
        }
    }
}

impl<'a> Iterator for PositionedCharacters<'a> {
    type Item = (CharacterPosition, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.indexed_characters
            .next()
            .map(|(utf_8_index, character)| {
                let position = CharacterPosition {
                    byte: self.byte_index,
                    utf_8: utf_8_index,
                };
                self.byte_index += character.len_utf8();
                (position, character)
            })
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

fn parse_numeric_literal_type_suffix<'a>(
    positioned_characters: &mut Peekable<PositionedCharacters<'a>>,
    line: &'a str,
    line_index: usize,
    type_suffix_start_character_position: CharacterPosition,
    value: &'a str,
    is_floating_point: bool,
) -> Result<CharacterPosition, LexicalError<'a>> {
    parse_identifier(
        positioned_characters,
        line,
        line_index,
        type_suffix_start_character_position,
    )
    .map_err(|error| match error {
        LexicalError::IdentifierIncomplete { position, string } => {
            LexicalError::NumericLiteralTypeSuffixIncomplete {
                position,
                string,
                value,
                value_kind: if is_floating_point {
                    NumericLiteralValueKind::FloatingPoint
                } else {
                    NumericLiteralValueKind::Integer
                },
            }
        }
        LexicalError::IdentifierUnexpectedCharacter {
            character,
            expected,
            position,
            string,
        } => LexicalError::NumericLiteralTypeSuffixUnexpectedCharacter {
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
        _ => unreachable!(),
    })
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

fn parse_floating_point_numeric_literal_exponent<'a>(
    positioned_characters: &mut Peekable<PositionedCharacters<'a>>,
    line: &'a str,
    line_index: usize,
    start_character_position: CharacterPosition,
) -> Result<(), LexicalError<'a>> {
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
            return Err(
                LexicalError::NumericLiteralValueUnexpectedCharacter {
                    string: &line[start_character_position.byte
                        ..unexpected_character_end_position.byte],
                    expected: NUMERIC_CHARACTERS,
                    position: SubstringPosition {
                        start_line: line_index,
                        end_line: line_index,
                        start_character: start_character_position,
                        end_character: unexpected_character_end_position,
                    },
                    character: candidate,
                    kind: NumericLiteralValueKind::FloatingPoint,
                },
            );
        }
    } else {
        return Err(LexicalError::NumericLiteralValueIncomplete {
            string: &line[start_character_position.byte..],
            position: SubstringPosition {
                start_line: line_index,
                end_line: line_index,
                start_character: start_character_position,
                end_character: to_line_end_position(line),
            },
            kind: NumericLiteralValueKind::FloatingPoint,
        });
    };
    loop {
        if let Some((_, candidate)) = positioned_characters.peek() {
            if is_numeric_character(*candidate) {
                let _ = positioned_characters.next();
            } else {
                break;
            }
        } else {
            return Err(LexicalError::NumericLiteralTypeSuffixIncomplete {
                string: &line[start_character_position.byte..],
                position: SubstringPosition {
                    start_line: line_index,
                    end_line: line_index,
                    start_character: start_character_position,
                    end_character: to_line_end_position(line),
                },
                value: &line[start_character_position.byte..],
                value_kind: NumericLiteralValueKind::FloatingPoint,
            });
        }
    }
    Ok(())
}

fn parse_identifier<'a>(
    positioned_characters: &mut Peekable<PositionedCharacters<'a>>,
    line: &'a str,
    line_index: usize,
    start_character_position: CharacterPosition,
) -> Result<CharacterPosition, LexicalError<'a>> {
    if let Some((_, candidate)) = positioned_characters.next() {
        if !STARTING_IDENTIFIER_CHARACTERS.contains(candidate) {
            return Err(LexicalError::IdentifierUnexpectedCharacter {
                character: candidate,
                expected: STARTING_IDENTIFIER_CHARACTERS,
                position: SubstringPosition {
                    start_line: line_index,
                    end_line: line_index,
                    start_character: start_character_position,
                    end_character: to_line_end_position(line),
                },
                string: &line[start_character_position.byte..],
            });
        }
    } else {
        return Err(LexicalError::IdentifierIncomplete {
            string: &line[start_character_position.byte..],
            position: SubstringPosition {
                start_line: line_index,
                end_line: line_index,
                start_character: start_character_position,
                end_character: to_line_end_position(line),
            },
        });
    };
    Ok(parse_non_starting_identifier_characters(
        positioned_characters,
        start_character_position,
    ))
}

fn parse_non_starting_identifier_characters(
    positioned_characters: &mut Peekable<PositionedCharacters<'_>>,
    identifier_start_character_position: CharacterPosition,
) -> CharacterPosition {
    loop {
        if let Some((candidate_position, candidate)) =
            positioned_characters.peek()
        {
            if !is_non_starting_identifier_character(*candidate) {
                return *candidate_position;
            }
        } else {
            return CharacterPosition {
                byte: identifier_start_character_position.byte + 1usize,
                utf_8: identifier_start_character_position.utf_8 + 1usize,
            };
        }
        let (candidate_position, _) =
            unsafe { positioned_characters.next().unwrap_unchecked() };
        if positioned_characters.peek().is_none() {
            return CharacterPosition {
                byte: candidate_position.byte + 1usize,
                utf_8: candidate_position.utf_8 + 1usize,
            };
        }
    }
}

fn to_line_end_position(line: &str) -> CharacterPosition {
    CharacterPosition {
        byte: line.len(),
        utf_8: line.chars().count(),
    }
}
