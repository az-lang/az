use crate::formatting::format::line_offset::LineOffset;
use crate::parsing::{FillerContent, Script, Statement};
use crate::tokenization::{
    ByteCount, ByteSize, CharacterPosition, Utf8Count, Utf8Size,
};

use super::checked_format_in_single_line::CheckedFormatInSingleLine;
use super::format_context::FormatContextBuilder;
use super::format_in_multiple_lines::FormatInMultipleLines;
use super::format_local_context::FormatLocalContext;
use super::offset::Offset;

pub(crate) fn format_script<
    StringType: AsRef<str> + ByteSize + Utf8Size + for<'a> From<&'a str>,
>(
    value: &mut Script<StringType>,
) where
    FillerContent<StringType>: ToString,
    Statement<StringType>: CheckedFormatInSingleLine + FormatInMultipleLines,
{
    let global_context = FormatContextBuilder::new().build();
    let mut local_context = FormatLocalContext {
        current_character_position: CharacterPosition {
            byte: ByteCount::default(),
            utf_8: Utf8Count::default(),
        },
    };
    let depth = 0usize;
    match value.statements.as_mut_slice() {
        [] => {
            local_context.format_filler_vec_in_multiple_lines(
                &mut value.fillers,
                &global_context,
                depth,
                Offset::default(),
            );
        }
        [first_statement, rest_statements @ ..] => {
            first_statement
                .checked_format_in_single_line(
                    local_context.clone(),
                    &global_context,
                    Offset::default(),
                )
                .map(|mut replacement| {
                    std::mem::swap(&mut local_context, &mut replacement);
                })
                .unwrap_or_else(|| {
                    first_statement.format_in_multiple_lines(
                        &mut local_context,
                        &global_context,
                        depth,
                        Offset::default(),
                    );
                });
            for statement in rest_statements {
                statement
                    .checked_format_in_single_line(
                        local_context.clone(),
                        &global_context,
                        Offset {
                            line: LineOffset::from(1usize),
                            character: global_context
                                .depth_to_character_offset(depth),
                        },
                    )
                    .map(|mut replacement| {
                        std::mem::swap(&mut local_context, &mut replacement);
                    })
                    .unwrap_or_else(|| {
                        statement.format_in_multiple_lines(
                            &mut local_context,
                            &global_context,
                            depth,
                            Offset {
                                line: LineOffset::from(1usize),
                                character: global_context
                                    .depth_to_character_offset(depth),
                            },
                        );
                    });
            }
            local_context.format_filler_vec_in_multiple_lines(
                &mut value.fillers,
                &global_context,
                depth,
                Offset {
                    line: LineOffset::from(1usize),
                    character: global_context.depth_to_character_offset(depth),
                },
            );
        }
    }
}
