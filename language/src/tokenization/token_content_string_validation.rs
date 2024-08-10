use super::byte_count::ByteCount;
use super::byte_size::ByteSize;
use super::character_position::CharacterPosition;
use super::constants::{
    COMMENT_BLOCK_PREFIX, COMMENT_BLOCK_SUFFIX, COMMENT_LINE_PREFIX, NEWLINE,
};
use super::contracts::{
    is_non_newline_whitespace, is_non_starting_identifier_character,
    is_numeric_character, is_starting_identifier_character,
};
use super::numeric_literal_value_kind::NumericLiteralValueKind;
use super::positioned_characters::PositionedCharacters;
use super::utf_8_size::Utf8Size;

#[derive(Debug)]
pub(crate) struct CommentBlockLinesValidationError(
    CommentBlockLinesValidationErrorKind,
);

#[derive(Debug)]
pub(crate) struct CommentLineStringValidationError(
    CommentLineStringValidationErrorKind,
);

#[derive(Debug)]
pub(crate) struct IdentifierStringValidationError(
    IdentifierStringValidationErrorKind,
);

#[derive(Debug)]
pub(crate) struct NumericLiteralValueValidationError(
    NumericLiteralValueValidationErrorKind,
);

#[derive(Debug)]
pub(crate) struct WhitespaceStringValidationError(
    WhitespaceStringValidationErrorKind,
);

#[derive(Debug)]
enum CommentBlockLinesValidationErrorKind {
    Empty,
    InvalidPrefix,
    InvalidSuffix,
    PrematureEnd { position: CharacterPosition },
}

#[derive(Debug)]
enum CommentLineStringValidationErrorKind {
    Empty,
    EndsWithNewline,
    InvalidPrefix,
}

#[derive(Debug)]
enum IdentifierStringValidationErrorKind {
    Empty,
    UnexpectedCharacter {
        position: CharacterPosition,
        value: char,
    },
}

#[derive(Debug)]
enum NumericLiteralValueValidationErrorKind {
    Empty,
    UnexpectedCharacter {
        kind: NumericLiteralValueKind,
        position: CharacterPosition,
        value: char,
    },
}

#[derive(Debug)]
enum WhitespaceStringValidationErrorKind {
    Empty,
    UnexpectedCharacter {
        position: CharacterPosition,
        value: char,
    },
}

impl std::fmt::Display for CommentBlockLinesValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl std::fmt::Display for CommentBlockLinesValidationErrorKind {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Empty => {
                write!(formatter, "comment block should not be empty")
            }
            Self::InvalidSuffix => {
                write!(
                    formatter,
                    "comment block should end with {:?}",
                    COMMENT_BLOCK_SUFFIX
                )
            }
            Self::InvalidPrefix => {
                write!(
                    formatter,
                    "comment block should start with {:?}",
                    COMMENT_BLOCK_PREFIX
                )
            }
            Self::PrematureEnd { position } => {
                write!(
                    formatter,
                    "comment block prematurely ends at position {}",
                    character_position_to_string(*position)
                )
            }
        }
    }
}

impl std::fmt::Display for CommentLineStringValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl std::fmt::Display for CommentLineStringValidationErrorKind {
    fn fmt(
        self: &CommentLineStringValidationErrorKind,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Empty => {
                write!(formatter, "comment line should not be empty")
            }
            Self::EndsWithNewline => {
                write!(
                    formatter,
                    "comment line should not end with {:?}",
                    NEWLINE
                )
            }
            Self::InvalidPrefix => {
                write!(
                    formatter,
                    "comment line should start with {:?}",
                    COMMENT_LINE_PREFIX
                )
            }
        }
    }
}

impl std::fmt::Display for IdentifierStringValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl std::fmt::Display for IdentifierStringValidationErrorKind {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Empty => {
                write!(formatter, "identifier should not be empty")
            }
            Self::UnexpectedCharacter { position, value } => {
                write!(
                    formatter,
                    "non-identifier character {:?} found at position {}",
                    value,
                    character_position_to_string(*position)
                )
            }
        }
    }
}

impl std::fmt::Display for NumericLiteralValueValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl std::fmt::Display for NumericLiteralValueValidationErrorKind {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Empty => {
                write!(formatter, "numeric literal should not be empty")
            }
            Self::UnexpectedCharacter {
                kind,
                position,
                value,
            } => {
                write!(
                    formatter,
                    "non-{} character {:?} found at position {}",
                    match kind {
                        NumericLiteralValueKind::FloatingPoint => {
                            "floating point"
                        }
                        NumericLiteralValueKind::Integer => {
                            "integer"
                        }
                    },
                    value,
                    character_position_to_string(*position)
                )
            }
        }
    }
}

impl std::fmt::Display for WhitespaceStringValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl std::fmt::Display for WhitespaceStringValidationErrorKind {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Empty => {
                write!(formatter, "whitespace should not be empty")
            }
            Self::UnexpectedCharacter { position, value } => {
                write!(
                    formatter,
                    "non-whitespace character {:?} found at position {}",
                    value,
                    character_position_to_string(*position)
                )
            }
        }
    }
}

pub(crate) fn validate_comment_block_lines<StringType: AsRef<str>>(
    lines: &[StringType],
    expected_start_character_position: CharacterPosition,
) -> Result<(), Vec<CommentBlockLinesValidationError>> {
    if lines.is_empty() {
        Err(vec![CommentBlockLinesValidationError(
            CommentBlockLinesValidationErrorKind::Empty,
        )])
    } else {
        let mut errors = vec![];
        if !lines[0].as_ref().starts_with(COMMENT_BLOCK_PREFIX) {
            errors.push(CommentBlockLinesValidationError(
                CommentBlockLinesValidationErrorKind::InvalidPrefix,
            ));
        }
        errors.extend(lines.iter().enumerate().filter_map(
            |(line_index, line)| {
                let line = line.as_ref();
                if let Some(raw_byte_index) = line.find(COMMENT_BLOCK_SUFFIX) {
                    let byte_index = ByteCount::from(raw_byte_index);
                    if !((line_index == 0usize
                        && byte_index < COMMENT_BLOCK_PREFIX.byte_size())
                        || (line_index == lines.len() - 1usize
                            && byte_index + COMMENT_BLOCK_SUFFIX.byte_size()
                                == line.byte_size()))
                    {
                        Some(CommentBlockLinesValidationError(
                        CommentBlockLinesValidationErrorKind::PrematureEnd {
                            position: expected_start_character_position
                                .move_by(CharacterPosition {
                                    byte: byte_index,
                                    utf_8: line[..byte_index.into()]
                                        .utf_8_size(),
                                }),
                        },
                    ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
        ));
        if !lines[lines.len() - 1]
            .as_ref()
            .ends_with(COMMENT_BLOCK_SUFFIX)
        {
            errors.push(CommentBlockLinesValidationError(
                CommentBlockLinesValidationErrorKind::InvalidSuffix,
            ));
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

pub(crate) fn validate_floating_point_literal_string<
    StringType: AsRef<str>,
>(
    value: &StringType,
    expected_start_character_position: CharacterPosition,
) -> Result<(), Vec<NumericLiteralValueValidationError>> {
    const KIND: NumericLiteralValueKind =
        NumericLiteralValueKind::FloatingPoint;
    let value = value.as_ref();
    if value.is_empty() {
        return Err(vec![NumericLiteralValueValidationError(
            NumericLiteralValueValidationErrorKind::Empty,
        )]);
    }
    let mut positioned_characters = PositionedCharacters::from(value);
    let mut errors = vec![];
    let mut dot_seen_before = false;
    while let Some((mut non_numeric_position, mut non_numeric_character)) =
        to_next_non_digit_character(&mut positioned_characters)
    {
        if !dot_seen_before && non_numeric_character == '.' {
            if let Some((
                next_non_numeric_position,
                next_non_numeric_character,
            )) = to_next_non_digit_character(&mut positioned_characters)
            {
                (non_numeric_position, non_numeric_character) =
                    (next_non_numeric_position, next_non_numeric_character);
            } else {
                break;
            }
            dot_seen_before = true;
        }
        if non_numeric_character == 'e' || non_numeric_character == 'E' {
            if let Some((next_position, next_character)) =
                positioned_characters.next()
            {
                if next_character == '+' || next_character == '-' {
                    if let Some((next_next_position, next_next_character)) =
                        positioned_characters.next()
                    {
                        if !is_numeric_character(next_next_character) {
                            errors.push(
                                NumericLiteralValueValidationError(
                                    NumericLiteralValueValidationErrorKind::UnexpectedCharacter {
                                        kind: KIND,
                                        position: expected_start_character_position.move_by(
                                            non_numeric_position,
                                        ),
                                        value: non_numeric_character,
                                    }
                                )
                            );
                            errors.push(
                                NumericLiteralValueValidationError(
                                    NumericLiteralValueValidationErrorKind::UnexpectedCharacter {
                                        kind: KIND,
                                        position: expected_start_character_position.move_by(
                                            next_position,
                                        ),
                                        value: next_character,
                                    }
                                )
                            );
                            errors.push(
                                NumericLiteralValueValidationError(
                                    NumericLiteralValueValidationErrorKind::UnexpectedCharacter {
                                        kind: KIND,
                                        position: expected_start_character_position.move_by(
                                            next_next_position,
                                        ),
                                        value: next_next_character,
                                    }
                                )
                            );
                        }
                    } else {
                        errors.push(
                            NumericLiteralValueValidationError(
                                NumericLiteralValueValidationErrorKind::UnexpectedCharacter {
                                    kind: KIND,
                                    position: expected_start_character_position.move_by(
                                        non_numeric_position,
                                    ),
                                    value: non_numeric_character,
                                }
                            )
                        );
                        errors.push(
                            NumericLiteralValueValidationError(
                                NumericLiteralValueValidationErrorKind::UnexpectedCharacter {
                                    kind: KIND,
                                    position: expected_start_character_position.move_by(
                                        next_position,
                                    ),
                                    value: next_character,
                                }
                            )
                        );
                    }
                } else if !is_numeric_character(next_character) {
                    errors.push(
                        NumericLiteralValueValidationError(
                            NumericLiteralValueValidationErrorKind::UnexpectedCharacter {
                                kind: KIND,
                                position: expected_start_character_position.move_by(
                                    non_numeric_position,
                                ),
                                value: non_numeric_character,
                            }
                        )
                    );
                    errors.push(
                        NumericLiteralValueValidationError(
                            NumericLiteralValueValidationErrorKind::UnexpectedCharacter {
                                kind: KIND,
                                position: expected_start_character_position.move_by(
                                    next_position,
                                ),
                                value: next_character,
                            }
                        )
                    );
                }
            } else {
                errors.push(
                    NumericLiteralValueValidationError(
                        NumericLiteralValueValidationErrorKind::UnexpectedCharacter {
                            kind: KIND,
                            position: expected_start_character_position.move_by(
                                non_numeric_position,
                            ),
                            value: non_numeric_character,
                        }
                    )
                );
                return Err(errors);
            }
            break;
        } else {
            errors.push(NumericLiteralValueValidationError(
                NumericLiteralValueValidationErrorKind::UnexpectedCharacter {
                    kind: KIND,
                    position: expected_start_character_position
                        .move_by(non_numeric_position),
                    value: non_numeric_character,
                },
            ));
        }
    }
    errors.extend(positioned_characters.filter_map(
        |(character_position, character)| {
            if is_numeric_character(character) {
                None
            } else {
                Some(NumericLiteralValueValidationError(
                    NumericLiteralValueValidationErrorKind::UnexpectedCharacter {
                        kind: KIND,
                        position: expected_start_character_position.move_by(
                            character_position,
                        ),
                        value: character,
                    }
                ))
            }
        },
    ));
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub(crate) fn validate_comment_line_string<StringType: AsRef<str>>(
    value: &StringType,
) -> Result<(), Vec<CommentLineStringValidationError>> {
    let value = value.as_ref();
    if value.is_empty() {
        Err(vec![CommentLineStringValidationError(
            CommentLineStringValidationErrorKind::Empty,
        )])
    } else {
        let mut errors = vec![];
        if !value.starts_with(COMMENT_LINE_PREFIX) {
            errors.push(CommentLineStringValidationError(
                CommentLineStringValidationErrorKind::InvalidPrefix,
            ));
        }
        if value.ends_with(NEWLINE) {
            errors.push(CommentLineStringValidationError(
                CommentLineStringValidationErrorKind::EndsWithNewline,
            ));
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

pub(crate) fn validate_identifier_string<StringType: AsRef<str>>(
    value: &StringType,
    expected_start_character_position: CharacterPosition,
) -> Result<(), Vec<IdentifierStringValidationError>> {
    let mut positioned_characters = PositionedCharacters::from(value.as_ref());
    if let Some((first_position, first_character)) =
        positioned_characters.next()
    {
        let mut errors = vec![];
        if !is_starting_identifier_character(first_character) {
            errors.push(IdentifierStringValidationError(
                IdentifierStringValidationErrorKind::UnexpectedCharacter {
                    position: expected_start_character_position
                        .move_by(first_position),
                    value: first_character,
                },
            ));
        }
        errors.extend(positioned_characters.filter_map(
            |(character_position, character)| {
                if is_non_starting_identifier_character(character) {
                    None
                } else {
                    Some(IdentifierStringValidationError(
                        IdentifierStringValidationErrorKind::UnexpectedCharacter {
                            position: expected_start_character_position.move_by(
                                character_position,
                            ),
                            value: character,
                        },
                    ))
                }
            },
        ));
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    } else {
        Err(vec![IdentifierStringValidationError(
            IdentifierStringValidationErrorKind::Empty,
        )])
    }
}

pub(crate) fn validate_integer_literal_string<StringType: AsRef<str>>(
    value: &StringType,
    expected_start_character_position: CharacterPosition,
) -> Result<(), Vec<NumericLiteralValueValidationError>> {
    let value = value.as_ref();
    if value.is_empty() {
        return Err(vec![NumericLiteralValueValidationError(
            NumericLiteralValueValidationErrorKind::Empty,
        )]);
    }
    let errors = PositionedCharacters::from(value)
        .filter_map(|(character_position, character)| {
            if is_numeric_character(character) {
                None
            } else {
                Some(NumericLiteralValueValidationError(
                    NumericLiteralValueValidationErrorKind::UnexpectedCharacter {
                        kind: NumericLiteralValueKind::Integer,
                        position: expected_start_character_position.move_by(
                            character_position,
                        ),
                        value: character,
                    }
                ))
            }
        })
        .collect::<Vec<_>>();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub(crate) fn validate_whitespace_string<StringType: AsRef<str>>(
    value: &StringType,
    expected_start_character_position: CharacterPosition,
) -> Result<(), Vec<WhitespaceStringValidationError>> {
    let value = value.as_ref();
    if value.is_empty() {
        return Err(vec![WhitespaceStringValidationError(
            WhitespaceStringValidationErrorKind::Empty,
        )]);
    }
    let errors = PositionedCharacters::from(value)
        .filter_map(|(character_position, character)| {
            if is_non_newline_whitespace(character) {
                None
            } else {
                Some(WhitespaceStringValidationError(
                    WhitespaceStringValidationErrorKind::UnexpectedCharacter {
                        position: expected_start_character_position
                            .move_by(character_position),
                        value: character,
                    },
                ))
            }
        })
        .collect::<Vec<_>>();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn character_position_to_string(
    character_position: CharacterPosition,
) -> String {
    format!(
        "(byte: {}, utf-8: {})",
        usize::from(character_position.byte),
        usize::from(character_position.utf_8)
    )
}

fn to_next_non_digit_character(
    positioned_characters: &mut PositionedCharacters<'_>,
) -> Option<(CharacterPosition, char)> {
    positioned_characters
        .find(|(_, character)| !is_numeric_character(*character))
}
