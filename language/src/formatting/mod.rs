pub(crate) use self::format::{
    format_script, CheckedFormatInSingleLine, FormatInMultipleLines,
};
pub(crate) use self::reset_positions::{
    reset_script_positions, ResetPositions,
};

mod format;
mod reset_positions;
