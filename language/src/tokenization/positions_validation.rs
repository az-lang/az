use super::byte_count::ByteCount;
use super::character_position::CharacterPosition;
use super::utf_8_count::Utf8Count;

#[derive(Debug)]
pub(crate) struct PositionsValidationError(PositionsValidationErrorKind);

type PositionKind = bool;

pub(crate) const END_POSITION_KIND: PositionKind = false;
pub(crate) const START_POSITION_KIND: PositionKind = true;

pub(crate) fn compare_character_positions<
    const POSITION_KIND: PositionKind,
>(
    actual: CharacterPosition,
    expected: CharacterPosition,
) -> Vec<PositionsValidationError> {
    match POSITION_KIND {
        START_POSITION_KIND => compare_character_positions_impl::<
            START_POSITION_KIND,
        >(actual, expected)
        .into_iter()
        .map(|error| {
            PositionsValidationError(PositionsValidationErrorKind::Start(
                error,
            ))
        })
        .collect(),
        END_POSITION_KIND => compare_character_positions_impl::<
            END_POSITION_KIND,
        >(actual, expected)
        .into_iter()
        .map(|error| {
            PositionsValidationError(PositionsValidationErrorKind::End(error))
        })
        .collect(),
    }
}

fn compare_character_positions_impl<const POSITION_KIND: PositionKind>(
    actual: CharacterPosition,
    expected: CharacterPosition,
) -> Vec<PositionValidationError<POSITION_KIND>> {
    let mut errors = vec![];
    if actual.byte != expected.byte {
        errors.push(PositionValidationError(IndexMismatch::Byte {
            actual: actual.byte,
            expected: expected.byte,
        }));
    }
    if actual.utf_8 != expected.utf_8 {
        errors.push(PositionValidationError(IndexMismatch::Utf8 {
            actual: actual.utf_8,
            expected: expected.utf_8,
        }));
    }
    errors
}

#[derive(Debug)]
struct PositionValidationError<const POSITION_KIND: PositionKind>(
    IndexMismatch<POSITION_KIND>,
);

#[derive(Debug)]
enum IndexMismatch<const POSITION_KIND: PositionKind> {
    Byte {
        actual: ByteCount,
        expected: ByteCount,
    },
    Utf8 {
        actual: Utf8Count,
        expected: Utf8Count,
    },
}

#[derive(Debug)]
enum PositionsValidationErrorKind {
    Start(PositionValidationError<START_POSITION_KIND>),
    End(PositionValidationError<END_POSITION_KIND>),
}

impl std::fmt::Display for PositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl<const POSITION_KIND: PositionKind> std::fmt::Display
    for PositionValidationError<POSITION_KIND>
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl<const POSITION_KIND: PositionKind> std::fmt::Display
    for IndexMismatch<POSITION_KIND>
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let position_kind_string = match POSITION_KIND {
            END_POSITION_KIND => "end",
            START_POSITION_KIND => "start",
        };
        match self {
            IndexMismatch::Byte { actual, expected } => {
                write!(
                    formatter,
                    "{} character byte index mismatch: actual {}, expected {}",
                    position_kind_string,
                    usize::from(*actual),
                    usize::from(*expected)
                )
            }
            IndexMismatch::Utf8 { actual, expected } => {
                write!(
                    formatter,
                    "{} character utf-8 index mismatch: actual {}, expected {}",
                    position_kind_string,
                    usize::from(*actual),
                    usize::from(*expected)
                )
            }
        }
    }
}

impl std::fmt::Display for PositionsValidationErrorKind {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            PositionsValidationErrorKind::Start(value) => {
                std::fmt::Display::fmt(value, formatter)
            }
            PositionsValidationErrorKind::End(value) => {
                std::fmt::Display::fmt(value, formatter)
            }
        }
    }
}
