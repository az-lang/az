import fc from 'fast-check'
import {
  AnnotatedIdentifier,
  Assignment,
  BidirectionalConditional,
  BinaryAdditionOperator,
  BinaryArithmeticOperation,
  BinaryComparison,
  BinaryDivisionOperator,
  BinaryMultiplicationOperator,
  BinarySubtractionOperator,
  Block,
  Call,
  Filler,
  FillerKind,
  FunctionDefinition,
  Grouping,
  Identifier,
  MemberAccess,
  NumericLiteral,
  Return,
  SubstringPosition,
  Tuple,
  UnaryArithmeticOperation,
  UnidirectionalConditional,
  WhileLoop
} from '../../../../pkg'
import { BinaryArithmeticOperationData, Expression } from '../../../types'
import { nonCommentFillerStrategy } from '../../filler'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'

type BinaryArithmeticOperator =
  typeof BinaryAdditionOperator
  | typeof BinaryDivisionOperator
  | typeof BinaryMultiplicationOperator
  | typeof BinarySubtractionOperator;

function binaryArithmeticOperationArgumentsToStrategy<
  LeftOperand extends Expression, RightOperand extends Expression, Output
> (
  factory: (
    left: LeftOperand,
    right: RightOperand,
    operator: BinaryArithmeticOperator,
    operatorPosition: SubstringPosition,
    operatorFillers: Filler[]
  ) => Output,
  operator: BinaryArithmeticOperator,
  leftOperandStrategy: fc.Arbitrary<LeftOperand>,
  rightOperandStrategy: fc.Arbitrary<RightOperand>
): fc.Arbitrary<Output> {
  return fc.tuple(
    leftOperandStrategy,
    Object.is(operator, BinaryDivisionOperator)
      ? fc.tuple(rightOperandStrategy, nonCommentFillerStrategy).map(
        ([divisor, nonCommentFiller]) => toValidDivisor(
          divisor, nonCommentFiller
        )
      )
      : rightOperandStrategy,
    substringPositionStrategy,
    fillerArrayStrategy
  ).map(
    ([left, right, operatorPosition, operatorFillers]) => (
      factory(left, right, operator, operatorPosition, operatorFillers)
    )
  )
}

function isCommentFiller (filler: Filler) {
  return [FillerKind.COMMENT_BLOCK, FillerKind.COMMENT_LINE].includes(
    filler.content.kind
  )
}

function toValidDivisor<Divisor extends Expression> (
  divisor: Divisor, nonCommentFiller: Filler
): Divisor {
  switch (divisor.constructor) {
    case AnnotatedIdentifier: {
      const annotatedIdentifier = divisor as AnnotatedIdentifier
      const originalIdentifier = annotatedIdentifier.identifier
      const identifier = toValidDivisor(originalIdentifier, nonCommentFiller)
      return Object.is(identifier, originalIdentifier)
        ? divisor
        : new AnnotatedIdentifier(
          identifier,
          annotatedIdentifier.annotation,
          annotatedIdentifier.operatorPosition,
          annotatedIdentifier.operatorFillers
        ) as Divisor
    }
    case Assignment: {
      const assignment = divisor as Assignment
      const originalTarget = assignment.target as Expression
      const target = toValidDivisor(originalTarget, nonCommentFiller)
      return Object.is(target, originalTarget)
        ? divisor
        : new Assignment(
          target,
          assignment.value,
          assignment.operatorPosition,
          assignment.operatorFillers
        ) as Divisor
    }
    case BidirectionalConditional: {
      const bidirectionalConditional = divisor as BidirectionalConditional
      return (
        bidirectionalConditional.antecedentOpenerFillers.length === 0
        || !isCommentFiller(
          bidirectionalConditional.antecedentOpenerFillers[0]!
        )
      )
        ? divisor
        : new BidirectionalConditional(
          bidirectionalConditional.antecedent,
          bidirectionalConditional.consequent,
          bidirectionalConditional.alternative,
          bidirectionalConditional.antecedentOpenerPosition,
          bidirectionalConditional.alternativeOpenerPosition,
          [
            nonCommentFiller,
            ...bidirectionalConditional.antecedentOpenerFillers
          ],
          bidirectionalConditional.alternativeOpenerFillers
        ) as Divisor
    }
    case BinaryArithmeticOperation: {
      const binaryArithmeticOperation = divisor as BinaryArithmeticOperation
      const originalLeft = binaryArithmeticOperation.left as Expression
      const left = toValidDivisor(originalLeft, nonCommentFiller)
      return Object.is(left, originalLeft)
        ? divisor
        : new BinaryArithmeticOperation(
          left,
          binaryArithmeticOperation.right,
          binaryArithmeticOperation.operator,
          binaryArithmeticOperation.operatorPosition,
          binaryArithmeticOperation.operatorFillers
        ) as Divisor
    }
    case BinaryComparison: {
      const binaryComparison = divisor as BinaryComparison
      const originalLeft = binaryComparison.left as Expression
      const left = toValidDivisor(originalLeft, nonCommentFiller)
      return Object.is(left, originalLeft)
        ? divisor
        : new BinaryComparison(
          left,
          binaryComparison.right,
          binaryComparison.operator,
          binaryComparison.operatorPosition,
          binaryComparison.operatorFillers
        ) as Divisor
    }
    case Block: {
      const block = divisor as Block
      return (
        block.openBraceFillers.length === 0
        || !isCommentFiller(block.openBraceFillers[0]!)
      )
        ? divisor
        : new Block(
          block.statements,
          block.expression,
          block.openBracePosition,
          block.closeBracePosition,
          [nonCommentFiller, ...block.openBraceFillers],
          block.closeBraceFillers
        ) as Divisor
    }
    case Call: {
      const call = divisor as Call
      const originalCallable = call.callable as Expression
      const callable = toValidDivisor(originalCallable, nonCommentFiller)
      return Object.is(callable, originalCallable)
        ? divisor
        : new Call(
          callable,
          call.arguments_,
          call.openParenthesisPosition,
          call.commaPositions,
          call.closeParenthesisPosition,
          call.openParenthesisFillers,
          call.commaFillers,
          call.closeParenthesisFillers
        ) as Divisor
    }
    case FunctionDefinition: {
      const functionDefinition = divisor as FunctionDefinition
      return (
        functionDefinition.openerFillers.length === 0
        || !isCommentFiller(functionDefinition.openerFillers[0]!)
      )
        ? divisor
        : new FunctionDefinition(
          functionDefinition.parameters,
          functionDefinition.returnType,
          functionDefinition.body,
          functionDefinition.openerPosition,
          functionDefinition.openParenthesisPosition,
          functionDefinition.commaPositions,
          functionDefinition.closeParenthesisPosition,
          functionDefinition.arrowPosition,
          [nonCommentFiller, ...functionDefinition.openerFillers],
          functionDefinition.openParenthesisFillers,
          functionDefinition.commaFillers,
          functionDefinition.closeParenthesisFillers,
          functionDefinition.arrowFillers
        ) as Divisor
    }
    case Grouping: {
      const grouping = divisor as Grouping
      return (
        grouping.openParenthesisFillers.length === 0
        || !isCommentFiller(grouping.openParenthesisFillers[0]!)
      )
        ? divisor
        : new Grouping(
          grouping.expression,
          grouping.openParenthesisPosition,
          grouping.closeParenthesisPosition,
          [nonCommentFiller, ...grouping.openParenthesisFillers],
          grouping.closeParenthesisFillers
        ) as Divisor
    }
    case Identifier: {
      const identifier = divisor as Identifier
      return (
        identifier.fillers.length === 0
        || !isCommentFiller(identifier.fillers[0]!)
      )
        ? divisor
        : new Identifier(
          identifier.string,
          identifier.position,
          [nonCommentFiller, ...identifier.fillers]
        ) as Divisor
    }
    case MemberAccess: {
      const memberAccess = divisor as MemberAccess
      const originalObject = memberAccess.object as Expression
      const object = toValidDivisor(originalObject, nonCommentFiller)
      return Object.is(object, originalObject)
        ? divisor
        : new MemberAccess(
          object,
          memberAccess.member,
          memberAccess.operatorPosition,
          memberAccess.operatorFillers
        ) as Divisor
    }
    case NumericLiteral: {
      const numericLiteral = divisor as NumericLiteral
      return (
        numericLiteral.fillers.length === 0
        || !isCommentFiller(numericLiteral.fillers[0]!)
      )
        ? divisor
        : new NumericLiteral(
          numericLiteral.value,
          numericLiteral.type_,
          numericLiteral.position,
          [nonCommentFiller, ...numericLiteral.fillers]
        ) as Divisor
    }
    case Return: {
      const return_ = divisor as Return
      return (
        return_.operatorFillers.length === 0
        || !isCommentFiller(return_.operatorFillers[0]!)
      )
        ? divisor
        : new Return(
          return_.expression,
          return_.operatorPosition,
          [nonCommentFiller, ...return_.operatorFillers]
        ) as Divisor
    }
    case Tuple: {
      const tuple = divisor as Tuple
      return (
        tuple.openParenthesisFillers.length === 0
        || !isCommentFiller(tuple.openParenthesisFillers[0]!)
      )
        ? divisor
        : new Tuple(
          tuple.elements,
          tuple.openParenthesisPosition,
          tuple.commaPositions,
          tuple.closeParenthesisPosition,
          [nonCommentFiller, ...tuple.openParenthesisFillers],
          tuple.commaFillers,
          tuple.closeParenthesisFillers
        ) as Divisor
    }
    case UnaryArithmeticOperation: {
      const unaryArithmeticOperation = divisor as UnaryArithmeticOperation
      return (
        unaryArithmeticOperation.operatorFillers.length === 0
        || !isCommentFiller(unaryArithmeticOperation.operatorFillers[0]!)
      )
        ? divisor
        : new UnaryArithmeticOperation(
          unaryArithmeticOperation.operand,
          unaryArithmeticOperation.operator,
          unaryArithmeticOperation.operatorPosition,
          [nonCommentFiller, ...unaryArithmeticOperation.operatorFillers]
        ) as Divisor
    }
    case UnidirectionalConditional: {
      const unidirectionalConditional = divisor as UnidirectionalConditional
      return (
        unidirectionalConditional.openerFillers.length === 0
        || !isCommentFiller(unidirectionalConditional.openerFillers[0]!)
      )
        ? divisor
        : new UnidirectionalConditional(
          unidirectionalConditional.antecedent,
          unidirectionalConditional.consequent,
          unidirectionalConditional.openerPosition,
          [nonCommentFiller, ...unidirectionalConditional.openerFillers]
        ) as Divisor
    }
    case WhileLoop: {
      const whileLoop = divisor as WhileLoop
      return (
        whileLoop.openerFillers.length === 0
        || !isCommentFiller(whileLoop.openerFillers[0]!)
      )
        ? divisor
        : new UnidirectionalConditional(
          whileLoop.condition,
          whileLoop.body,
          whileLoop.openerPosition,
          [nonCommentFiller, ...whileLoop.openerFillers]
        ) as Divisor
    }
    default:
      throw new TypeError(`Unsupported divisor: ${divisor.toString()}`)
  }
}

export function toBinaryArithmeticOperationDataStrategy<
  LeftOperand extends Expression, RightOperand extends Expression
> (
  operator: BinaryArithmeticOperator,
  leftOperandStrategy: fc.Arbitrary<LeftOperand>,
  rightOperandStrategy: fc.Arbitrary<RightOperand>
): fc.Arbitrary<BinaryArithmeticOperationData<LeftOperand, RightOperand>> {
  return binaryArithmeticOperationArgumentsToStrategy(
    (left, right, operator, operatorPosition, operatorFillers) => (
      { left, right, operator, operatorPosition, operatorFillers }
    ),
    operator,
    leftOperandStrategy,
    rightOperandStrategy
  )
}

export function toBinaryArithmeticOperationStrategy<
  LeftOperand extends Expression, RightOperand extends Expression
> (
  operator: BinaryArithmeticOperator,
  leftOperandStrategy: fc.Arbitrary<LeftOperand>,
  rightOperandStrategy: fc.Arbitrary<RightOperand>
) {
  return binaryArithmeticOperationArgumentsToStrategy(
    (left, right, operator, operatorPosition, operatorFillers) => {
      const result = new BinaryArithmeticOperation(
        left, right, operator, operatorPosition, operatorFillers
      )
      result.validateContents()
      return result
    },
    operator,
    leftOperandStrategy,
    rightOperandStrategy
  )
}
