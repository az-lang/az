use std::rc::Rc;
use std::sync::Arc;

use crate::tokenization::{
    compare_character_positions, ByteSize, CharacterPosition,
    CommentBlockLinesValidationError, CommentLineStringValidationError,
    PositionsValidationError, SubstringPosition, Token, TokenContent,
    Utf8Size, WhitespaceStringValidationError, END_POSITION_KIND,
    START_POSITION_KIND,
};

use super::filler_content::{FillerContent, FillerContentValidationError};
use super::positions_validation_step::PositionsValidationStep;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Filler<StringType> {
    pub content: FillerContent<StringType>,
    pub position: SubstringPosition,
}

impl<StringType: AsRef<str>> Filler<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<FillerContentsValidationError>> {
        self.content.validate_impl(self.position.start).map_err(|errors| {
            errors
                .into_iter()
                .map(|error| match error {
                    FillerContentValidationError::CommentBlock(error) => {
                        FillerContentsValidationError(
                            FillerContentsValidationErrorKind::CommentBlock(error)
                        )
                    }
                    FillerContentValidationError::CommentLine(error) => {
                        FillerContentsValidationError(
                            FillerContentsValidationErrorKind::CommentLine(error)
                        )
                    }
                    FillerContentValidationError::Whitespace(error) => {
                        FillerContentsValidationError(
                            FillerContentsValidationErrorKind::Whitespace(error)
                        )
                    }
                })
                .collect()
        })
    }
}

impl<StringType> Filler<StringType> {
    pub(crate) fn is_comment(&self) -> bool {
        matches!(
            self.content,
            FillerContent::CommentBlock(_) | FillerContent::CommentLine(_)
        )
    }

    pub(crate) fn is_non_comment(&self) -> bool {
        matches!(
            self.content,
            FillerContent::Newline | FillerContent::Whitespace(_)
        )
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> Filler<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.position.start)
    }

    fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> Result<(), Vec<FillerPositionsValidationError>> {
        let mut errors = vec![];
        errors.extend(
            compare_character_positions::<START_POSITION_KIND>(
                self.position.start,
                expected_start_character_position,
            )
            .into_iter()
            .map(FillerPositionsValidationError),
        );
        errors.extend(
            compare_character_positions::<END_POSITION_KIND>(
                self.position.end,
                filler_content_to_expected_end_character_position(
                    &self.content,
                    expected_start_character_position,
                ),
            )
            .into_iter()
            .map(FillerPositionsValidationError),
        );
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

pub(crate) type FillerVec<StringType> = Vec<Filler<StringType>>;

#[derive(Debug)]
pub(crate) struct FillerVecContentsValidationError(
    FillerVecContentsValidationErrorKind,
);

#[derive(Debug)]
pub(crate) struct FillerVecPositionsValidationError {
    error: FillerPositionsValidationError,
    index: usize,
}

type FillerVecPosition = bool;
pub(crate) const NON_TERMINAL_FILLER_VEC_POSITION: FillerVecPosition = false;
pub(crate) const TERMINAL_FILLER_VEC_POSITION: FillerVecPosition = true;

pub(crate) fn validate_filler_vec_contents<
    const POSITION: FillerVecPosition,
    StringType: AsRef<str>,
>(
    filler_vec: &FillerVec<StringType>,
) -> Result<(), Vec<FillerVecContentsValidationError>> {
    let mut errors = vec![];
    let mut enumerated_filler_iterator = filler_vec.iter().enumerate();
    while let Some((index, filler)) = enumerated_filler_iterator.next() {
        if matches!(filler.content, FillerContent::Whitespace(_)) {
            let start = index;
            let mut end = index;
            for (next_index, next_filler) in
                enumerated_filler_iterator.by_ref()
            {
                if matches!(next_filler.content, FillerContent::Whitespace(_))
                {
                    end = next_index;
                } else {
                    break;
                }
            }
            if start != end {
                errors.push(FillerVecContentsValidationError(
                    FillerVecContentsValidationErrorKind::Slice(
                        FillerSliceContentsValidationError(
                            FillerSliceContentsValidationErrorKind::UnmergedWhitespace {
                                start,
                                end,
                            }
                        )
                    )
                ));
            }
        }
    }
    if filler_vec.len()
        > match POSITION {
            NON_TERMINAL_FILLER_VEC_POSITION => 0usize,
            TERMINAL_FILLER_VEC_POSITION => 1usize,
        }
    {
        for (index, (filler, next_filler)) in filler_vec
            [..filler_vec.len() - 1usize]
            .iter()
            .zip(&filler_vec[1usize..])
            .enumerate()
        {
            if matches!(filler.content, FillerContent::CommentLine(_))
                && !matches!(next_filler.content, FillerContent::Newline)
            {
                errors.push(FillerVecContentsValidationError(
                            FillerVecContentsValidationErrorKind::Slice(
                                FillerSliceContentsValidationError(
                                    FillerSliceContentsValidationErrorKind::CommentLineNotFollowedByNewline {
                                        index,
                                    }
                                )
                            )
                        ));
            }
        }
        // we are using match to get compilation error in case of non-exhaustive check
        #[allow(clippy::single_match)]
        match POSITION {
            NON_TERMINAL_FILLER_VEC_POSITION => {
                if matches!(
                    filler_vec[filler_vec.len() - 1usize].content,
                    FillerContent::CommentLine(_)
                ) {
                    errors.push(FillerVecContentsValidationError(
                    FillerVecContentsValidationErrorKind::Slice(
                        FillerSliceContentsValidationError(
                            FillerSliceContentsValidationErrorKind::CommentLineNotFollowedByNewline {
                                index: filler_vec.len() - 1usize,
                            }
                        )
                    )
                ));
                }
            }
            TERMINAL_FILLER_VEC_POSITION => {}
        }
    }
    for (index, filler) in filler_vec.iter().enumerate() {
        if let Err(filler_errors) = filler.validate_contents_impl() {
            errors.extend(filler_errors.into_iter().map(|error| {
                FillerVecContentsValidationError(
                    FillerVecContentsValidationErrorKind::Element(
                        FillerVecElementContentsValidationError {
                            error,
                            index,
                        },
                    ),
                )
            }));
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub(crate) fn validate_filler_vec_positions<
    StringType: AsRef<str> + ByteSize + Utf8Size,
>(
    filler_vec: &FillerVec<StringType>,
    expected_start_character_position: CharacterPosition,
) -> PositionsValidationStep<FillerVecPositionsValidationError> {
    let mut errors = vec![];
    let mut filler_expected_start_byte_index =
        expected_start_character_position.byte;
    let mut filler_expected_start_utf_8_index =
        expected_start_character_position.utf_8;
    for (index, filler) in filler_vec.iter().enumerate() {
        if let Err(filler_errors) =
            filler.validate_positions_impl(CharacterPosition {
                byte: filler_expected_start_byte_index,
                utf_8: filler_expected_start_utf_8_index,
            })
        {
            errors.extend(filler_errors.into_iter().map(|error| {
                FillerVecPositionsValidationError { error, index }
            }));
        }
        CharacterPosition {
            byte: filler_expected_start_byte_index,
            utf_8: filler_expected_start_utf_8_index,
        } = filler_content_to_expected_end_character_position(
            &filler.content,
            CharacterPosition {
                byte: filler_expected_start_byte_index,
                utf_8: filler_expected_start_utf_8_index,
            },
        );
    }
    PositionsValidationStep {
        expected_end_character_position: CharacterPosition {
            byte: filler_expected_start_byte_index,
            utf_8: filler_expected_start_utf_8_index,
        },
        errors,
    }
}

fn filler_content_to_expected_end_character_position<
    StringType: AsRef<str> + ByteSize + Utf8Size,
>(
    content: &FillerContent<StringType>,
    expected_start_character_position: CharacterPosition,
) -> CharacterPosition {
    expected_start_character_position.move_by(content.to_character_count())
}

#[derive(Debug)]
struct FillerContentsValidationError(FillerContentsValidationErrorKind);

#[derive(Debug)]
enum FillerContentsValidationErrorKind {
    CommentBlock(CommentBlockLinesValidationError),
    CommentLine(CommentLineStringValidationError),
    Whitespace(WhitespaceStringValidationError),
}

#[derive(Debug)]
struct FillerPositionsValidationError(PositionsValidationError);

#[derive(Debug)]
struct FillerSliceContentsValidationError(
    FillerSliceContentsValidationErrorKind,
);

#[derive(Debug)]
enum FillerSliceContentsValidationErrorKind {
    CommentLineNotFollowedByNewline { index: usize },
    UnmergedWhitespace { start: usize, end: usize },
}

#[derive(Debug)]
enum FillerVecContentsValidationErrorKind {
    Element(FillerVecElementContentsValidationError),
    Slice(FillerSliceContentsValidationError),
}

#[derive(Debug)]
struct FillerVecElementContentsValidationError {
    error: FillerContentsValidationError,
    index: usize,
}

impl std::fmt::Display for FillerContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl std::fmt::Display for FillerContentsValidationErrorKind {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            FillerContentsValidationErrorKind::CommentBlock(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FillerContentsValidationErrorKind::CommentLine(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FillerContentsValidationErrorKind::Whitespace(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for FillerSliceContentsValidationErrorKind {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            FillerSliceContentsValidationErrorKind::CommentLineNotFollowedByNewline { index } => {
                write!(
                    formatter,
                    "invalid filler contents with index {}: comment line is not followed by newline",
                    index
                )
            }
            FillerSliceContentsValidationErrorKind::UnmergedWhitespace {
                start,
                end,
            } => write!(
                formatter,
                "invalid filler contents with indices from {} to {}: unmerged whitespace",
                start, end
            ),
        }
    }
}

impl std::fmt::Display for FillerSliceContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl std::fmt::Display for FillerVecContentsValidationErrorKind {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            FillerVecContentsValidationErrorKind::Element(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FillerVecContentsValidationErrorKind::Slice(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for FillerVecContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl std::fmt::Display for FillerVecElementContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            formatter,
            "invalid filler contents with index {}: {}",
            self.index, self.error
        )
    }
}

impl std::fmt::Display for FillerPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl std::fmt::Display for FillerVecPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            formatter,
            "invalid filler positions with index {}: {}",
            self.index, self.error
        )
    }
}

impl std::error::Error for FillerContentsValidationError {}
impl std::error::Error for FillerPositionsValidationError {}

impl<StringType, TokenStringType> From<Filler<StringType>>
    for Token<TokenStringType>
where
    FillerContent<StringType>: Into<TokenContent<TokenStringType>>,
{
    fn from(value: Filler<StringType>) -> Self {
        Token {
            content: value.content.into(),
            position: value.position,
        }
    }
}

macro_rules! impl_filler_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Filler<$source_string_type>>
            for Filler<$target_string_type>
        {
            fn from(value: Filler<$source_string_type>) -> Self {
                Filler {
                    content: value.content.into(),
                    position: value.position,
                }
            }
        }
    };
}

impl_filler_string_type_conversion!(&str, Arc<str>);
impl_filler_string_type_conversion!(&str, Box<str>);
impl_filler_string_type_conversion!(&str, Rc<str>);
impl_filler_string_type_conversion!(&str, String);
impl_filler_string_type_conversion!(Box<str>, Arc<str>);
impl_filler_string_type_conversion!(Box<str>, Rc<str>);
impl_filler_string_type_conversion!(Box<str>, String);
impl_filler_string_type_conversion!(String, Arc<str>);
impl_filler_string_type_conversion!(String, Box<str>);
impl_filler_string_type_conversion!(String, Rc<str>);
