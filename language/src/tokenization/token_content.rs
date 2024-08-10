use super::byte_count::ByteCount;
use super::byte_size::ByteSize;
use super::character_position::CharacterPosition;
use super::constants::{
    ARROW, ASSIGNMENT, ASTERISK, CLOSE_BRACE, CLOSE_PARENTHESIS, COLON, COMMA,
    DOT, EQUAL_TO, GREATER_THAN, GREATER_THAN_OR_EQUAL_TO, LESS_THAN,
    LESS_THAN_OR_EQUAL_TO, MINUS, NEWLINE, NOT_EQUAL_TO, OPEN_BRACE,
    OPEN_PARENTHESIS, PLUS, SEMICOLON, SLASH, TYPE_SUFFIX_SEPARATOR,
};
use super::numeric_literal_type::NumericLiteralType;
use super::token_content_string_validation::{
    validate_comment_block_lines, validate_comment_line_string,
    validate_floating_point_literal_string, validate_identifier_string,
    validate_integer_literal_string, validate_whitespace_string,
    CommentBlockLinesValidationError, CommentLineStringValidationError,
    IdentifierStringValidationError, NumericLiteralValueValidationError,
    WhitespaceStringValidationError,
};
use super::utf_8_count::Utf8Count;
use super::utf_8_size::Utf8Size;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenContent<StringType> {
    Arrow,
    Assignment,
    Asterisk,
    CloseBrace,
    CloseParenthesis,
    Colon,
    Comma,
    CommentBlock(StringType),
    CommentLine(StringType),
    Dot,
    EqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    NumericLiteral {
        value: StringType,
        type_: NumericLiteralType,
    },
    Identifier(StringType),
    LessThan,
    LessThanOrEqualTo,
    Minus,
    Newline,
    NotEqualTo,
    OpenBrace,
    OpenParenthesis,
    Plus,
    Semicolon,
    Slash,
    Whitespace(StringType),
}

impl<StringType: AsRef<str>> TokenContent<StringType> {
    pub(crate) fn as_ref(&self) -> TokenContent<&str> {
        match self {
            Self::Arrow => TokenContent::Arrow,
            Self::Assignment => TokenContent::Assignment,
            Self::Asterisk => TokenContent::Asterisk,
            Self::CloseBrace => TokenContent::CloseBrace,
            Self::CloseParenthesis => TokenContent::CloseParenthesis,
            Self::Colon => TokenContent::Colon,
            Self::Comma => TokenContent::Comma,
            Self::CommentBlock(value) => {
                TokenContent::CommentBlock(value.as_ref())
            }
            Self::CommentLine(value) => {
                TokenContent::CommentLine(value.as_ref())
            }
            Self::Dot => TokenContent::Dot,
            Self::EqualTo => TokenContent::EqualTo,
            Self::GreaterThan => TokenContent::GreaterThan,
            Self::GreaterThanOrEqualTo => TokenContent::GreaterThanOrEqualTo,
            TokenContent::NumericLiteral { value, type_ } => {
                TokenContent::NumericLiteral {
                    value: value.as_ref(),
                    type_: *type_,
                }
            }
            Self::Identifier(value) => {
                TokenContent::Identifier(value.as_ref())
            }
            Self::LessThan => TokenContent::LessThan,
            Self::LessThanOrEqualTo => TokenContent::LessThanOrEqualTo,
            Self::Minus => TokenContent::Minus,
            Self::Newline => TokenContent::Newline,
            Self::NotEqualTo => TokenContent::NotEqualTo,
            Self::OpenBrace => TokenContent::OpenBrace,
            Self::OpenParenthesis => TokenContent::OpenParenthesis,
            Self::Plus => TokenContent::Plus,
            Self::Semicolon => TokenContent::Semicolon,
            Self::Slash => TokenContent::Slash,
            Self::Whitespace(value) => {
                TokenContent::Whitespace(value.as_ref())
            }
        }
    }
}

impl<StringType: AsRef<str>> TokenContent<StringType> {
    pub fn validate(&self) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_impl(CharacterPosition {
            byte: ByteCount::default(),
            utf_8: Utf8Count::default(),
        })
    }

    pub(super) fn validate_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> Result<(), Vec<TokenContentValidationError>> {
        match self {
            Self::Arrow
            | Self::Assignment
            | Self::Asterisk
            | Self::CloseBrace
            | Self::CloseParenthesis
            | Self::Colon
            | Self::Comma
            | Self::Dot
            | Self::EqualTo
            | Self::GreaterThan
            | Self::GreaterThanOrEqualTo
            | Self::LessThan
            | Self::LessThanOrEqualTo
            | Self::Minus
            | Self::Newline
            | Self::NotEqualTo
            | Self::OpenBrace
            | Self::OpenParenthesis
            | Self::Plus
            | Self::Semicolon
            | Self::Slash => Ok(()),
            Self::CommentBlock(value) => validate_comment_block_lines(
                value.as_ref()
                    .split_inclusive(NEWLINE)
                    .collect::<Vec<_>>()
                    .as_slice(),
                expected_start_character_position,
            )
            .map_err(|errors| {
                errors
                    .into_iter()
                    .map(|error| TokenContentValidationError(
                        TokenContentValidationErrorKind::CommentBlock(error)
                    ))
                    .collect()
            }),
            Self::CommentLine(value) => {
                validate_comment_line_string(value).map_err(|errors| {
                    errors
                        .into_iter()
                        .map(|error| TokenContentValidationError(
                            TokenContentValidationErrorKind::CommentLine(error)
                        ))
                        .collect()
                })
            }
            Self::NumericLiteral { value, type_ } => match type_ {
                NumericLiteralType::F32 | NumericLiteralType::F64 => {
                    validate_floating_point_literal_string(
                        value,
                        expected_start_character_position,
                    )
                    .map_err(|errors| {
                        errors
                            .into_iter()
                            .map(|error| TokenContentValidationError(
                                TokenContentValidationErrorKind::NumericLiteral(error)
                            ))
                            .collect()
                    })
                }
                NumericLiteralType::I8
                | NumericLiteralType::I16
                | NumericLiteralType::I32
                | NumericLiteralType::I64
                | NumericLiteralType::ISize
                | NumericLiteralType::U8
                | NumericLiteralType::U16
                | NumericLiteralType::U32
                | NumericLiteralType::U64
                | NumericLiteralType::USize => {
                    validate_integer_literal_string(
                        value,
                        expected_start_character_position,
                    )
                    .map_err(|errors| {
                        errors
                            .into_iter()
                            .map(|error| TokenContentValidationError(
                                TokenContentValidationErrorKind::NumericLiteral(error)
                            ))
                            .collect()
                    })
                }
            },
            Self::Identifier(value) => validate_identifier_string(
                value,
                expected_start_character_position,
            )
            .map_err(|errors| {
                errors
                    .into_iter()
                    .map(|error| TokenContentValidationError(
                        TokenContentValidationErrorKind::Identifier(error)
                    ))
                    .collect()
            }),
            Self::Whitespace(value) => validate_whitespace_string(
                value,
                expected_start_character_position,
            )
            .map_err(|errors| {
                errors
                    .into_iter()
                    .map(|error|TokenContentValidationError(TokenContentValidationErrorKind::Whitespace(error)))
                    .collect()
            }),
        }
    }
}

impl<StringType: ByteSize + Utf8Size> TokenContent<StringType> {
    pub(super) fn to_character_count(&self) -> CharacterPosition {
        match self {
            Self::Arrow => CharacterPosition {
                byte: ARROW.byte_size(),
                utf_8: ARROW.utf_8_size(),
            },
            Self::Assignment => CharacterPosition {
                byte: ASSIGNMENT.byte_size(),
                utf_8: ASSIGNMENT.utf_8_size(),
            },
            Self::Asterisk => CharacterPosition {
                byte: ASTERISK.byte_size(),
                utf_8: ASTERISK.utf_8_size(),
            },
            Self::CloseBrace => CharacterPosition {
                byte: CLOSE_BRACE.byte_size(),
                utf_8: CLOSE_BRACE.utf_8_size(),
            },
            Self::CloseParenthesis => CharacterPosition {
                byte: CLOSE_PARENTHESIS.byte_size(),
                utf_8: CLOSE_PARENTHESIS.utf_8_size(),
            },
            Self::Colon => CharacterPosition {
                byte: COLON.byte_size(),
                utf_8: COLON.utf_8_size(),
            },
            Self::Comma => CharacterPosition {
                byte: COMMA.byte_size(),
                utf_8: COMMA.utf_8_size(),
            },
            Self::CommentBlock(value) => CharacterPosition {
                byte: value.byte_size(),
                utf_8: value.utf_8_size(),
            },
            Self::CommentLine(value) => CharacterPosition {
                byte: value.byte_size(),
                utf_8: value.utf_8_size(),
            },
            Self::Dot => CharacterPosition {
                byte: DOT.byte_size(),
                utf_8: DOT.utf_8_size(),
            },
            Self::EqualTo => CharacterPosition {
                byte: EQUAL_TO.byte_size(),
                utf_8: EQUAL_TO.utf_8_size(),
            },
            Self::GreaterThan => CharacterPosition {
                byte: GREATER_THAN.byte_size(),
                utf_8: GREATER_THAN.utf_8_size(),
            },
            Self::GreaterThanOrEqualTo => CharacterPosition {
                byte: GREATER_THAN_OR_EQUAL_TO.byte_size(),
                utf_8: GREATER_THAN_OR_EQUAL_TO.utf_8_size(),
            },
            Self::NumericLiteral { value, type_ } => {
                let type_string = <&'static str>::from(type_);
                CharacterPosition {
                    byte: value.byte_size() + type_string.byte_size(),
                    utf_8: value.utf_8_size() + type_string.utf_8_size(),
                }
            }
            Self::Identifier(value) => CharacterPosition {
                byte: value.byte_size(),
                utf_8: value.utf_8_size(),
            },
            Self::LessThan => CharacterPosition {
                byte: LESS_THAN.byte_size(),
                utf_8: LESS_THAN.utf_8_size(),
            },
            Self::LessThanOrEqualTo => CharacterPosition {
                byte: LESS_THAN_OR_EQUAL_TO.byte_size(),
                utf_8: LESS_THAN_OR_EQUAL_TO.utf_8_size(),
            },
            Self::Minus => CharacterPosition {
                byte: MINUS.byte_size(),
                utf_8: MINUS.utf_8_size(),
            },
            Self::Newline => CharacterPosition {
                byte: NEWLINE.byte_size(),
                utf_8: NEWLINE.utf_8_size(),
            },
            Self::NotEqualTo => CharacterPosition {
                byte: NOT_EQUAL_TO.byte_size(),
                utf_8: NOT_EQUAL_TO.utf_8_size(),
            },
            Self::OpenBrace => CharacterPosition {
                byte: OPEN_BRACE.byte_size(),
                utf_8: OPEN_BRACE.utf_8_size(),
            },
            Self::OpenParenthesis => CharacterPosition {
                byte: OPEN_PARENTHESIS.byte_size(),
                utf_8: OPEN_PARENTHESIS.utf_8_size(),
            },
            Self::Plus => CharacterPosition {
                byte: PLUS.byte_size(),
                utf_8: PLUS.utf_8_size(),
            },
            Self::Semicolon => CharacterPosition {
                byte: SEMICOLON.byte_size(),
                utf_8: SEMICOLON.utf_8_size(),
            },
            Self::Slash => CharacterPosition {
                byte: SLASH.byte_size(),
                utf_8: SLASH.utf_8_size(),
            },
            Self::Whitespace(value) => CharacterPosition {
                byte: value.byte_size(),
                utf_8: value.utf_8_size(),
            },
        }
    }
}

#[derive(Debug)]
pub(crate) struct TokenContentValidationError(TokenContentValidationErrorKind);

#[derive(Debug)]
enum TokenContentValidationErrorKind {
    CommentBlock(CommentBlockLinesValidationError),
    CommentLine(CommentLineStringValidationError),
    Identifier(IdentifierStringValidationError),
    NumericLiteral(NumericLiteralValueValidationError),
    Whitespace(WhitespaceStringValidationError),
}

impl std::fmt::Display for TokenContentValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl std::fmt::Display for TokenContentValidationErrorKind {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            TokenContentValidationErrorKind::CommentBlock(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            TokenContentValidationErrorKind::CommentLine(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            TokenContentValidationErrorKind::Identifier(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            TokenContentValidationErrorKind::NumericLiteral(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            TokenContentValidationErrorKind::Whitespace(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for TokenContentValidationError {}

impl<StringType: AsRef<str>> std::fmt::Display for TokenContent<StringType> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            TokenContent::Arrow => formatter.write_str(ARROW),
            TokenContent::Assignment => formatter.write_char(ASSIGNMENT),
            TokenContent::Asterisk => formatter.write_char(ASTERISK),
            TokenContent::CloseBrace => formatter.write_char(CLOSE_BRACE),
            TokenContent::CloseParenthesis => {
                formatter.write_char(CLOSE_PARENTHESIS)
            }
            TokenContent::Colon => formatter.write_char(COLON),
            TokenContent::Comma => formatter.write_char(COMMA),
            TokenContent::CommentBlock(value) => {
                formatter.write_str(value.as_ref())
            }
            TokenContent::CommentLine(value) => {
                formatter.write_str(value.as_ref())
            }
            TokenContent::Dot => formatter.write_char(DOT),
            TokenContent::EqualTo => formatter.write_str(EQUAL_TO),
            TokenContent::GreaterThan => formatter.write_char(GREATER_THAN),
            TokenContent::GreaterThanOrEqualTo => {
                formatter.write_str(GREATER_THAN_OR_EQUAL_TO)
            }
            TokenContent::Identifier(value) => {
                formatter.write_str(value.as_ref())
            }
            TokenContent::LessThan => formatter.write_char(LESS_THAN),
            TokenContent::LessThanOrEqualTo => {
                formatter.write_str(LESS_THAN_OR_EQUAL_TO)
            }
            TokenContent::Minus => formatter.write_char(MINUS),
            TokenContent::Newline => formatter.write_char(NEWLINE),
            TokenContent::NotEqualTo => formatter.write_str(NOT_EQUAL_TO),
            TokenContent::NumericLiteral { value, type_ } => {
                formatter.write_str(value.as_ref())?;
                formatter.write_char(TYPE_SUFFIX_SEPARATOR)?;
                formatter.write_str(type_.into())
            }
            TokenContent::OpenBrace => formatter.write_char(OPEN_BRACE),
            TokenContent::OpenParenthesis => {
                formatter.write_char(OPEN_PARENTHESIS)
            }
            TokenContent::Plus => formatter.write_char(PLUS),
            TokenContent::Semicolon => formatter.write_char(SEMICOLON),
            TokenContent::Slash => formatter.write_char(SLASH),
            TokenContent::Whitespace(value) => {
                formatter.write_str(value.as_ref())
            }
        }
    }
}
