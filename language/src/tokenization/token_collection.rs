use std::iter::Peekable;

use super::byte_count::ByteCount;
use super::byte_size::ByteSize;
use super::character_position::CharacterPosition;
use super::constants::{
    ARROW, ASSIGNMENT, ASTERISK, CLOSE_BRACE, CLOSE_PARENTHESIS, COLON, COMMA,
    COMMENT_BLOCK_PREFIX, COMMENT_BLOCK_SUFFIX, COMMENT_LINE_PREFIX, DOT,
    EQUAL_TO, F32_NAME, F64_NAME, GREATER_THAN, GREATER_THAN_OR_EQUAL_TO,
    I16_NAME, I32_NAME, I64_NAME, I8_NAME, ISIZE_NAME, LESS_THAN,
    LESS_THAN_OR_EQUAL_TO, MINUS, NEWLINE, NOT_EQUAL_TO, NUMERIC_CHARACTERS,
    OPEN_BRACE, OPEN_PARENTHESIS, PLUS, SEMICOLON, SLASH,
    STARTING_IDENTIFIER_CHARACTERS, TYPE_SUFFIX_SEPARATOR,
    TYPE_SUFFIX_SEPARATOR_STRING, U16_NAME, U32_NAME, U64_NAME, U8_NAME,
    USIZE_NAME,
};
use super::contracts::{
    is_non_newline_whitespace, is_non_starting_identifier_character,
    is_numeric_character, is_starting_identifier_character,
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
use super::positioned_characters::PositionedCharacters;
use super::substring_position::SubstringPosition;
use super::token::{
    token_content_to_expected_end_character_position, Token,
    TokenContentsValidationError, TokenPositionsValidationError,
};
use super::token_content::TokenContent;
use super::utf_8_count::Utf8Count;
use super::utf_8_size::Utf8Size;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TokenCollection<StringType>(Box<[Token<StringType>]>);

impl<StringType> TokenCollection<StringType> {
    pub fn new(tokens: Vec<Token<StringType>>) -> Self {
        Self(tokens.into_boxed_slice())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Token<StringType>> {
        self.0.iter()
    }
}

pub struct TokenCollectionOwningIterator<StringType>(
    std::vec::IntoIter<Token<StringType>>,
);

impl<StringType> Iterator for TokenCollectionOwningIterator<StringType> {
    type Item = Token<StringType>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<StringType> IntoIterator for TokenCollection<StringType> {
    type Item = Token<StringType>;
    type IntoIter = TokenCollectionOwningIterator<StringType>;

    fn into_iter(self) -> Self::IntoIter {
        TokenCollectionOwningIterator(Vec::from(self.0).into_iter())
    }
}

impl<StringType: AsRef<str>> TokenCollection<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<TokenCollectionContentsValidationError>> {
        let mut errors = vec![];
        for (index, token) in self.0.iter().enumerate() {
            if let Err(token_errors) = token.validate_contents_impl() {
                errors.extend(token_errors.into_iter().map(|error| {
                    TokenCollectionContentsValidationError { index, error }
                }));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size>
    TokenCollection<StringType>
{
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(CharacterPosition {
            byte: ByteCount::default(),
            utf_8: Utf8Count::default(),
        })
    }

    fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> Result<(), Vec<TokenCollectionPositionsValidationError>> {
        let mut errors = vec![];
        let mut token_expected_start_byte_index =
            expected_start_character_position.byte;
        let mut token_expected_start_utf_8_index =
            expected_start_character_position.utf_8;
        for (index, token) in self.iter().enumerate() {
            if let Err(token_errors) =
                token.validate_positions_impl(CharacterPosition {
                    byte: token_expected_start_byte_index,
                    utf_8: token_expected_start_utf_8_index,
                })
            {
                errors.extend(token_errors.into_iter().map(|error| {
                    TokenCollectionPositionsValidationError { error, index }
                }));
            }
            CharacterPosition {
                byte: token_expected_start_byte_index,
                utf_8: token_expected_start_utf_8_index,
            } = token_content_to_expected_end_character_position(
                &token.content,
                CharacterPosition {
                    byte: token_expected_start_byte_index,
                    utf_8: token_expected_start_utf_8_index,
                },
            );
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug)]
pub(crate) struct TokenCollectionContentsValidationError {
    error: TokenContentsValidationError,
    index: usize,
}

#[derive(Debug)]
pub(crate) struct TokenCollectionPositionsValidationError {
    error: TokenPositionsValidationError,
    index: usize,
}

impl std::fmt::Display for TokenCollectionContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            formatter,
            "invalid token contents with index {}: {}",
            self.index, self.error
        )
    }
}

impl std::fmt::Display for TokenCollectionPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            formatter,
            "invalid token positions with index {}: {}",
            self.index, self.error
        )
    }
}

impl<StringType: AsRef<str>> std::fmt::Display
    for TokenCollection<StringType>
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for token in self.0.iter() {
            std::fmt::Display::fmt(&token.content, formatter)?;
        }
        Ok(())
    }
}

impl std::error::Error for TokenCollectionContentsValidationError {}

impl std::error::Error for TokenCollectionPositionsValidationError {}

impl<'a, StringType: From<&'a str>> TryFrom<&'a str>
    for TokenCollection<StringType>
{
    type Error = LexicalError<StringType>;

    fn try_from(string: &'a str) -> Result<Self, Self::Error> {
        let mut tokens = vec![];
        let mut positioned_characters =
            PositionedCharacters::from(string).peekable();
        'outer: while let Some((mut character_position, mut character)) =
            positioned_characters.next()
        {
            if is_non_newline_whitespace(character) {
                let whitespace_start_character_position = character_position;
                loop {
                    if let Some((candidate_position, candidate)) =
                        positioned_characters.next()
                    {
                        if !is_non_newline_whitespace(candidate) {
                            tokens.push(Token {
                                    position: SubstringPosition {
                                        start: whitespace_start_character_position,
                                        end: candidate_position,
                                    },
                                    content: TokenContent::Whitespace(
                                        StringType::from(slice_string_by_character_positions(
                                            string,
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
                                start: whitespace_start_character_position,
                                end: to_string_end_position(string),
                            },
                            content: TokenContent::Whitespace(
                                StringType::from(
                                    slice_string_by_byte_indices(
                                        string,
                                        whitespace_start_character_position
                                            .byte,
                                        string.byte_size(),
                                    ),
                                ),
                            ),
                        });
                        break 'outer;
                    }
                }
            }
            debug_assert!(!is_non_newline_whitespace(character));
            if character == SLASH {
                if let Some((_, '/')) = positioned_characters.peek() {
                    let _ = positioned_characters.next();
                    let comment_line_start_character_position =
                        character_position;
                    let string_after_comment_line_prefix =
                        tail_string_from_byte_index(
                            string,
                            comment_line_start_character_position.byte
                                + COMMENT_LINE_PREFIX.byte_size(),
                        );
                    match string_after_comment_line_prefix.find(NEWLINE) {
                        Some(newline_raw_byte_offset) => {
                            let comment_line_after_prefix_characters_count =
                                string_after_comment_line_prefix
                                    [..newline_raw_byte_offset]
                                    .utf_8_size();
                            skip_characters_in_place(
                                &mut positioned_characters,
                                comment_line_after_prefix_characters_count
                                    + NEWLINE.utf_8_size(),
                            );
                            let comment_line_end_character_position =
                                CharacterPosition {
                                    byte: comment_line_start_character_position
                                        .byte
                                        + COMMENT_LINE_PREFIX.byte_size()
                                        + ByteCount::from(
                                            newline_raw_byte_offset,
                                        ),
                                    utf_8:
                                        comment_line_start_character_position
                                            .utf_8
                                            + COMMENT_LINE_PREFIX.utf_8_size()
                                            + comment_line_after_prefix_characters_count,
                                };
                            tokens.push(Token {
                                    position: SubstringPosition {
                                        start: comment_line_start_character_position,
                                        end: comment_line_end_character_position,
                                    },
                                    content: TokenContent::CommentLine(
                                        StringType::from(
                                            slice_string_by_character_positions(
                                                string,
                                                comment_line_start_character_position,
                                                comment_line_end_character_position,
                                            ),
                                        ),
                                    ),
                                });
                            tokens.push(construct_token_from_character(
                                comment_line_end_character_position,
                                NEWLINE,
                                TokenContent::Newline,
                            ));
                            continue;
                        }
                        None => {
                            tokens.push(Token {
                                position: SubstringPosition {
                                    start: comment_line_start_character_position,
                                    end: to_string_end_position(string),
                                },
                                content: TokenContent::CommentLine(
                                    StringType::from(tail_string_from_position(
                                        string,
                                        comment_line_start_character_position,
                                    )),
                                ),
                            });
                            break;
                        }
                    };
                } else if let Some((_, '*')) = positioned_characters.peek() {
                    let _ = positioned_characters.next();
                    let comment_block_start_character_position =
                        character_position;
                    let comment_block_after_prefix_start_byte_index =
                        comment_block_start_character_position.byte
                            + COMMENT_BLOCK_PREFIX.byte_size();
                    if let Some(comment_block_suffix_start_byte_offset) =
                        tail_string_from_byte_index(
                            string,
                            comment_block_after_prefix_start_byte_index,
                        )
                        .find(COMMENT_BLOCK_SUFFIX)
                        .map(ByteCount::from)
                    {
                        let comment_block_end_byte_index =
                            comment_block_after_prefix_start_byte_index
                                + comment_block_suffix_start_byte_offset
                                + COMMENT_BLOCK_SUFFIX.byte_size();
                        let comment_block_after_prefix_characters_count =
                            slice_string_by_byte_indices(
                                string,
                                comment_block_after_prefix_start_byte_index,
                                comment_block_end_byte_index,
                            )
                            .utf_8_size();
                        skip_characters_in_place(
                            &mut positioned_characters,
                            comment_block_after_prefix_characters_count,
                        );
                        let comment_block_end_character_position =
                            CharacterPosition {
                                byte: comment_block_end_byte_index,
                                utf_8: comment_block_start_character_position.utf_8
                                    + COMMENT_BLOCK_PREFIX.utf_8_size()
                                    + comment_block_after_prefix_characters_count,
                            };
                        tokens.push(Token {
                            position: SubstringPosition {
                                start: comment_block_start_character_position,
                                end: comment_block_end_character_position,
                            },
                            content: TokenContent::CommentBlock(
                                StringType::from(
                                    slice_string_by_character_positions(
                                        string,
                                        comment_block_start_character_position,
                                        comment_block_end_character_position,
                                    ),
                                ),
                            ),
                        });
                        continue;
                    } else {
                        return Err(LexicalError::CommentBlockIncomplete(
                            CommentBlockIncomplete {
                                position: SubstringPosition {
                                    start:
                                        comment_block_start_character_position,
                                    end: to_string_end_position(string),
                                },
                                string: StringType::from(
                                    tail_string_from_position(
                                        string,
                                        comment_block_start_character_position,
                                    ),
                                ),
                            },
                        ));
                    }
                } else {
                    tokens.push(construct_token_from_character(
                        character_position,
                        SLASH,
                        TokenContent::Slash,
                    ));
                    continue;
                }
            }
            let token = match character {
                ASSIGNMENT => {
                    if let Some((_, '=')) = positioned_characters.peek() {
                        let _ = positioned_characters.next();
                        construct_token_from_string(
                            character_position,
                            EQUAL_TO,
                            TokenContent::EqualTo,
                        )
                    } else {
                        construct_token_from_character(
                            character_position,
                            ASSIGNMENT,
                            TokenContent::Assignment,
                        )
                    }
                }
                ASTERISK => construct_token_from_character(
                    character_position,
                    ASTERISK,
                    TokenContent::Asterisk,
                ),
                CLOSE_BRACE => construct_token_from_character(
                    character_position,
                    CLOSE_BRACE,
                    TokenContent::CloseBrace,
                ),
                CLOSE_PARENTHESIS => construct_token_from_character(
                    character_position,
                    CLOSE_PARENTHESIS,
                    TokenContent::CloseParenthesis,
                ),
                COLON => construct_token_from_character(
                    character_position,
                    COLON,
                    TokenContent::Colon,
                ),
                COMMA => construct_token_from_character(
                    character_position,
                    COMMA,
                    TokenContent::Comma,
                ),
                DOT => {
                    if matches!(
                        positioned_characters.peek(),
                        Some((_, candidate)) if is_numeric_character(*candidate)
                    ) {
                        // "floating-point numeric literal starting with dot" case
                        let _ = positioned_characters.next();
                        parse_floating_point_literal_starting_with_dot(
                            &mut positioned_characters,
                            string,
                            character_position,
                        )?
                    } else {
                        // "dot operator" case
                        construct_token_from_character(
                            character_position,
                            DOT,
                            TokenContent::Dot,
                        )
                    }
                }
                GREATER_THAN => {
                    if let Some((_, '=')) = positioned_characters.peek() {
                        let _ = positioned_characters.next();
                        construct_token_from_string(
                            character_position,
                            GREATER_THAN_OR_EQUAL_TO,
                            TokenContent::GreaterThanOrEqualTo,
                        )
                    } else {
                        construct_token_from_character(
                            character_position,
                            GREATER_THAN,
                            TokenContent::GreaterThan,
                        )
                    }
                }
                LESS_THAN => {
                    if let Some((_, '=')) = positioned_characters.peek() {
                        let _ = positioned_characters.next();
                        construct_token_from_string(
                            character_position,
                            LESS_THAN_OR_EQUAL_TO,
                            TokenContent::LessThanOrEqualTo,
                        )
                    } else {
                        construct_token_from_character(
                            character_position,
                            LESS_THAN,
                            TokenContent::LessThan,
                        )
                    }
                }
                MINUS => {
                    if matches!(positioned_characters.peek(), Some((_, '>'))) {
                        let _ = positioned_characters.next();
                        construct_token_from_string(
                            character_position,
                            ARROW,
                            TokenContent::Arrow,
                        )
                    } else {
                        construct_token_from_character(
                            character_position,
                            MINUS,
                            TokenContent::Minus,
                        )
                    }
                }
                NEWLINE => construct_token_from_character(
                    character_position,
                    NEWLINE,
                    TokenContent::Newline,
                ),
                OPEN_BRACE => construct_token_from_character(
                    character_position,
                    OPEN_BRACE,
                    TokenContent::OpenBrace,
                ),
                OPEN_PARENTHESIS => construct_token_from_character(
                    character_position,
                    OPEN_PARENTHESIS,
                    TokenContent::OpenParenthesis,
                ),
                PLUS => construct_token_from_character(
                    character_position,
                    PLUS,
                    TokenContent::Plus,
                ),
                SEMICOLON => construct_token_from_character(
                    character_position,
                    SEMICOLON,
                    TokenContent::Semicolon,
                ),
                '!' => {
                    if let Some((_, '=')) = positioned_characters.peek() {
                        let _ = positioned_characters.next();
                        construct_token_from_string(
                            character_position,
                            NOT_EQUAL_TO,
                            TokenContent::NotEqualTo,
                        )
                    } else {
                        return Err(LexicalError::UnexpectedCharacter(
                            UnexpectedCharacter {
                                character,
                                position: SubstringPosition {
                                    start: character_position,
                                    end: to_string_end_position(string),
                                },
                                string: StringType::from(
                                    tail_string_from_position(
                                        string,
                                        character_position,
                                    ),
                                ),
                            },
                        ));
                    }
                }
                _ if is_numeric_character(character) => parse_numeric_literal(
                    &mut positioned_characters,
                    string,
                    character_position,
                )?,
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
                            start: identifier_start_character_position,
                            end: identifier_end_character_position,
                        },
                        content: TokenContent::Identifier(StringType::from(
                            slice_string_by_character_positions(
                                string,
                                identifier_start_character_position,
                                identifier_end_character_position,
                            ),
                        )),
                    }
                }
                _ => {
                    return Err(LexicalError::UnexpectedCharacter(
                        UnexpectedCharacter {
                            character,
                            position: SubstringPosition {
                                start: character_position,
                                end: to_string_end_position(string),
                            },
                            string: StringType::from(
                                tail_string_from_position(
                                    string,
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
        Ok(Self(tokens.into_boxed_slice()))
    }
}

fn construct_token_from_string<'a, StringType: From<&'a str>>(
    start_character_position: CharacterPosition,
    value: &'a str,
    content: TokenContent<StringType>,
) -> Token<StringType> {
    Token {
        position: SubstringPosition {
            start: start_character_position,
            end: CharacterPosition {
                byte: start_character_position.byte + value.byte_size(),
                utf_8: start_character_position.utf_8 + value.utf_8_size(),
            },
        },
        content,
    }
}

fn construct_token_from_character<'a, StringType: From<&'a str>>(
    start_character_position: CharacterPosition,
    value: char,
    content: TokenContent<StringType>,
) -> Token<StringType> {
    Token {
        position: SubstringPosition {
            start: start_character_position,
            end: CharacterPosition {
                byte: start_character_position.byte + value.byte_size(),
                utf_8: start_character_position.utf_8 + value.utf_8_size(),
            },
        },
        content,
    }
}

fn parse_numeric_literal<'a, StringType: From<&'a str>>(
    positioned_characters: &mut Peekable<PositionedCharacters<'a>>,
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
                    start: start_character_position,
                    end: to_string_end_position(line),
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
                        start: start_character_position,
                        end: to_string_end_position(line),
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
                        start: start_character_position,
                        end: to_string_end_position(line),
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
                            start: start_character_position,
                            end: to_string_end_position(line),
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
            start_character_position,
        )?;
        (value_end_character_position, character) =
            positioned_characters.next().ok_or_else(|| {
                LexicalError::NumericLiteralTypeSuffixIncomplete(
                    NumericLiteralTypeSuffixIncomplete {
                        position: SubstringPosition {
                            start: start_character_position,
                            end: to_string_end_position(line),
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
    let value = StringType::from(slice_string_by_character_positions(
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
                        start: start_character_position,
                        end: to_string_end_position(line),
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
        type_suffix_start_character_position,
        value,
        is_floating_point,
    )?;
    let string = StringType::from(slice_string_by_character_positions(
        line,
        start_character_position,
        end_character_position,
    ));
    let type_suffix = slice_string_by_character_positions(
        line,
        type_suffix_start_character_position,
        end_character_position,
    );
    let token_position = SubstringPosition {
        start: start_character_position,
        end: end_character_position,
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
    string: &'a str,
    start_character_position: CharacterPosition,
) -> Result<Token<StringType>, LexicalError<StringType>> {
    parse_digits(positioned_characters).ok_or_else(|| {
        LexicalError::NumericLiteralTypeSuffixIncomplete(
            NumericLiteralTypeSuffixIncomplete {
                string: StringType::from(tail_string_from_position(
                    string,
                    start_character_position,
                )),
                position: SubstringPosition {
                    start: start_character_position,
                    end: to_string_end_position(string),
                },
                value: StringType::from(tail_string_from_position(
                    string,
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
                        string,
                        start_character_position,
                    )),
                    position: SubstringPosition {
                        start: start_character_position,
                        end: to_string_end_position(string),
                    },
                    value: StringType::from(tail_string_from_position(
                        string,
                        start_character_position,
                    )),
                    value_kind: NumericLiteralValueKind::FloatingPoint,
                },
            )
        })?;
    if character == 'e' || character == 'E' {
        parse_floating_point_numeric_literal_exponent(
            positioned_characters,
            string,
            start_character_position,
        )?;
        (value_end_character_position, character) =
            positioned_characters.next().ok_or_else(|| {
                LexicalError::NumericLiteralTypeSuffixIncomplete(
                    NumericLiteralTypeSuffixIncomplete {
                        string: StringType::from(tail_string_from_position(
                            string,
                            start_character_position,
                        )),
                        position: SubstringPosition {
                            start: start_character_position,
                            end: to_string_end_position(string),
                        },
                        value: StringType::from(tail_string_from_position(
                            string,
                            start_character_position,
                        )),
                        value_kind: NumericLiteralValueKind::FloatingPoint,
                    },
                )
            })?;
    }
    let value = StringType::from(slice_string_by_character_positions(
        string,
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
                        start: start_character_position,
                        end: to_string_end_position(string),
                    },
                    string: StringType::from(tail_string_from_position(
                        string,
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
        string,
        type_suffix_start_character_position,
        value,
        true,
    )?;
    let token_position = SubstringPosition {
        start: start_character_position,
        end: end_character_position,
    };
    let type_suffix = slice_string_by_character_positions(
        string,
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
                    string: StringType::from(
                        slice_string_by_character_positions(
                            string,
                            start_character_position,
                            end_character_position,
                        ),
                    ),
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
                    string: StringType::from(
                        slice_string_by_character_positions(
                            string,
                            start_character_position,
                            end_character_position,
                        ),
                    ),
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

fn parse_numeric_literal_type_suffix<'a, StringType: From<&'a str>>(
    positioned_characters: &mut Peekable<PositionedCharacters<'a>>,
    string: &'a str,
    type_suffix_start_character_position: CharacterPosition,
    value: StringType,
    is_floating_point: bool,
) -> Result<(CharacterPosition, StringType), LexicalError<StringType>> {
    match parse_identifier(
        positioned_characters,
        string,
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
    value: &'a str,
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
                .unwrap_or_else(|| to_string_end_position(value));
            return Err(LexicalError::NumericLiteralValueUnexpectedCharacter(
                NumericLiteralValueUnexpectedCharacter {
                    string: StringType::from(
                        slice_string_by_character_positions(
                            value,
                            start_character_position,
                            unexpected_character_end_position,
                        ),
                    ),
                    expected: StringType::from(NUMERIC_CHARACTERS),
                    position: SubstringPosition {
                        start: start_character_position,
                        end: unexpected_character_end_position,
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
                    value,
                    start_character_position,
                )),
                position: SubstringPosition {
                    start: start_character_position,
                    end: to_string_end_position(value),
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
                        value,
                        start_character_position,
                    )),
                    position: SubstringPosition {
                        start: start_character_position,
                        end: to_string_end_position(value),
                    },
                    value: StringType::from(tail_string_from_position(
                        value,
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
    string: &'a str,
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
                        start: start_character_position,
                        end: to_string_end_position(string),
                    },
                    string: StringType::from(tail_string_from_position(
                        string,
                        start_character_position,
                    )),
                },
            ));
        }
    } else {
        return Err(LexicalError::IdentifierIncomplete(
            IdentifierIncomplete {
                string: StringType::from(tail_string_from_position(
                    string,
                    start_character_position,
                )),
                position: SubstringPosition {
                    start: start_character_position,
                    end: to_string_end_position(string),
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

fn skip_characters_in_place(
    positioned_characters: &mut Peekable<PositionedCharacters<'_>>,
    count: Utf8Count,
) {
    for _ in 0usize..count.into() {
        let item = positioned_characters.next();
        debug_assert!(item.is_some());
    }
}

fn slice_string_by_byte_indices(
    line: &str,
    start_byte_index: ByteCount,
    end_byte_index: ByteCount,
) -> &str {
    &line[start_byte_index.into()..end_byte_index.into()]
}

fn slice_string_by_character_positions(
    line: &str,
    start_character_position: CharacterPosition,
    end_character_position: CharacterPosition,
) -> &str {
    slice_string_by_byte_indices(
        line,
        start_character_position.byte,
        end_character_position.byte,
    )
}

fn tail_string_from_position(line: &str, position: CharacterPosition) -> &str {
    tail_string_from_byte_index(line, position.byte)
}

fn tail_string_from_byte_index(line: &str, byte_index: ByteCount) -> &str {
    &line[byte_index.into()..]
}

fn to_string_end_position(value: &str) -> CharacterPosition {
    CharacterPosition {
        byte: value.byte_size(),
        utf_8: value.utf_8_size(),
    }
}
