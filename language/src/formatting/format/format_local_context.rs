use crate::parsing::{Filler, FillerContent};
use crate::tokenization::constants::NEWLINE;
use crate::tokenization::{
    ByteCount, ByteSize, CharacterPosition, SubstringPosition, Utf8Count,
    Utf8Size,
};

use super::character_offset::CharacterOffset;
use super::format_context::FormatContext;
use super::offset::Offset;

#[derive(Clone, Debug)]
pub struct FormatLocalContext {
    pub(super) current_character_position: CharacterPosition,
}

impl FormatLocalContext {
    pub(super) fn checked_format_filler_vec_in_single_line<
        StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>,
    >(
        mut self,
        value: &mut Vec<Filler<StringType>>,
        global_context: &FormatContext,
        offset: Offset,
    ) -> Option<Self>
    where
        FillerContent<StringType>: ToString,
    {
        self = self.checked_fit_fillers_in_single_line(value, offset)?;
        for filler in value {
            self.reset_filler_position(filler);
            self.check(global_context)?;
        }
        Some(self)
    }

    pub(super) fn checked_format_non_empty_string_without_newline_in_single_line<
        StringType: ?Sized + AsRef<str> + ByteSize + Utf8Size,
    >(
        mut self,
        string: &StringType,
        substring_position: &mut SubstringPosition,
        global_context: &FormatContext,
    ) -> Option<Self> {
        self.reset_non_empty_string_position(string, substring_position);
        self.check(global_context)?;
        Some(self)
    }

    pub(super) fn format_filler_vec_in_multiple_lines<
        StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>,
    >(
        &mut self,
        value: &mut Vec<Filler<StringType>>,
        global_context: &FormatContext,
        depth: usize,
        offset: Offset,
    ) where
        FillerContent<StringType>: ToString,
    {
        value.retain(Filler::is_comment);
        let newlines_count = usize::from(offset.line);
        let mut replacement = Vec::with_capacity(
            value.len()
                + (newlines_count
                    + if offset.character == CharacterOffset::default() {
                        0usize
                    } else {
                        1usize
                    })
                    * (value.len() + 1usize),
        );
        std::mem::swap(value, &mut replacement);
        for _ in 0usize..newlines_count {
            value.push(Filler {
                content: FillerContent::Newline,
                position: default_substring_position(),
            });
        }
        if offset.character > CharacterOffset::default() {
            value.push(Filler {
                content: character_offset_to_whitespace_filler_content(
                    offset.character,
                ),
                position: default_substring_position(),
            });
        }
        for filler in replacement {
            value.push(filler);
            value.push(Filler {
                content: FillerContent::Newline,
                position: default_substring_position(),
            });
            if depth > 0usize {
                value.push(Filler {
                    content: character_offset_to_whitespace_filler_content(
                        global_context.depth_to_character_offset(depth),
                    ),
                    position: default_substring_position(),
                });
            }
        }
        for filler in value {
            self.reset_filler_position(filler);
        }
    }

    pub(super) fn reset_non_empty_string_position<
        StringType: ?Sized + AsRef<str> + ByteSize + Utf8Size,
    >(
        &mut self,
        string: &StringType,
        substring_position: &mut SubstringPosition,
    ) {
        debug_assert!(!string.as_ref().is_empty());
        substring_position.start = self.current_character_position;
        self.current_character_position.byte += string.byte_size();
        self.current_character_position.utf_8 += string.utf_8_size();
        substring_position.end = self.current_character_position;
    }

    fn check(&mut self, global_context: &FormatContext) -> Option<()> {
        if self.current_character_position.utf_8
            > global_context.max_line_utf_8_size()
        {
            None
        } else {
            Some(())
        }
    }

    fn checked_fit_fillers_in_single_line<
        StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>,
    >(
        self,
        value: &mut Vec<Filler<StringType>>,
        offset: Offset,
    ) -> Option<Self>
    where
        FillerContent<StringType>: ToString,
    {
        for filler in value.iter() {
            match &filler.content {
                FillerContent::CommentBlock(value) => {
                    if value.as_ref().contains(NEWLINE) {
                        return None;
                    }
                }
                FillerContent::CommentLine(_) => {
                    return None;
                }
                FillerContent::Newline | FillerContent::Whitespace(_) => {}
            }
        }
        value.retain(|filler| {
            matches!(filler.content, FillerContent::CommentBlock(_))
        });
        let newlines_count = usize::from(offset.line);
        let mut replacement =
            Vec::with_capacity(newlines_count + value.len() * 2usize + 1usize);
        std::mem::swap(value, &mut replacement);
        for _ in 0usize..newlines_count {
            value.push(Filler {
                content: FillerContent::Newline,
                position: default_substring_position(),
            });
        }
        if offset.character > CharacterOffset::default() {
            value.push(Filler {
                content: character_offset_to_whitespace_filler_content(
                    offset.character,
                ),
                position: default_substring_position(),
            })
        };
        for filler in replacement {
            value.push(filler);
            value.push(Filler {
                content: character_offset_to_whitespace_filler_content(
                    CharacterOffset::from(1usize),
                ),
                position: default_substring_position(),
            });
        }
        Some(self)
    }

    fn reset_filler_position<StringType: AsRef<str> + ByteSize + Utf8Size>(
        &mut self,
        filler: &mut Filler<StringType>,
    ) where
        FillerContent<StringType>: ToString,
    {
        filler.position.start = self.current_character_position;
        match &filler.content {
            FillerContent::CommentBlock(value) => {
                self.current_character_position.byte += value.byte_size();
                self.current_character_position.utf_8 += value.utf_8_size();
                filler.position.end = self.current_character_position;
            }
            FillerContent::CommentLine(value) => {
                self.current_character_position.byte += value.byte_size();
                self.current_character_position.utf_8 += value.utf_8_size();
                filler.position.end = self.current_character_position;
            }
            FillerContent::Newline => {
                let filler_content_string = filler.content.to_string();
                self.current_character_position.byte +=
                    filler_content_string.byte_size();
                self.current_character_position.utf_8 +=
                    filler_content_string.utf_8_size();
                filler.position.end = self.current_character_position;
            }
            FillerContent::Whitespace(value) => {
                self.current_character_position.byte += value.byte_size();
                self.current_character_position.utf_8 += value.utf_8_size();
                filler.position.end = self.current_character_position;
            }
        }
    }
}

fn default_substring_position() -> SubstringPosition {
    SubstringPosition {
        start: CharacterPosition {
            byte: ByteCount::default(),
            utf_8: Utf8Count::default(),
        },
        end: CharacterPosition {
            byte: ByteCount::default(),
            utf_8: Utf8Count::default(),
        },
    }
}

fn character_offset_to_whitespace_filler_content<
    StringType: for<'a> From<&'a str>,
>(
    value: CharacterOffset,
) -> FillerContent<StringType> {
    repeated_whitespace_filler_content(value.into())
}

fn repeated_whitespace_filler_content<StringType: for<'a> From<&'a str>>(
    value: usize,
) -> FillerContent<StringType> {
    debug_assert!(value > 0usize);
    FillerContent::Whitespace(StringType::from(&" ".repeat(value)))
}
