use crate::tokenization::{
    compare_character_positions, ByteSize, CharacterPosition,
    PositionsValidationError, SubstringPosition, Utf8Size, END_POSITION_KIND,
    START_POSITION_KIND,
};

pub(crate) struct PositionsValidationStep<Error> {
    pub(crate) errors: Vec<Error>,
    pub(crate) expected_end_character_position: CharacterPosition,
}

impl PositionsValidationStep<PositionsValidationError> {
    pub(crate) fn from_string<StringType: ?Sized + ByteSize + Utf8Size>(
        string: &StringType,
        actual_substring_position: &SubstringPosition,
        expected_start_character_position: CharacterPosition,
    ) -> Self {
        let mut errors = compare_character_positions::<START_POSITION_KIND>(
            actual_substring_position.start,
            expected_start_character_position,
        );
        let expected_end_character_position = CharacterPosition {
            byte: expected_start_character_position.byte + string.byte_size(),
            utf_8: expected_start_character_position.utf_8
                + string.utf_8_size(),
        };
        errors.append(&mut compare_character_positions::<END_POSITION_KIND>(
            actual_substring_position.end,
            expected_end_character_position,
        ));
        PositionsValidationStep {
            expected_end_character_position,
            errors,
        }
    }
}

impl<Error> PositionsValidationStep<Error> {
    pub(crate) fn into_result(self) -> Result<(), Vec<Error>> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }

    pub(crate) fn map_error<NextError>(
        self,
        operator: impl FnMut(Error) -> NextError,
    ) -> PositionsValidationStep<NextError> {
        PositionsValidationStep::<NextError> {
            expected_end_character_position: self
                .expected_end_character_position,
            errors: self.errors.into_iter().map(operator).collect(),
        }
    }

    pub(crate) fn merge_with(
        self,
        to_next: impl FnOnce(CharacterPosition) -> PositionsValidationStep<Error>,
    ) -> PositionsValidationStep<Error> {
        let mut next = to_next(self.expected_end_character_position);
        PositionsValidationStep {
            expected_end_character_position: next
                .expected_end_character_position,
            errors: {
                let mut errors = self.errors;
                errors.append(&mut next.errors);
                errors
            },
        }
    }
}
