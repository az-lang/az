use std::num::NonZeroUsize;

use crate::tokenization::Utf8Count;

use super::character_offset::CharacterOffset;

pub struct FormatContextBuilder {
    indent_increment: Option<NonZeroUsize>,
    max_line_utf_8_size: Option<Utf8Count>,
}

impl FormatContextBuilder {
    pub fn new() -> Self {
        Self {
            indent_increment: None,
            max_line_utf_8_size: None,
        }
    }

    pub fn build(self) -> FormatContext {
        FormatContext {
            indent_increment: self.indent_increment.unwrap_or_else(|| {
                const DEFAULT_INDENT_SIZE: NonZeroUsize =
                    unsafe { NonZeroUsize::new_unchecked(2usize) };
                DEFAULT_INDENT_SIZE
            }),
            max_line_utf_8_size: self
                .max_line_utf_8_size
                .unwrap_or_else(|| Utf8Count::from(79usize)),
        }
    }
}

pub struct FormatContext {
    indent_increment: NonZeroUsize,
    max_line_utf_8_size: Utf8Count,
}

impl FormatContext {
    pub(super) fn depth_to_character_offset(
        &self,
        depth: usize,
    ) -> CharacterOffset {
        CharacterOffset::from(depth * self.indent_increment.get())
    }

    pub(super) fn max_line_utf_8_size(&self) -> Utf8Count {
        self.max_line_utf_8_size
    }
}
