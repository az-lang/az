use super::constants::{
    NEWLINE, NON_STARTING_IDENTIFIER_CHARACTERS, NUMERIC_CHARACTERS,
    STARTING_IDENTIFIER_CHARACTERS,
};

pub(super) fn is_non_newline_whitespace(character: char) -> bool {
    character != NEWLINE && character.is_whitespace()
}

pub(super) fn is_non_starting_identifier_character(candidate: char) -> bool {
    NON_STARTING_IDENTIFIER_CHARACTERS.contains(candidate)
}

pub(super) fn is_starting_identifier_character(character: char) -> bool {
    STARTING_IDENTIFIER_CHARACTERS.contains(character)
}

pub(super) fn is_numeric_character(character: char) -> bool {
    NUMERIC_CHARACTERS.contains(character)
}
