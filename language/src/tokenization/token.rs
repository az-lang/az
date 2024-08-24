use super::byte_size::ByteSize;
use super::character_position::CharacterPosition;
use super::positions_validation::{
    compare_character_positions, PositionsValidationError, END_POSITION_KIND,
    START_POSITION_KIND,
};
use super::substring_position::SubstringPosition;
use super::token_content::{TokenContent, TokenContentValidationError};
use super::utf_8_size::Utf8Size;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Token<StringType> {
    pub content: TokenContent<StringType>,
    pub position: SubstringPosition,
}

impl<StringType: AsRef<str>> Token<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(crate) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<TokenContentsValidationError>> {
        self.content
            .validate_impl(self.position.start)
            .map_err(|errors| {
                errors
                    .into_iter()
                    .map(TokenContentsValidationError)
                    .collect()
            })
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> Token<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.position.start)
    }

    pub(crate) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> Result<(), Vec<TokenPositionsValidationError>> {
        let mut errors = compare_character_positions::<START_POSITION_KIND>(
            self.position.start,
            expected_start_character_position,
        )
        .into_iter()
        .map(TokenPositionsValidationError)
        .collect::<Vec<_>>();
        errors.extend(
            compare_character_positions::<END_POSITION_KIND>(
                self.position.end,
                token_content_to_expected_end_character_position(
                    &self.content,
                    expected_start_character_position,
                ),
            )
            .into_iter()
            .map(TokenPositionsValidationError),
        );
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug)]
pub(crate) struct TokenContentsValidationError(TokenContentValidationError);

#[derive(Debug)]
pub(crate) struct TokenPositionsValidationError(PositionsValidationError);

pub(super) fn token_content_to_expected_end_character_position<
    StringType: AsRef<str> + ByteSize + Utf8Size,
>(
    content: &TokenContent<StringType>,
    expected_start_character_position: CharacterPosition,
) -> CharacterPosition {
    expected_start_character_position.move_by(content.to_character_count())
}

impl std::fmt::Display for TokenContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl std::fmt::Display for TokenPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl std::error::Error for TokenContentsValidationError {}
impl std::error::Error for TokenPositionsValidationError {}
