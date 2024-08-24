use crate::parsing::filler_content::FillerContent;
use crate::parsing::keywords::{
    CONDITIONAL_ANTECEDENT_OPENER, FUNCTION_DEFINITION_OPENER,
    WHILE_LOOP_OPENER,
};
use crate::parsing::ReturnOperator;
use crate::tokenization::TokenContent;

use super::annotated_identifier::AnnotatedIdentifier;
use super::assignment::Assignment;
use super::bidirectional_conditional::BidirectionalConditional;
use super::binary_arithmetic_operation::BinaryArithmeticOperation;
use super::binary_comparison::BinaryComparison;
use super::block::Block;
use super::call::Call;
use super::expression::Expression;
use super::function_definition::FunctionDefinition;
use super::function_type::FunctionType;
use super::grouping::Grouping;
use super::identifier::Identifier;
use super::member_access::MemberAccess;
use super::numeric_literal::NumericLiteral;
use super::return_::Return;
use super::tuple::Tuple;
use super::unary_arithmetic_operation::UnaryArithmeticOperation;
use super::unidirectional_conditional::UnidirectionalConditional;
use super::while_loop::WhileLoop;

pub(super) trait ToReducedFirstTokenContent<'a, TokenStringType> {
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType>;
}

impl<'a, StringType: AsRef<str>, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType> for Expression<StringType>
where
    TokenContent<TokenStringType>: From<TokenContent<&'a str>>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        match self {
            Expression::AnnotatedIdentifier(value) => {
                value.to_reduced_first_token_content()
            }
            Expression::Assignment(value) => {
                value.to_reduced_first_token_content()
            }
            Expression::BidirectionalConditional(value) => {
                value.to_reduced_first_token_content()
            }
            Expression::BinaryArithmeticOperation(value) => {
                value.to_reduced_first_token_content()
            }
            Expression::BinaryComparison(value) => {
                value.to_reduced_first_token_content()
            }
            Expression::Block(value) => value.to_reduced_first_token_content(),
            Expression::Call(value) => value.to_reduced_first_token_content(),
            Expression::FunctionDefinition(value) => {
                value.to_reduced_first_token_content()
            }
            Expression::FunctionType(value) => {
                value.to_reduced_first_token_content()
            }
            Expression::Grouping(value) => {
                value.to_reduced_first_token_content()
            }
            Expression::Identifier(value) => {
                value.to_reduced_first_token_content()
            }
            Expression::MemberAccess(value) => {
                value.to_reduced_first_token_content()
            }
            Expression::NumericLiteral(value) => {
                value.to_reduced_first_token_content()
            }
            Expression::Return(value) => {
                value.to_reduced_first_token_content()
            }
            Expression::Tuple(value) => value.to_reduced_first_token_content(),
            Expression::UnaryArithmeticOperation(value) => {
                value.to_reduced_first_token_content()
            }
            Expression::UnidirectionalConditional(value) => {
                value.to_reduced_first_token_content()
            }
            Expression::WhileLoop(value) => {
                value.to_reduced_first_token_content()
            }
        }
    }
}

impl<'a, StringType, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType>
    for AnnotatedIdentifier<StringType>
where
    Identifier<StringType>: ToReducedFirstTokenContent<'a, TokenStringType>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        self.identifier.to_reduced_first_token_content()
    }
}

impl<'a, StringType, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType> for Assignment<StringType>
where
    Expression<StringType>: ToReducedFirstTokenContent<'a, TokenStringType>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        self.target.as_ref().to_reduced_first_token_content()
    }
}

impl<'a, StringType: AsRef<str>, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType>
    for BidirectionalConditional<StringType>
where
    TokenContent<TokenStringType>: From<TokenContent<&'a str>>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        if let Some(first_filler) = self.antecedent_opener_fillers.first() {
            filler_content_to_reduced_token_content(&first_filler.content)
        } else {
            TokenContent::Identifier(CONDITIONAL_ANTECEDENT_OPENER).into()
        }
    }
}

impl<'a, StringType, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType>
    for BinaryArithmeticOperation<StringType>
where
    Expression<StringType>: ToReducedFirstTokenContent<'a, TokenStringType>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        self.left.as_ref().to_reduced_first_token_content()
    }
}

impl<'a, StringType, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType>
    for BinaryComparison<StringType>
where
    Expression<StringType>: ToReducedFirstTokenContent<'a, TokenStringType>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        self.left.as_ref().to_reduced_first_token_content()
    }
}

impl<'a, StringType: AsRef<str>, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType> for Block<StringType>
where
    TokenContent<TokenStringType>: From<TokenContent<&'a str>>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        if let Some(first_filler) = self.open_brace_fillers.first() {
            filler_content_to_reduced_token_content(&first_filler.content)
        } else {
            TokenContent::OpenBrace
        }
    }
}

impl<'a, StringType, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType> for Call<StringType>
where
    Expression<StringType>: ToReducedFirstTokenContent<'a, TokenStringType>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        self.callable.to_reduced_first_token_content()
    }
}

impl<'a, StringType: AsRef<str>, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType>
    for FunctionDefinition<StringType>
where
    TokenContent<TokenStringType>: From<TokenContent<&'a str>>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        if let Some(first_filler) = self.opener_fillers.first() {
            filler_content_to_reduced_token_content(&first_filler.content)
        } else {
            TokenContent::Identifier(FUNCTION_DEFINITION_OPENER).into()
        }
    }
}

impl<'a, StringType: AsRef<str>, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType>
    for FunctionType<StringType>
where
    TokenContent<TokenStringType>: From<TokenContent<&'a str>>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        if let Some(first_filler) = self.open_parenthesis_fillers.first() {
            filler_content_to_reduced_token_content(&first_filler.content)
        } else {
            TokenContent::OpenParenthesis
        }
    }
}

impl<'a, StringType: AsRef<str>, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType> for Grouping<StringType>
where
    TokenContent<TokenStringType>: From<TokenContent<&'a str>>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        if let Some(first_filler) = self.open_parenthesis_fillers.first() {
            filler_content_to_reduced_token_content(&first_filler.content)
        } else {
            TokenContent::OpenParenthesis
        }
    }
}

impl<'a, StringType: AsRef<str>, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType> for Identifier<StringType>
where
    TokenContent<TokenStringType>: From<TokenContent<&'a str>>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        if let Some(first_filler) = self.fillers.first() {
            filler_content_to_reduced_token_content(&first_filler.content)
        } else {
            TokenContent::Identifier(self.string.as_ref()).into()
        }
    }
}

impl<'a, StringType, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType>
    for MemberAccess<StringType>
where
    Expression<StringType>: ToReducedFirstTokenContent<'a, TokenStringType>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        self.object.to_reduced_first_token_content()
    }
}

impl<'a, StringType: AsRef<str>, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType>
    for NumericLiteral<StringType>
where
    TokenContent<TokenStringType>: From<TokenContent<&'a str>>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        if let Some(first_filler) = self.fillers.first() {
            filler_content_to_reduced_token_content(&first_filler.content)
        } else {
            TokenContent::NumericLiteral {
                value: self.value.as_ref(),
                type_: self.type_,
            }
            .into()
        }
    }
}

impl<'a, StringType: AsRef<str>, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType> for Tuple<StringType>
where
    TokenContent<TokenStringType>: From<TokenContent<&'a str>>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        if let Some(first_filler) = self.open_parenthesis_fillers.first() {
            filler_content_to_reduced_token_content(&first_filler.content)
        } else {
            TokenContent::OpenParenthesis
        }
    }
}

impl<'a, StringType: AsRef<str>, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType> for Return<StringType>
where
    TokenContent<TokenStringType>: From<TokenContent<&'a str>>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        if let Some(first_filler) = self.operator_fillers.first() {
            filler_content_to_reduced_token_content(&first_filler.content)
        } else {
            ReturnOperator.into_const_token_content().into()
        }
    }
}

impl<'a, StringType: AsRef<str>, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType>
    for UnaryArithmeticOperation<StringType>
where
    TokenContent<TokenStringType>: From<TokenContent<&'a str>>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        if let Some(first_filler) = self.operator_fillers.first() {
            filler_content_to_reduced_token_content(&first_filler.content)
        } else {
            self.operator.into_const_token_content().into()
        }
    }
}

impl<'a, StringType: AsRef<str>, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType>
    for UnidirectionalConditional<StringType>
where
    TokenContent<TokenStringType>: From<TokenContent<&'a str>>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        if let Some(first_filler) = self.opener_fillers.first() {
            filler_content_to_reduced_token_content(&first_filler.content)
        } else {
            TokenContent::Identifier(CONDITIONAL_ANTECEDENT_OPENER).into()
        }
    }
}

impl<'a, StringType: AsRef<str>, TokenStringType>
    ToReducedFirstTokenContent<'a, TokenStringType> for WhileLoop<StringType>
where
    TokenContent<TokenStringType>: From<TokenContent<&'a str>>,
{
    fn to_reduced_first_token_content(
        &'a self,
    ) -> TokenContent<TokenStringType> {
        if let Some(first_filler) = self.opener_fillers.first() {
            filler_content_to_reduced_token_content(&first_filler.content)
        } else {
            TokenContent::Identifier(WHILE_LOOP_OPENER).into()
        }
    }
}

fn filler_content_to_reduced_token_content<
    'a,
    StringType: AsRef<str>,
    TokenStringType,
>(
    value: &'a FillerContent<StringType>,
) -> TokenContent<TokenStringType>
where
    TokenContent<TokenStringType>: From<TokenContent<&'a str>>,
{
    match value {
        FillerContent::CommentBlock(_) => {
            TokenContent::CommentBlock("").into()
        }
        FillerContent::CommentLine(_) => TokenContent::CommentLine("").into(),
        FillerContent::Newline => TokenContent::Newline,
        FillerContent::Whitespace(value) => {
            TokenContent::Whitespace(value.as_ref()).into()
        }
    }
}
