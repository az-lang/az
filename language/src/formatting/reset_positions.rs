use crate::parsing::{
    AnnotatedIdentifier, Assignment, BinaryArithmeticOperation,
    BinaryAssignmentOperator, BinaryComparison, Block, Call, Conditional,
    Expression, ExpressionStatement, Filler, FillerContent,
    FunctionDefinition, Grouping, Identifier, MemberAccess,
    MemberAccessOperator, NumericLiteral, Script, Statement, Tuple,
    UnaryArithmeticOperation, CONDITIONAL_ALTERNATIVE_OPENER,
    CONDITIONAL_ANTECEDENT_OPENER, FUNCTION_OPENER,
};
use crate::tokenization::{
    ByteSize, CharacterPosition, TokenContent, Utf8Size,
};

pub(crate) fn reset_script_positions<StringType: ByteSize + Utf8Size>(
    value: &mut Script<StringType>,
) where
    FillerContent<StringType>: ToString,
    Statement<StringType>: ResetPositions,
{
    let mut context = ResetPositionsContext {
        current_line_index: 0usize,
        current_character_position: CharacterPosition {
            byte: 0usize.into(),
            utf_8: 0usize.into(),
        },
    };
    for statement in &mut value.statements {
        statement.reset_positions(&mut context);
    }
    reset_fillers_positions(&mut value.fillers, &mut context);
}

type Fillers<StringType> = Vec<Filler<StringType>>;

#[derive(Clone)]
pub struct ResetPositionsContext {
    current_line_index: usize,
    current_character_position: CharacterPosition,
}

pub trait ResetPositions {
    fn reset_positions(&mut self, context: &mut ResetPositionsContext);
}

impl<StringType: ByteSize + Utf8Size> ResetPositions
    for AnnotatedIdentifier<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    Identifier<StringType>: ResetPositions,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        self.identifier.reset_positions(context);
        reset_fillers_positions(&mut self.operator_fillers, context);
        self.operator_position.start_line = context.current_line_index;
        self.operator_position.start_character =
            context.current_character_position;
        let operator_string = TokenContent::Semicolon.to_string();
        context.current_character_position.byte += operator_string.byte_size();
        context.current_character_position.utf_8 +=
            operator_string.utf_8_size();
        self.operator_position.end_line = context.current_line_index;
        self.operator_position.end_character =
            context.current_character_position;
        self.annotation.reset_positions(context);
    }
}

impl<StringType: ByteSize + Utf8Size> ResetPositions for Assignment<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        self.target.reset_positions(context);
        reset_fillers_positions(&mut self.operator_fillers, context);
        self.operator_position.start_line = context.current_line_index;
        self.operator_position.start_character =
            context.current_character_position;
        let operator_string =
            TokenContent::from(BinaryAssignmentOperator).to_string();
        context.current_character_position.byte += operator_string.byte_size();
        context.current_character_position.utf_8 +=
            operator_string.utf_8_size();
        self.operator_position.end_line = context.current_line_index;
        self.operator_position.end_character =
            context.current_character_position;
        self.value.reset_positions(context);
    }
}

impl<StringType: ByteSize + Utf8Size> ResetPositions
    for BinaryArithmeticOperation<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        self.left.reset_positions(context);
        reset_fillers_positions(&mut self.operator_fillers, context);
        self.operator_position.start_line = context.current_line_index;
        self.operator_position.start_character =
            context.current_character_position;
        let operator_string = TokenContent::from(self.operator).to_string();
        context.current_character_position.byte += operator_string.byte_size();
        context.current_character_position.utf_8 +=
            operator_string.utf_8_size();
        self.operator_position.end_line = context.current_line_index;
        self.operator_position.end_character =
            context.current_character_position;
        self.right.reset_positions(context);
    }
}

impl<StringType: ByteSize + Utf8Size> ResetPositions
    for BinaryComparison<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        self.left.reset_positions(context);
        reset_fillers_positions(&mut self.operator_fillers, context);
        self.operator_position.start_line = context.current_line_index;
        self.operator_position.start_character =
            context.current_character_position;
        let operator_string = TokenContent::from(self.operator).to_string();
        context.current_character_position.byte += operator_string.byte_size();
        context.current_character_position.utf_8 +=
            operator_string.utf_8_size();
        self.operator_position.end_line = context.current_line_index;
        self.operator_position.end_character =
            context.current_character_position;
        self.right.reset_positions(context);
    }
}

impl<StringType: ByteSize + Utf8Size> ResetPositions for Block<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    Statement<StringType>: ResetPositions,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        reset_fillers_positions(&mut self.open_brace_fillers, context);
        self.open_brace_position.start_line = context.current_line_index;
        self.open_brace_position.start_character =
            context.current_character_position;
        let open_brace_string = TokenContent::OpenBrace.to_string();
        context.current_character_position.byte +=
            open_brace_string.byte_size();
        context.current_character_position.utf_8 +=
            open_brace_string.utf_8_size();
        self.open_brace_position.end_line = context.current_line_index;
        self.open_brace_position.end_character =
            context.current_character_position;
        for statement in &mut self.statements {
            statement.reset_positions(context);
        }
        if let Some(expression) = &mut self.expression {
            expression.reset_positions(context);
        }
        reset_fillers_positions(&mut self.close_brace_fillers, context);
        self.close_brace_position.start_line = context.current_line_index;
        self.close_brace_position.start_character =
            context.current_character_position;
        let close_brace_string = TokenContent::CloseBrace.to_string();
        context.current_character_position.byte +=
            close_brace_string.byte_size();
        context.current_character_position.utf_8 +=
            close_brace_string.utf_8_size();
        self.close_brace_position.end_line = context.current_line_index;
        self.close_brace_position.end_character =
            context.current_character_position;
    }
}

impl<StringType: ByteSize + Utf8Size> ResetPositions for Call<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        self.callable.reset_positions(context);
        reset_fillers_positions(&mut self.open_parenthesis_fillers, context);
        self.open_parenthesis_position.start_line = context.current_line_index;
        self.open_parenthesis_position.start_character =
            context.current_character_position;
        let open_parenthesis_string =
            TokenContent::OpenParenthesis.to_string();
        context.current_character_position.byte +=
            open_parenthesis_string.byte_size();
        context.current_character_position.utf_8 +=
            open_parenthesis_string.utf_8_size();
        self.open_parenthesis_position.end_line = context.current_line_index;
        self.open_parenthesis_position.end_character =
            context.current_character_position;

        debug_assert_eq!(
            self.commas_positions.len(),
            self.commas_fillers.len()
        );
        if self.arguments.len() == self.commas_positions.len() {
            let comma_string = TokenContent::Comma.to_string();
            for (argument, (comma_position, comma_fillers)) in
                self.arguments.iter_mut().zip(
                    self.commas_positions
                        .iter_mut()
                        .zip(self.commas_fillers.iter_mut()),
                )
            {
                argument.reset_positions(context);
                reset_fillers_positions(comma_fillers, context);
                comma_position.start_line = context.current_line_index;
                comma_position.start_character =
                    context.current_character_position;
                context.current_character_position.byte +=
                    comma_string.byte_size();
                context.current_character_position.utf_8 +=
                    comma_string.utf_8_size();
                comma_position.end_line = context.current_line_index;
                comma_position.end_character =
                    context.current_character_position;
            }
        } else {
            debug_assert_eq!(
                self.arguments.len(),
                self.commas_positions.len() + 1usize
            );
            if let [head_arguments @ .., last_argument] =
                self.arguments.as_mut_slice()
            {
                let comma_string = TokenContent::Comma.to_string();
                for (argument, (comma_position, comma_fillers)) in
                    head_arguments.iter_mut().zip(
                        self.commas_positions
                            .iter_mut()
                            .zip(self.commas_fillers.iter_mut()),
                    )
                {
                    argument.reset_positions(context);
                    reset_fillers_positions(comma_fillers, context);
                    comma_position.start_line = context.current_line_index;
                    comma_position.start_character =
                        context.current_character_position;
                    context.current_character_position.byte +=
                        comma_string.byte_size();
                    context.current_character_position.utf_8 +=
                        comma_string.utf_8_size();
                    comma_position.end_line = context.current_line_index;
                    comma_position.end_character =
                        context.current_character_position;
                }
                last_argument.reset_positions(context);
            } else {
                unreachable!("There should be at least one argument");
            }
        }

        reset_fillers_positions(&mut self.close_parenthesis_fillers, context);
        self.close_parenthesis_position.start_line =
            context.current_line_index;
        self.close_parenthesis_position.start_character =
            context.current_character_position;
        let close_parenthesis_string =
            TokenContent::CloseParenthesis.to_string();
        context.current_character_position.byte +=
            close_parenthesis_string.byte_size();
        context.current_character_position.utf_8 +=
            close_parenthesis_string.utf_8_size();
        self.close_parenthesis_position.end_line = context.current_line_index;
        self.close_parenthesis_position.end_character =
            context.current_character_position;
    }
}

impl<StringType: ByteSize + Utf8Size> ResetPositions
    for Conditional<StringType>
where
    Block<StringType>: ResetPositions,
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        reset_fillers_positions(&mut self.opener_fillers, context);
        self.opener_position.start_line = context.current_line_index;
        self.opener_position.start_character =
            context.current_character_position;
        context.current_character_position.byte +=
            CONDITIONAL_ANTECEDENT_OPENER.byte_size();
        context.current_character_position.utf_8 +=
            CONDITIONAL_ANTECEDENT_OPENER.utf_8_size();
        self.opener_position.end_line = context.current_line_index;
        self.opener_position.end_character =
            context.current_character_position;
        self.antecedent.reset_positions(context);
        self.consequent.reset_positions(context);
        reset_fillers_positions(&mut self.alternative_opener_fillers, context);
        if let Some(alternative_opener_position) =
            &mut self.alternative_opener_position
        {
            alternative_opener_position.start_line =
                context.current_line_index;
            alternative_opener_position.start_character =
                context.current_character_position;
            context.current_character_position.byte +=
                CONDITIONAL_ALTERNATIVE_OPENER.byte_size();
            context.current_character_position.utf_8 +=
                CONDITIONAL_ALTERNATIVE_OPENER.utf_8_size();
            alternative_opener_position.end_line = context.current_line_index;
            alternative_opener_position.end_character =
                context.current_character_position;
        }
        if let Some(alternative) = &mut self.alternative {
            alternative.reset_positions(context);
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for Expression<StringType>
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
            Expression::BinaryArithmeticOperation(value) => {
                value.reset_positions(context)
            }
            Expression::BinaryComparison(value) => {
                value.reset_positions(context)
            }
            Expression::Block(value) => value.reset_positions(context),
            Expression::Call(value) => value.reset_positions(context),
            Expression::Conditional(value) => value.reset_positions(context),
            Expression::FunctionDefinition(value) => {
                value.reset_positions(context)
            }
            Expression::Grouping(value) => value.reset_positions(context),
            Expression::Identifier(value) => value.reset_positions(context),
            Expression::MemberAccess(value) => value.reset_positions(context),
            Expression::NumericLiteral(value) => {
                value.reset_positions(context)
            }
            Expression::Tuple(value) => value.reset_positions(context),
            Expression::UnaryArithmeticOperation(value) => {
                value.reset_positions(context)
            }
        }
    }
}

impl<StringType: ByteSize + Utf8Size> ResetPositions
    for ExpressionStatement<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        self.expression.reset_positions(context);
        reset_fillers_positions(&mut self.semicolon_fillers, context);
        self.semicolon_position.start_line = context.current_line_index;
        self.semicolon_position.start_character =
            context.current_character_position;
        let semicolon_string = TokenContent::Semicolon.to_string();
        context.current_character_position.byte +=
            semicolon_string.byte_size();
        context.current_character_position.utf_8 +=
            semicolon_string.utf_8_size();
        self.semicolon_position.end_line = context.current_line_index;
        self.semicolon_position.end_character =
            context.current_character_position;
    }
}

impl<StringType: ByteSize + Utf8Size> ResetPositions
    for FunctionDefinition<StringType>
where
    Block<StringType>: ResetPositions,
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        reset_fillers_positions(&mut self.opener_fillers, context);
        self.opener_position.start_line = context.current_line_index;
        self.opener_position.start_character =
            context.current_character_position;
        context.current_character_position.byte += FUNCTION_OPENER.byte_size();
        context.current_character_position.utf_8 +=
            FUNCTION_OPENER.utf_8_size();
        self.opener_position.end_line = context.current_line_index;
        self.opener_position.end_character =
            context.current_character_position;

        reset_fillers_positions(&mut self.open_parenthesis_fillers, context);
        self.open_parenthesis_position.start_line = context.current_line_index;
        self.open_parenthesis_position.start_character =
            context.current_character_position;
        let open_parenthesis_string =
            TokenContent::OpenParenthesis.to_string();
        context.current_character_position.byte +=
            open_parenthesis_string.byte_size();
        context.current_character_position.utf_8 +=
            open_parenthesis_string.utf_8_size();
        self.open_parenthesis_position.end_line = context.current_line_index;
        self.open_parenthesis_position.end_character =
            context.current_character_position;

        debug_assert_eq!(
            self.commas_positions.len(),
            self.commas_fillers.len()
        );
        if self.parameters.len() == self.commas_positions.len() {
            let comma_string = TokenContent::Comma.to_string();
            for (parameter, (comma_position, comma_fillers)) in
                self.parameters.iter_mut().zip(
                    self.commas_positions
                        .iter_mut()
                        .zip(self.commas_fillers.iter_mut()),
                )
            {
                parameter.reset_positions(context);
                reset_fillers_positions(comma_fillers, context);
                comma_position.start_line = context.current_line_index;
                comma_position.start_character =
                    context.current_character_position;
                context.current_character_position.byte +=
                    comma_string.byte_size();
                context.current_character_position.utf_8 +=
                    comma_string.utf_8_size();
                comma_position.end_line = context.current_line_index;
                comma_position.end_character =
                    context.current_character_position;
            }
        } else {
            debug_assert_eq!(
                self.parameters.len(),
                self.commas_positions.len() + 1usize
            );
            if let [head_parameters @ .., last_parameter] =
                self.parameters.as_mut_slice()
            {
                let comma_string = TokenContent::Comma.to_string();
                for (parameter, (comma_position, comma_fillers)) in
                    head_parameters.iter_mut().zip(
                        self.commas_positions
                            .iter_mut()
                            .zip(self.commas_fillers.iter_mut()),
                    )
                {
                    parameter.reset_positions(context);
                    reset_fillers_positions(comma_fillers, context);
                    comma_position.start_line = context.current_line_index;
                    comma_position.start_character =
                        context.current_character_position;
                    context.current_character_position.byte +=
                        comma_string.byte_size();
                    context.current_character_position.utf_8 +=
                        comma_string.utf_8_size();
                    comma_position.end_line = context.current_line_index;
                    comma_position.end_character =
                        context.current_character_position;
                }
                last_parameter.reset_positions(context);
            } else {
                unreachable!("There should be at least one parameter");
            }
        }

        reset_fillers_positions(&mut self.close_parenthesis_fillers, context);
        self.close_parenthesis_position.start_line =
            context.current_line_index;
        self.close_parenthesis_position.start_character =
            context.current_character_position;
        let close_parenthesis_string =
            TokenContent::CloseParenthesis.to_string();
        context.current_character_position.byte +=
            close_parenthesis_string.byte_size();
        context.current_character_position.utf_8 +=
            close_parenthesis_string.utf_8_size();
        self.close_parenthesis_position.end_line = context.current_line_index;
        self.close_parenthesis_position.end_character =
            context.current_character_position;
        reset_fillers_positions(&mut self.arrow_fillers, context);
        self.arrow_position.start_line = context.current_line_index;
        self.arrow_position.start_character =
            context.current_character_position;
        let arrow_string = TokenContent::Arrow.to_string();
        context.current_character_position.byte += arrow_string.byte_size();
        context.current_character_position.utf_8 += arrow_string.utf_8_size();
        self.arrow_position.end_line = context.current_line_index;
        self.arrow_position.end_character = context.current_character_position;
        self.return_type.reset_positions(context);
        self.body.reset_positions(context);
    }
}

impl<StringType: ByteSize + Utf8Size> ResetPositions for Grouping<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        reset_fillers_positions(&mut self.open_parenthesis_fillers, context);
        self.open_parenthesis_position.start_line = context.current_line_index;
        self.open_parenthesis_position.start_character =
            context.current_character_position;
        let open_parenthesis_string =
            TokenContent::OpenParenthesis.to_string();
        context.current_character_position.byte +=
            open_parenthesis_string.byte_size();
        context.current_character_position.utf_8 +=
            open_parenthesis_string.utf_8_size();
        self.open_parenthesis_position.end_line = context.current_line_index;
        self.open_parenthesis_position.end_character =
            context.current_character_position;
        self.expression.reset_positions(context);
        reset_fillers_positions(&mut self.close_parenthesis_fillers, context);
        self.close_parenthesis_position.start_line =
            context.current_line_index;
        self.close_parenthesis_position.start_character =
            context.current_character_position;
        let close_parenthesis_string =
            TokenContent::CloseParenthesis.to_string();
        context.current_character_position.byte +=
            close_parenthesis_string.byte_size();
        context.current_character_position.utf_8 +=
            close_parenthesis_string.utf_8_size();
        self.close_parenthesis_position.end_line = context.current_line_index;
        self.close_parenthesis_position.end_character =
            context.current_character_position;
    }
}

impl<StringType: ByteSize + Utf8Size> ResetPositions for Identifier<StringType>
where
    FillerContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        reset_fillers_positions(&mut self.fillers, context);
        self.position.start_line = context.current_line_index;
        self.position.start_character = context.current_character_position;
        context.current_character_position.byte += self.string.byte_size();
        context.current_character_position.utf_8 += self.string.utf_8_size();
        self.position.end_line = context.current_line_index;
        self.position.end_character = context.current_character_position;
    }
}

impl<StringType: ByteSize + Utf8Size> ResetPositions
    for MemberAccess<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        self.object.reset_positions(context);
        reset_fillers_positions(&mut self.operator_fillers, context);
        self.operator_position.start_line = context.current_line_index;
        self.operator_position.start_character =
            context.current_character_position;
        let operator_string =
            TokenContent::from(MemberAccessOperator).to_string();
        context.current_character_position.byte += operator_string.byte_size();
        context.current_character_position.utf_8 +=
            operator_string.utf_8_size();
        self.operator_position.end_line = context.current_line_index;
        self.operator_position.end_character =
            context.current_character_position;
        self.member.reset_positions(context);
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> ResetPositions
    for NumericLiteral<StringType>
where
    FillerContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        reset_fillers_positions(&mut self.fillers, context);
        self.position.start_line = context.current_line_index;
        self.position.start_character = context.current_character_position;
        let literal_string = TokenContent::NumericLiteral {
            value: self.value.as_ref(),
            type_: self.type_,
        }
        .to_string();
        context.current_character_position.byte += literal_string.byte_size();
        context.current_character_position.utf_8 +=
            literal_string.utf_8_size();
        self.position.end_line = context.current_line_index;
        self.position.end_character = context.current_character_position;
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

impl<StringType: ByteSize + Utf8Size> ResetPositions for Tuple<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        reset_fillers_positions(&mut self.open_parenthesis_fillers, context);
        self.open_parenthesis_position.start_line = context.current_line_index;
        self.open_parenthesis_position.start_character =
            context.current_character_position;
        let open_parenthesis_string =
            TokenContent::OpenParenthesis.to_string();
        context.current_character_position.byte +=
            open_parenthesis_string.byte_size();
        context.current_character_position.utf_8 +=
            open_parenthesis_string.utf_8_size();
        self.open_parenthesis_position.end_line = context.current_line_index;
        self.open_parenthesis_position.end_character =
            context.current_character_position;

        debug_assert_eq!(
            self.commas_positions.len(),
            self.commas_fillers.len()
        );
        if self.elements.len() == self.commas_positions.len() {
            let comma_string = TokenContent::Comma.to_string();
            for (element, (comma_position, comma_fillers)) in
                self.elements.iter_mut().zip(
                    self.commas_positions
                        .iter_mut()
                        .zip(self.commas_fillers.iter_mut()),
                )
            {
                element.reset_positions(context);
                reset_fillers_positions(comma_fillers, context);
                comma_position.start_line = context.current_line_index;
                comma_position.start_character =
                    context.current_character_position;
                context.current_character_position.byte +=
                    comma_string.byte_size();
                context.current_character_position.utf_8 +=
                    comma_string.utf_8_size();
                comma_position.end_line = context.current_line_index;
                comma_position.end_character =
                    context.current_character_position;
            }
        } else {
            debug_assert_eq!(
                self.elements.len(),
                self.commas_positions.len() + 1usize
            );
            if let [head_elements @ .., last_element] =
                self.elements.as_mut_slice()
            {
                let comma_string = TokenContent::Comma.to_string();
                for (element, (comma_position, comma_fillers)) in
                    head_elements.iter_mut().zip(
                        self.commas_positions
                            .iter_mut()
                            .zip(self.commas_fillers.iter_mut()),
                    )
                {
                    element.reset_positions(context);
                    reset_fillers_positions(comma_fillers, context);
                    comma_position.start_line = context.current_line_index;
                    comma_position.start_character =
                        context.current_character_position;
                    context.current_character_position.byte +=
                        comma_string.byte_size();
                    context.current_character_position.utf_8 +=
                        comma_string.utf_8_size();
                    comma_position.end_line = context.current_line_index;
                    comma_position.end_character =
                        context.current_character_position;
                }
                last_element.reset_positions(context);
            } else {
                unreachable!("There should be at least one element");
            }
        }

        reset_fillers_positions(&mut self.close_parenthesis_fillers, context);
        self.close_parenthesis_position.start_line =
            context.current_line_index;
        self.close_parenthesis_position.start_character =
            context.current_character_position;
        let close_parenthesis_string =
            TokenContent::CloseParenthesis.to_string();
        context.current_character_position.byte +=
            close_parenthesis_string.byte_size();
        context.current_character_position.utf_8 +=
            close_parenthesis_string.utf_8_size();
        self.close_parenthesis_position.end_line = context.current_line_index;
        self.close_parenthesis_position.end_character =
            context.current_character_position;
    }
}

impl<StringType: ByteSize + Utf8Size> ResetPositions
    for UnaryArithmeticOperation<StringType>
where
    Expression<StringType>: ResetPositions,
    FillerContent<StringType>: ToString,
    TokenContent<StringType>: ToString,
{
    fn reset_positions(&mut self, context: &mut ResetPositionsContext) {
        reset_fillers_positions(&mut self.operator_fillers, context);
        self.operator_position.start_line = context.current_line_index;
        self.operator_position.start_character =
            context.current_character_position;
        let operator_string = TokenContent::from(self.operator).to_string();
        context.current_character_position.byte += operator_string.byte_size();
        context.current_character_position.utf_8 +=
            operator_string.utf_8_size();
        self.operator_position.end_line = context.current_line_index;
        self.operator_position.end_character =
            context.current_character_position;
        self.operand.reset_positions(context);
    }
}

fn reset_fillers_positions<StringType: ByteSize + Utf8Size>(
    value: &mut Fillers<StringType>,
    context: &mut ResetPositionsContext,
) where
    FillerContent<StringType>: ToString,
{
    for filler in value {
        filler.position.start_line = context.current_line_index;
        filler.position.start_character = context.current_character_position;
        match &filler.content {
            FillerContent::CommentBlock(value) => {
                match value.as_slice() {
                    [only_line] => {
                        context.current_character_position.byte +=
                            only_line.byte_size();
                        context.current_character_position.utf_8 +=
                            only_line.utf_8_size();
                    }
                    [.., last_line] => {
                        context.current_line_index += value.len() - 1usize;
                        context.current_character_position.byte =
                            last_line.byte_size();
                        context.current_character_position.utf_8 =
                            last_line.utf_8_size();
                    }
                    _ => unreachable!(
                        "Comment block should have at least one line"
                    ),
                }
                filler.position.end_line = context.current_line_index;
                filler.position.end_character =
                    context.current_character_position;
            }
            FillerContent::CommentLine(value) => {
                context.current_character_position.byte += value.byte_size();
                context.current_character_position.utf_8 += value.utf_8_size();
                filler.position.end_line = context.current_line_index;
                filler.position.end_character =
                    context.current_character_position;
                context.current_line_index += 1usize;
                context.current_character_position.byte = 0usize.into();
                context.current_character_position.utf_8 = 0usize.into();
            }
            FillerContent::Newline => {
                let filler_content_string = filler.content.to_string();
                context.current_character_position.byte +=
                    filler_content_string.byte_size();
                context.current_character_position.utf_8 +=
                    filler_content_string.utf_8_size();
                filler.position.end_line = context.current_line_index;
                filler.position.end_character =
                    context.current_character_position;
                context.current_line_index += 1usize;
                context.current_character_position.byte = 0usize.into();
                context.current_character_position.utf_8 = 0usize.into();
            }
            FillerContent::Whitespace(value) => {
                context.current_character_position.byte += value.byte_size();
                context.current_character_position.utf_8 += value.utf_8_size();
                filler.position.end_line = context.current_line_index;
                filler.position.end_character =
                    context.current_character_position;
            }
        }
    }
}
