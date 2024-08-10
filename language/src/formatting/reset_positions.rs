use crate::parsing::{
    AnnotatedIdentifier, AnnotationOperator, Assignment, AssignmentOperator,
    BidirectionalConditional, BinaryArithmeticOperation, BinaryComparison,
    Block, Call, Expression, ExpressionStatement, Filler, FillerContent,
    FunctionDefinition, FunctionType, FunctionTypeOperator, Grouping,
    Identifier, MemberAccess, MemberAccessOperator, NumericLiteral, Return,
    ReturnOperator, Script, Statement, Tuple, UnaryArithmeticOperation,
    UnidirectionalConditional, WhileLoop, CONDITIONAL_ALTERNATIVE_OPENER,
    CONDITIONAL_ANTECEDENT_OPENER, FUNCTION_DEFINITION_OPENER,
    WHILE_LOOP_OPENER,
};
use crate::tokenization::{
    ByteCount, ByteSize, CharacterPosition, SubstringPosition, TokenContent,
    Utf8Count, Utf8Size,
};

pub(crate) fn reset_script_positions<
    StringType: AsRef<str> + ByteSize + Utf8Size,
>(
    value: &mut Script<StringType>,
) where
    FillerContent<StringType>: ToString,
    Statement<StringType>: ResetPositions,
{
    let mut context = ResetPositionsContext {
        current_character_position: CharacterPosition {
            byte: ByteCount::default(),
            utf_8: Utf8Count::default(),
        },
    };
    for statement in &mut value.statements {
        statement.reset_positions(&mut context);
    }
    context.process_filler_vec(&mut value.fillers);
}

#[derive(Clone)]
pub struct ResetPositionsContext {
    current_character_position: CharacterPosition,
}

impl ResetPositionsContext {
    fn process_filler_vec<StringType: AsRef<str> + ByteSize + Utf8Size>(
        &mut self,
        value: &mut Vec<Filler<StringType>>,
    ) where
        FillerContent<StringType>: ToString,
    {
        for filler in value {
            filler.position.start = self.current_character_position;
            self.current_character_position = self
                .current_character_position
                .move_by(filler.content.to_character_count());
            filler.position.end = self.current_character_position;
        }
    }

    fn process_non_empty_string<
        StringType: AsRef<str> + ?Sized + AsRef<str> + ByteSize + Utf8Size,
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
}

pub trait ResetPositions {
    fn reset_positions(&mut self, context: &mut ResetPositionsContext);
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for AnnotatedIdentifier<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    Identifier<StringType>: ResetPositions,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        self.identifier.reset_positions(context);
        context.process_filler_vec(&mut self.operator_fillers);
        context.process_non_empty_string(
            &TokenContent::from(AnnotationOperator).to_string(),
            &mut self.operator_position,
        );
        self.annotation.reset_positions(context);
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for Assignment<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        self.target.reset_positions(context);
        context.process_filler_vec(&mut self.operator_fillers);
        context.process_non_empty_string(
            &TokenContent::from(AssignmentOperator).to_string(),
            &mut self.operator_position,
        );
        self.value.reset_positions(context);
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for BidirectionalConditional<StringType>
where
    Block<StringType>: ResetPositions,
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        context.process_filler_vec(&mut self.antecedent_opener_fillers);
        context.process_non_empty_string(
            CONDITIONAL_ANTECEDENT_OPENER,
            &mut self.antecedent_opener_position,
        );
        self.antecedent.reset_positions(context);
        self.consequent.reset_positions(context);
        context.process_filler_vec(&mut self.alternative_opener_fillers);
        context.process_non_empty_string(
            CONDITIONAL_ALTERNATIVE_OPENER,
            &mut self.alternative_opener_position,
        );
        self.alternative.reset_positions(context);
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for BinaryArithmeticOperation<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        self.left.reset_positions(context);
        context.process_filler_vec(&mut self.operator_fillers);
        context.process_non_empty_string(
            &TokenContent::from(self.operator).to_string(),
            &mut self.operator_position,
        );
        self.right.reset_positions(context);
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for BinaryComparison<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        self.left.reset_positions(context);
        context.process_filler_vec(&mut self.operator_fillers);
        context.process_non_empty_string(
            &TokenContent::from(self.operator).to_string(),
            &mut self.operator_position,
        );
        self.right.reset_positions(context);
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for Block<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    Statement<StringType>: ResetPositions,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        context.process_filler_vec(&mut self.open_brace_fillers);
        context.process_non_empty_string(
            &TokenContent::OpenBrace.to_string(),
            &mut self.open_brace_position,
        );
        for statement in &mut self.statements {
            statement.reset_positions(context);
        }
        if let Some(expression) = &mut self.expression {
            expression.reset_positions(context);
        }
        context.process_filler_vec(&mut self.close_brace_fillers);
        context.process_non_empty_string(
            &TokenContent::CloseBrace.to_string(),
            &mut self.close_brace_position,
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for Call<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        self.callable.reset_positions(context);
        context.process_filler_vec(&mut self.open_parenthesis_fillers);
        context.process_non_empty_string(
            &TokenContent::OpenParenthesis.to_string(),
            &mut self.open_parenthesis_position,
        );

        debug_assert_eq!(self.comma_positions.len(), self.comma_fillers.len());
        if self.arguments.len() == self.comma_positions.len() {
            let comma_string = TokenContent::Comma.to_string();
            for (argument, (comma_position, comma_fillers)) in
                self.arguments.iter_mut().zip(
                    self.comma_positions
                        .iter_mut()
                        .zip(self.comma_fillers.iter_mut()),
                )
            {
                argument.reset_positions(context);
                context.process_filler_vec(comma_fillers);
                context
                    .process_non_empty_string(&comma_string, comma_position);
            }
        } else {
            debug_assert_eq!(
                self.arguments.len(),
                self.comma_positions.len() + 1usize
            );
            if let [head_arguments @ .., last_argument] =
                self.arguments.as_mut_slice()
            {
                let comma_string = TokenContent::Comma.to_string();
                for (argument, (comma_position, comma_fillers)) in
                    head_arguments.iter_mut().zip(
                        self.comma_positions
                            .iter_mut()
                            .zip(self.comma_fillers.iter_mut()),
                    )
                {
                    argument.reset_positions(context);
                    context.process_filler_vec(comma_fillers);
                    context.process_non_empty_string(
                        &comma_string,
                        comma_position,
                    );
                }
                last_argument.reset_positions(context);
            } else {
                unreachable!("There should be at least one argument");
            }
        }

        context.process_filler_vec(&mut self.close_parenthesis_fillers);
        context.process_non_empty_string(
            &TokenContent::CloseParenthesis.to_string(),
            &mut self.close_parenthesis_position,
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + From<&'static str> + Utf8Size>
    ResetPositions for Expression<StringType>
where
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        match self {
            Expression::AnnotatedIdentifier(value) => {
                value.reset_positions(context)
            }
            Expression::Assignment(value) => value.reset_positions(context),
            Expression::BidirectionalConditional(value) => {
                value.reset_positions(context)
            }
            Expression::BinaryArithmeticOperation(value) => {
                value.reset_positions(context)
            }
            Expression::BinaryComparison(value) => {
                value.reset_positions(context)
            }
            Expression::Block(value) => value.reset_positions(context),
            Expression::Call(value) => value.reset_positions(context),
            Expression::FunctionDefinition(value) => {
                value.reset_positions(context)
            }
            Expression::FunctionType(value) => value.reset_positions(context),
            Expression::Grouping(value) => value.reset_positions(context),
            Expression::Identifier(value) => value.reset_positions(context),
            Expression::MemberAccess(value) => value.reset_positions(context),
            Expression::NumericLiteral(value) => {
                value.reset_positions(context)
            }
            Expression::Return(value) => value.reset_positions(context),
            Expression::Tuple(value) => value.reset_positions(context),
            Expression::UnaryArithmeticOperation(value) => {
                value.reset_positions(context)
            }
            Expression::UnidirectionalConditional(value) => {
                value.reset_positions(context)
            }
            Expression::WhileLoop(value) => value.reset_positions(context),
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for ExpressionStatement<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        self.expression.reset_positions(context);
        context.process_filler_vec(&mut self.semicolon_fillers);
        context.process_non_empty_string(
            &TokenContent::Semicolon.to_string(),
            &mut self.semicolon_position,
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for FunctionDefinition<StringType>
where
    AnnotatedIdentifier<StringType>: ResetPositions,
    Block<StringType>: ResetPositions,
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        context.process_filler_vec(&mut self.opener_fillers);
        context.process_non_empty_string(
            FUNCTION_DEFINITION_OPENER,
            &mut self.opener_position,
        );
        context.process_filler_vec(&mut self.open_parenthesis_fillers);
        context.process_non_empty_string(
            &TokenContent::OpenParenthesis.to_string(),
            &mut self.open_parenthesis_position,
        );

        debug_assert_eq!(self.comma_positions.len(), self.comma_fillers.len());
        if self.parameters.len() == self.comma_positions.len() {
            let comma_string = TokenContent::Comma.to_string();
            for (parameter, (comma_position, comma_fillers)) in
                self.parameters.iter_mut().zip(
                    self.comma_positions
                        .iter_mut()
                        .zip(self.comma_fillers.iter_mut()),
                )
            {
                parameter.reset_positions(context);
                context.process_filler_vec(comma_fillers);
                context
                    .process_non_empty_string(&comma_string, comma_position);
            }
        } else {
            debug_assert_eq!(
                self.parameters.len(),
                self.comma_positions.len() + 1usize
            );
            if let [head_parameters @ .., last_parameter] =
                self.parameters.as_mut_slice()
            {
                let comma_string = TokenContent::Comma.to_string();
                for (parameter, (comma_position, comma_fillers)) in
                    head_parameters.iter_mut().zip(
                        self.comma_positions
                            .iter_mut()
                            .zip(self.comma_fillers.iter_mut()),
                    )
                {
                    parameter.reset_positions(context);
                    context.process_filler_vec(comma_fillers);
                    context.process_non_empty_string(
                        &comma_string,
                        comma_position,
                    );
                }
                last_parameter.reset_positions(context);
            } else {
                unreachable!("There should be at least one parameter");
            }
        }

        context.process_filler_vec(&mut self.close_parenthesis_fillers);
        context.process_non_empty_string(
            &TokenContent::CloseParenthesis.to_string(),
            &mut self.close_parenthesis_position,
        );
        context.process_filler_vec(&mut self.arrow_fillers);
        context.process_non_empty_string(
            &TokenContent::Arrow.to_string(),
            &mut self.arrow_position,
        );
        self.return_type.reset_positions(context);
        self.body.reset_positions(context);
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for FunctionType<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        context.process_filler_vec(&mut self.open_parenthesis_fillers);
        context.process_non_empty_string(
            &TokenContent::OpenParenthesis.to_string(),
            &mut self.open_parenthesis_position,
        );

        debug_assert_eq!(self.comma_positions.len(), self.comma_fillers.len());
        if self.parameters.len() == self.comma_positions.len() {
            let comma_string = TokenContent::Comma.to_string();
            for (parameter, (comma_position, comma_fillers)) in
                self.parameters.iter_mut().zip(
                    self.comma_positions
                        .iter_mut()
                        .zip(self.comma_fillers.iter_mut()),
                )
            {
                parameter.reset_positions(context);
                context.process_filler_vec(comma_fillers);
                context
                    .process_non_empty_string(&comma_string, comma_position);
            }
        } else {
            debug_assert_eq!(
                self.parameters.len(),
                self.comma_positions.len() + 1usize
            );
            if let [head_parameters @ .., last_parameter] =
                self.parameters.as_mut_slice()
            {
                let comma_string = TokenContent::Comma.to_string();
                for (parameter, (comma_position, comma_fillers)) in
                    head_parameters.iter_mut().zip(
                        self.comma_positions
                            .iter_mut()
                            .zip(self.comma_fillers.iter_mut()),
                    )
                {
                    parameter.reset_positions(context);
                    context.process_filler_vec(comma_fillers);
                    context.process_non_empty_string(
                        &comma_string,
                        comma_position,
                    );
                }
                last_parameter.reset_positions(context);
            } else {
                unreachable!("There should be at least one parameter");
            }
        }

        context.process_filler_vec(&mut self.close_parenthesis_fillers);
        context.process_non_empty_string(
            &TokenContent::CloseParenthesis.to_string(),
            &mut self.close_parenthesis_position,
        );
        context.process_filler_vec(&mut self.operator_fillers);
        context.process_non_empty_string(
            &TokenContent::from(FunctionTypeOperator).to_string(),
            &mut self.operator_position,
        );
        self.return_type.reset_positions(context);
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for Grouping<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        context.process_filler_vec(&mut self.open_parenthesis_fillers);
        context.process_non_empty_string(
            &TokenContent::OpenParenthesis.to_string(),
            &mut self.open_parenthesis_position,
        );
        self.expression.reset_positions(context);
        context.process_filler_vec(&mut self.close_parenthesis_fillers);
        context.process_non_empty_string(
            &TokenContent::CloseParenthesis.to_string(),
            &mut self.close_parenthesis_position,
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for Identifier<StringType>
where
    FillerContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        context.process_filler_vec(&mut self.fillers);
        context.process_non_empty_string(&self.string, &mut self.position);
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for MemberAccess<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    Identifier<StringType>: ResetPositions,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        self.object.reset_positions(context);
        context.process_filler_vec(&mut self.operator_fillers);
        context.process_non_empty_string(
            &TokenContent::from(MemberAccessOperator).to_string(),
            &mut self.operator_position,
        );
        self.member.reset_positions(context);
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for NumericLiteral<StringType>
where
    FillerContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        context.process_filler_vec(&mut self.fillers);
        context.process_non_empty_string(
            &TokenContent::NumericLiteral {
                value: self.value.as_ref(),
                type_: self.type_,
            }
            .to_string(),
            &mut self.position,
        );
    }
}

impl<StringType> ResetPositions for Statement<StringType>
where
    ExpressionStatement<StringType>: ResetPositions,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        match self {
            Statement::Expression(value) => value.reset_positions(context),
        }
    }
}

impl<StringType: AsRef<str> + From<&'static str> + ByteSize + Utf8Size>
    ResetPositions for Return<StringType>
where
    Block<StringType>: ResetPositions,
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        context.process_filler_vec(&mut self.operator_fillers);
        context.process_non_empty_string(
            &TokenContent::from(ReturnOperator).to_string(),
            &mut self.operator_position,
        );
        self.expression.reset_positions(context);
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for Tuple<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        context.process_filler_vec(&mut self.open_parenthesis_fillers);
        context.process_non_empty_string(
            &TokenContent::OpenParenthesis.to_string(),
            &mut self.open_parenthesis_position,
        );

        debug_assert_eq!(self.comma_positions.len(), self.comma_fillers.len());
        if self.elements.len() == self.comma_positions.len() {
            let comma_string = TokenContent::Comma.to_string();
            for (element, (comma_position, comma_fillers)) in
                self.elements.iter_mut().zip(
                    self.comma_positions
                        .iter_mut()
                        .zip(self.comma_fillers.iter_mut()),
                )
            {
                element.reset_positions(context);
                context.process_filler_vec(comma_fillers);
                context
                    .process_non_empty_string(&comma_string, comma_position);
            }
        } else {
            debug_assert_eq!(
                self.elements.len(),
                self.comma_positions.len() + 1usize
            );
            if let [head_elements @ .., last_element] =
                self.elements.as_mut_slice()
            {
                let comma_string = TokenContent::Comma.to_string();
                for (element, (comma_position, comma_fillers)) in
                    head_elements.iter_mut().zip(
                        self.comma_positions
                            .iter_mut()
                            .zip(self.comma_fillers.iter_mut()),
                    )
                {
                    element.reset_positions(context);
                    context.process_filler_vec(comma_fillers);
                    context.process_non_empty_string(
                        &comma_string,
                        comma_position,
                    );
                }
                last_element.reset_positions(context);
            } else {
                unreachable!("There should be at least one element");
            }
        }

        context.process_filler_vec(&mut self.close_parenthesis_fillers);
        context.process_non_empty_string(
            &TokenContent::CloseParenthesis.to_string(),
            &mut self.close_parenthesis_position,
        );
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for UnaryArithmeticOperation<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        context.process_filler_vec(&mut self.operator_fillers);
        context.process_non_empty_string(
            &TokenContent::from(self.operator).to_string(),
            &mut self.operator_position,
        );
        self.operand.reset_positions(context);
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for UnidirectionalConditional<StringType>
where
    Block<StringType>: ResetPositions,
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        context.process_filler_vec(&mut self.opener_fillers);
        context.process_non_empty_string(
            CONDITIONAL_ANTECEDENT_OPENER,
            &mut self.opener_position,
        );
        self.antecedent.reset_positions(context);
        self.consequent.reset_positions(context);
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for WhileLoop<StringType>
where
    Block<StringType>: ResetPositions,
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        context.process_filler_vec(&mut self.opener_fillers);
        context.process_non_empty_string(
            WHILE_LOOP_OPENER,
            &mut self.opener_position,
        );
        self.condition.reset_positions(context);
        self.body.reset_positions(context);
    }
}
