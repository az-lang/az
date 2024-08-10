import fc from 'fast-check'
import {
  ExpressionStatement, Filler, SubstringPosition
} from '../../../../pkg'
import { Expression } from '../../../types'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'

function expressionStatementArgumentsToStrategy<Output> (
  factory: (
    expression: Expression,
    semicolonPosition: SubstringPosition,
    semicolonFillers: Filler[]
  ) => Output,
  expressionStrategy: fc.Arbitrary<Expression>
) {
  return fc.tuple(
    expressionStrategy, substringPositionStrategy, fillerArrayStrategy
  ).map(
    ([expression, semicolonPosition, semicolonFillers]) => (
      factory(expression, semicolonPosition, semicolonFillers)
    )
  )
}

export function toExpressionStatementDataStrategy (
  expressionStrategy: fc.Arbitrary<Expression>
) {
  return expressionStatementArgumentsToStrategy(
    (expression, semicolonPosition, semicolonFillers) => (
      { expression, semicolonPosition, semicolonFillers }
    ),
    expressionStrategy
  )
}

export function toExpressionStatementStrategy (
  expressionStrategy: fc.Arbitrary<Expression>
) {
  return expressionStatementArgumentsToStrategy(
    (expression, semicolonPosition, semicolonFillers) => {
      const result = new ExpressionStatement(
        expression, semicolonPosition, semicolonFillers
      )
      result.validateContents()
      return result
    },
    expressionStrategy
  )
}
