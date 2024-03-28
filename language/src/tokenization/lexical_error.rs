use super::numeric_literal_value_kind::NumericLiteralValueKind;
use super::substring_position::SubstringPosition;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct CommentBlockIncomplete<StringType> {
    pub position: SubstringPosition,
    pub strings: Vec<StringType>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct IdentifierIncomplete<StringType> {
    pub position: SubstringPosition,
    pub string: StringType,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct IdentifierUnexpectedCharacter<StringType> {
    pub character: char,
    pub expected: StringType,
    pub position: SubstringPosition,
    pub string: StringType,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct NumericLiteralTypeSuffixIncomplete<StringType> {
    pub position: SubstringPosition,
    pub string: StringType,
    pub value: StringType,
    pub value_kind: NumericLiteralValueKind,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct NumericLiteralTypeSuffixUnexpectedCharacter<StringType> {
    pub character: char,
    pub expected: StringType,
    pub position: SubstringPosition,
    pub string: StringType,
    pub value: StringType,
    pub value_kind: NumericLiteralValueKind,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct NumericLiteralTypeSuffixUnknown<StringType> {
    pub position: SubstringPosition,
    pub type_suffix: StringType,
    pub string: StringType,
    pub value: StringType,
    pub value_kind: NumericLiteralValueKind,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct NumericLiteralValueIncomplete<StringType> {
    pub kind: NumericLiteralValueKind,
    pub position: SubstringPosition,
    pub string: StringType,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct NumericLiteralValueTypeSuffixConflict<StringType> {
    pub position: SubstringPosition,
    pub type_suffix: StringType,
    pub string: StringType,
    pub value: StringType,
    pub value_kind: NumericLiteralValueKind,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct NumericLiteralValueUnexpectedCharacter<StringType> {
    pub character: char,
    pub expected: StringType,
    pub kind: NumericLiteralValueKind,
    pub position: SubstringPosition,
    pub string: StringType,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct UnexpectedCharacter<StringType> {
    pub character: char,
    pub position: SubstringPosition,
    pub string: StringType,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum LexicalError<StringType> {
    CommentBlockIncomplete(CommentBlockIncomplete<StringType>),
    IdentifierIncomplete(IdentifierIncomplete<StringType>),
    IdentifierUnexpectedCharacter(IdentifierUnexpectedCharacter<StringType>),
    NumericLiteralTypeSuffixIncomplete(
        NumericLiteralTypeSuffixIncomplete<StringType>,
    ),
    NumericLiteralTypeSuffixUnexpectedCharacter(
        NumericLiteralTypeSuffixUnexpectedCharacter<StringType>,
    ),
    NumericLiteralTypeSuffixUnknown(
        NumericLiteralTypeSuffixUnknown<StringType>,
    ),
    NumericLiteralValueIncomplete(NumericLiteralValueIncomplete<StringType>),
    NumericLiteralValueTypeSuffixConflict(
        NumericLiteralValueTypeSuffixConflict<StringType>,
    ),
    NumericLiteralValueUnexpectedCharacter(
        NumericLiteralValueUnexpectedCharacter<StringType>,
    ),
    UnexpectedCharacter(UnexpectedCharacter<StringType>),
}
