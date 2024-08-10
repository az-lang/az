import {
  AnnotatedIdentifier,
  AnnotationOperator,
  Assignment,
  AssignmentOperator,
  BidirectionalConditional,
  BinaryAdditionOperator,
  BinaryArithmeticOperation,
  BinaryComparison,
  BinaryDivisionOperator,
  BinaryEqualToOperator,
  BinaryGreaterThanOperator,
  BinaryGreaterThanOrEqualToOperator,
  BinaryLessThanOperator,
  BinaryLessThanOrEqualToOperator,
  BinaryMultiplicationOperator,
  BinaryNotEqualToOperator,
  BinarySubtractionOperator,
  Block,
  Call,
  CallOperator,
  ExpressionStatement,
  Filler,
  FunctionDefinition,
  Grouping,
  Identifier,
  MemberAccess,
  MemberAccessOperator,
  NumericLiteral,
  Return,
  SubstringPosition,
  Tuple,
  UnaryArithmeticOperation,
  UnaryNegationOperator,
  UnidirectionalConditional,
  WhileLoop
} from '../pkg'

export type BinaryArithmeticOperator = (
  typeof BinaryAdditionOperator
  | typeof BinaryDivisionOperator
  | typeof BinaryMultiplicationOperator
  | typeof BinarySubtractionOperator
  )
export type BinaryComparisonOperator = (
  typeof BinaryEqualToOperator
  | typeof BinaryGreaterThanOperator
  | typeof BinaryGreaterThanOrEqualToOperator
  | typeof BinaryLessThanOperator
  | typeof BinaryLessThanOrEqualToOperator
  | typeof BinaryNotEqualToOperator
  )
export type BinaryOperator = (
  BinaryArithmeticOperator
  | BinaryComparisonOperator
  | typeof AnnotationOperator
  | typeof AssignmentOperator
  | typeof CallOperator
  | typeof MemberAccessOperator
  )
export type Expression =
  AnnotatedIdentifier
  | Assignment
  | BidirectionalConditional
  | BinaryArithmeticOperation
  | BinaryComparison
  | Block
  | Call
  | FunctionDefinition
  | Grouping
  | Identifier
  | MemberAccess
  | NumericLiteral
  | Return
  | Tuple
  | UnaryArithmeticOperation
  | UnidirectionalConditional
  | WhileLoop
export type Statement = ExpressionStatement
export type UnaryArithmeticOperator = typeof UnaryNegationOperator

export type AnnotatedIdentifierData<Annotation extends Expression> = {
  identifier: Identifier,
  annotation: Annotation,
  operatorPosition: SubstringPosition,
  operatorFillers: Filler[]
}
export type AssignmentData<
  Target extends Expression, Value extends Expression
> = {
  target: Target,
  value: Value,
  operatorPosition: SubstringPosition,
  operatorFillers: Filler[]
}
export type BidirectionalConditionalData<Antecedent extends Expression> = {
  antecedent: Antecedent,
  consequent: Block,
  alternative: Expression,
  antecedentOpenerPosition: SubstringPosition,
  alternativeOpenerPosition: SubstringPosition,
  antecedentOpenerFillers: Filler[],
  alternativeOpenerFillers: Filler[]
}
export type BinaryArithmeticOperationData<
  LeftOperand extends Expression, RightOperand extends Expression
> = {
  left: LeftOperand,
  right: RightOperand,
  operator: BinaryArithmeticOperator,
  operatorPosition: SubstringPosition,
  operatorFillers: Filler[]
}
export type BinaryComparisonData<
  LeftOperand extends Expression, RightOperand extends Expression
> = {
  left: LeftOperand,
  right: RightOperand,
  operator: BinaryComparisonOperator,
  operatorPosition: SubstringPosition,
  operatorFillers: Filler[]
}
export type BlockData<ExpressionT extends Expression> = {
  statements: Statement[],
  expression: ExpressionT | null,
  openBracePosition: SubstringPosition,
  closeBracePosition: SubstringPosition,
  openBraceFillers: Filler[],
  closeBraceFillers: Filler[]
}
export type CallData<
  Callable extends Expression, Argument extends Expression
> = {
  callable: Callable,
  arguments_: Argument[],
  openParenthesisPosition: SubstringPosition,
  commaPositions: SubstringPosition[],
  closeParenthesisPosition: SubstringPosition,
  openParenthesisFillers: Filler[],
  commaFillers: Filler[][],
  closeParenthesisFillers: Filler[]
}
export type FunctionDefinitionData<ReturnType extends Expression> = {
  parameters: AnnotatedIdentifier[],
  returnType: ReturnType,
  body: Block,
  openerPosition: SubstringPosition,
  openParenthesisPosition: SubstringPosition,
  commaPositions: SubstringPosition[],
  closeParenthesisPosition: SubstringPosition,
  arrowPosition: SubstringPosition,
  openerFillers: Filler[],
  openParenthesisFillers: Filler[],
  commaFillers: Filler[][],
  closeParenthesisFillers: Filler[],
  arrowFillers: Filler[]
}
export type GroupingData<ExpressionT extends Expression> = {
  expression: ExpressionT,
  openParenthesisPosition: SubstringPosition,
  closeParenthesisPosition: SubstringPosition,
  openParenthesisFillers: Filler[],
  closeParenthesisFillers: Filler[]
}
export type MemberAccessData<ObjectT extends Expression> = {
  object: ObjectT,
  member: Identifier,
  operatorPosition: SubstringPosition,
  operatorFillers: Filler[]
}
export type ReturnData<ExpressionT extends Expression> = {
  expression: ExpressionT,
  operatorPosition: SubstringPosition,
  operatorFillers: Filler[]
}
export type TupleData<Element extends Expression> = {
  elements: Element[],
  openParenthesisPosition: SubstringPosition,
  commaPositions: SubstringPosition[],
  closeParenthesisPosition: SubstringPosition,
  openParenthesisFillers: Filler[],
  commaFillers: Filler[][],
  closeParenthesisFillers: Filler[]
}
export type UnaryArithmeticOperationData<Operand extends Expression> = {
  operand: Operand,
  operator: UnaryArithmeticOperator,
  operatorPosition: SubstringPosition,
  operatorFillers: Filler[]
}
export type UnidirectionalConditionalData<Antecedent extends Expression> = {
  antecedent: Antecedent,
  consequent: Block,
  openerPosition: SubstringPosition,
  openerFillers: Filler[]
}
export type WhileLoopData<Condition extends Expression> = {
  condition: Condition,
  body: Block,
  openerPosition: SubstringPosition,
  openerFillers: Filler[]
}
