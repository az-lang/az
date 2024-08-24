import {
  AnnotatedIdentifier,
  Assignment,
  BidirectionalConditional,
  BinaryArithmeticOperation,
  BinaryComparison,
  Block,
  Call,
  Filler,
  FunctionDefinition,
  Grouping,
  Identifier,
  MemberAccess,
  NumericLiteral,
  Return,
  Tuple,
  UnaryArithmeticOperation,
  UnidirectionalConditional,
  WhileLoop
} from '../../../../pkg'
import { Expression } from '../../../types'

export function toNonLexicallyConflictingExpression<
  ExpressionT extends Expression
> (expression: ExpressionT, nonCommentFiller: Filler): ExpressionT {
  switch (expression.constructor) {
    case AnnotatedIdentifier: {
      const annotatedIdentifier = expression as AnnotatedIdentifier
      const originalIdentifier = annotatedIdentifier.identifier
      const identifier = toNonLexicallyConflictingExpression(
        originalIdentifier, nonCommentFiller
      )
      return Object.is(identifier, originalIdentifier)
        ? expression
        : new AnnotatedIdentifier(
          identifier,
          annotatedIdentifier.annotation,
          annotatedIdentifier.operatorPosition,
          annotatedIdentifier.operatorFillers
        ) as ExpressionT
    }
    case Assignment: {
      const assignment = expression as Assignment
      const originalTarget = assignment.target as Expression
      const target = toNonLexicallyConflictingExpression(
        originalTarget, nonCommentFiller
      )
      return Object.is(target, originalTarget)
        ? expression
        : new Assignment(
          target,
          assignment.value,
          assignment.operatorPosition,
          assignment.operatorFillers
        ) as ExpressionT
    }
    case BidirectionalConditional: {
      const bidirectionalConditional = expression as BidirectionalConditional
      return (bidirectionalConditional.antecedentOpenerFillers.length !== 0)
        ? expression
        : new BidirectionalConditional(
          bidirectionalConditional.antecedent,
          bidirectionalConditional.consequent,
          bidirectionalConditional.alternative,
          bidirectionalConditional.antecedentOpenerPosition,
          bidirectionalConditional.alternativeOpenerPosition,
          [nonCommentFiller],
          bidirectionalConditional.alternativeOpenerFillers
        ) as ExpressionT
    }
    case BinaryArithmeticOperation: {
      const binaryArithmeticOperation = expression as BinaryArithmeticOperation
      const originalLeft = binaryArithmeticOperation.left as Expression
      const left = toNonLexicallyConflictingExpression(
        originalLeft, nonCommentFiller
      )
      return Object.is(left, originalLeft)
        ? expression
        : new BinaryArithmeticOperation(
          left,
          binaryArithmeticOperation.right,
          binaryArithmeticOperation.operator,
          binaryArithmeticOperation.operatorPosition,
          binaryArithmeticOperation.operatorFillers
        ) as ExpressionT
    }
    case BinaryComparison: {
      const binaryComparison = expression as BinaryComparison
      const originalLeft = binaryComparison.left as Expression
      const left = toNonLexicallyConflictingExpression(
        originalLeft, nonCommentFiller
      )
      return Object.is(left, originalLeft)
        ? expression
        : new BinaryComparison(
          left,
          binaryComparison.right,
          binaryComparison.operator,
          binaryComparison.operatorPosition,
          binaryComparison.operatorFillers
        ) as ExpressionT
    }
    case Block:
      return expression
    case Call: {
      const call = expression as Call
      const originalCallable = call.callable as Expression
      const callable = toNonLexicallyConflictingExpression(
        originalCallable, nonCommentFiller
      )
      return Object.is(callable, originalCallable)
        ? expression
        : new Call(
          callable,
          call.arguments_,
          call.openParenthesisPosition,
          call.commaPositions,
          call.closeParenthesisPosition,
          call.openParenthesisFillers,
          call.commaFillers,
          call.closeParenthesisFillers
        ) as ExpressionT
    }
    case FunctionDefinition: {
      const functionDefinition = expression as FunctionDefinition
      return (functionDefinition.openerFillers.length !== 0)
        ? expression
        : new FunctionDefinition(
          functionDefinition.parameters,
          functionDefinition.returnType,
          functionDefinition.body,
          functionDefinition.openerPosition,
          functionDefinition.openParenthesisPosition,
          functionDefinition.commaPositions,
          functionDefinition.closeParenthesisPosition,
          functionDefinition.arrowPosition,
          [nonCommentFiller],
          functionDefinition.openParenthesisFillers,
          functionDefinition.commaFillers,
          functionDefinition.closeParenthesisFillers,
          functionDefinition.arrowFillers
        ) as ExpressionT
    }
    case Grouping:
      return expression
    case Identifier: {
      const identifier = expression as Identifier
      return (identifier.fillers.length !== 0)
        ? expression
        : new Identifier(
          identifier.string, identifier.position, [nonCommentFiller]
        ) as ExpressionT
    }
    case MemberAccess: {
      const memberAccess = expression as MemberAccess
      const originalObject = memberAccess.object as Expression
      const object = toNonLexicallyConflictingExpression(
        originalObject, nonCommentFiller
      )
      return Object.is(object, originalObject)
        ? expression
        : new MemberAccess(
          object,
          memberAccess.member,
          memberAccess.operatorPosition,
          memberAccess.operatorFillers
        ) as ExpressionT
    }
    case NumericLiteral: {
      const numericLiteral = expression as NumericLiteral
      return (numericLiteral.fillers.length !== 0)
        ? expression
        : new NumericLiteral(
          numericLiteral.value,
          numericLiteral.type_,
          numericLiteral.position,
          [nonCommentFiller]
        ) as ExpressionT
    }
    case Return: {
      const return_ = expression as Return
      return (return_.operatorFillers.length !== 0)
        ? expression
        : new Return(
          return_.expression, return_.operatorPosition, [nonCommentFiller]
        ) as ExpressionT
    }
    case Tuple:
      return expression
    case UnaryArithmeticOperation:
      return expression
    case UnidirectionalConditional: {
      const unidirectionalConditional = expression as UnidirectionalConditional
      return (unidirectionalConditional.openerFillers.length !== 0)
        ? expression
        : new UnidirectionalConditional(
          unidirectionalConditional.antecedent,
          unidirectionalConditional.consequent,
          unidirectionalConditional.openerPosition,
          [nonCommentFiller]
        ) as ExpressionT
    }
    case WhileLoop: {
      const whileLoop = expression as WhileLoop
      return (whileLoop.openerFillers.length !== 0)
        ? expression
        : new UnidirectionalConditional(
          whileLoop.condition,
          whileLoop.body,
          whileLoop.openerPosition,
          [nonCommentFiller]
        ) as ExpressionT
    }
    default:
      throw new TypeError(`Unsupported expression: ${expression.toString()}`)
  }
}
