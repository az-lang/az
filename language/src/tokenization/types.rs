#[derive(Debug, Eq, PartialEq)]
pub enum LexicalError<'a> {
    CommentBlockIncomplete {
        strings: Vec<&'a str>,
        position: SubstringPosition,
    },
    IdentifierIncomplete {
        position: SubstringPosition,
        string: &'a str,
    },
    IdentifierUnexpectedCharacter {
        character: char,
        expected: &'static str,
        position: SubstringPosition,
        string: &'a str,
    },
    NumericLiteralTypeSuffixIncomplete {
        position: SubstringPosition,
        string: &'a str,
        value: &'a str,
        value_kind: NumericLiteralValueKind,
    },
    NumericLiteralTypeSuffixUnexpectedCharacter {
        character: char,
        expected: &'static str,
        position: SubstringPosition,
        string: &'a str,
        value: &'a str,
        value_kind: NumericLiteralValueKind,
    },
    NumericLiteralTypeSuffixUnknown {
        position: SubstringPosition,
        type_suffix: &'a str,
        string: &'a str,
        value: &'a str,
        value_kind: NumericLiteralValueKind,
    },
    NumericLiteralValueIncomplete {
        kind: NumericLiteralValueKind,
        position: SubstringPosition,
        string: &'a str,
    },
    NumericLiteralValueTypeSuffixConflict {
        position: SubstringPosition,
        type_suffix: &'a str,
        string: &'a str,
        value: &'a str,
        value_kind: NumericLiteralValueKind,
    },
    NumericLiteralValueUnexpectedCharacter {
        character: char,
        expected: &'static str,
        kind: NumericLiteralValueKind,
        position: SubstringPosition,
        string: &'a str,
    },
    UnexpectedCharacter {
        character: char,
        position: SubstringPosition,
        string: &'a str,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub enum NumericLiteralValueKind {
    FloatingPoint,
    Integer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharacterPosition {
    pub byte: usize,
    pub utf_8: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SubstringPosition {
    pub start_line: usize,
    pub start_character: CharacterPosition,
    pub end_line: usize,
    pub end_character: CharacterPosition,
}
