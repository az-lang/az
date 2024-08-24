import fc from 'fast-check'
import { Filler, Return, SubstringPosition } from '../../../../pkg'
import { Expression, ReturnData } from '../../../types'
import { nonCommentFillerStrategy } from '../../filler'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'
import { toNonLexicallyConflictingExpression } from './utils'

function returnArgumentsToStrategy<
  ExpressionT extends Expression, Output
> (
  factory: (
    expression: ExpressionT,
    operatorPosition: SubstringPosition,
    operatorFillers: Filler[]
  ) => Output,
  expressionStrategy: fc.Arbitrary<ExpressionT>
): fc.Arbitrary<Output> {
  return fc.tuple(
    fc.tuple(
      expressionStrategy, nonCommentFillerStrategy
    ).map(
      ([expression, nonCommentFiller]) => (
        toNonLexicallyConflictingExpression(expression, nonCommentFiller)
      )
    ),
    substringPositionStrategy,
    fillerArrayStrategy
  ).map(
    ([expression, operatorPosition, operatorFillers]) => (
      factory(expression, operatorPosition, operatorFillers)
    )
  )
}

export function toReturnDataStrategy<
  ExpressionT extends Expression
> (expressionStrategy: fc.Arbitrary<ExpressionT>): fc.Arbitrary<
  ReturnData<ExpressionT>
> {
  return returnArgumentsToStrategy(
    (expression, operatorPosition, operatorFillers) => (
      { expression, operatorPosition, operatorFillers }
    ),
    expressionStrategy
  )
}

export function toReturnStrategy<
  ExpressionT extends Expression
> (expressionStrategy: fc.Arbitrary<ExpressionT>) {
  return returnArgumentsToStrategy(
    (expression, operatorPosition, operatorFillers) => {
      const result = new Return(expression, operatorPosition, operatorFillers)
      result.validateContents()
      return result
    },
    expressionStrategy
  )
}
