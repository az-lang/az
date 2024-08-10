import fc from 'fast-check'
import { ExpressionStatement } from '../../pkg'
import { INTERNAL_NODE_EXPRESSION_CONTEXT } from '../expressions/utils'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import {
  expressionStatementDataStrategy, expressionStatementStrategy
} from './strategies'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            expressionStatementDataStrategy,
            ({ expression, semicolonPosition, semicolonFillers }) => (
              new ExpressionStatement(
                expression, semicolonPosition, semicolonFillers
              ) instanceof ExpressionStatement
            )
          )
        )
      }
    )
    test(
      'repeated',
      () => {
        fc.assert(
          fc.property(
            expressionStatementDataStrategy,
            ({ expression, semicolonPosition, semicolonFillers }) => (
              new ExpressionStatement(
                expression, semicolonPosition, semicolonFillers
              ).equalTo(
                new ExpressionStatement(
                  expression, semicolonPosition, semicolonFillers
                )
              )
            )
          )
        )
      }
    )
    test(
      'round-trip',
      () => {
        fc.assert(
          fc.property(
            expressionStatementStrategy,
            (value: ExpressionStatement) => (
              new ExpressionStatement(
                value.expression,
                value.semicolonPosition,
                value.semicolonFillers
              ).equalTo(value)
            )
          )
        )
      }
    )
  }
)

describe(
  '`equalTo` tests',
  () => {
    testEquivalenceOfEqualTo(expressionStatementStrategy)
  }
)

describe(
  '`toJSON` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            expressionStatementStrategy,
            (value: ExpressionStatement) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            expressionStatementStrategy,
            (value: ExpressionStatement) => {
              const result = ExpressionStatement.fromJSON(value.toJSON())

              return (
                result instanceof ExpressionStatement && result.equalTo(value)
              )
            }
          )
        )
      }
    )
  }
)

describe(
  '`toString` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            expressionStatementStrategy,
            (value) => typeof value.toString() === typeof ''
          )
        )
      }
    )
    test(
      '`eval` round-trip',
      () => {
        fc.assert(
          fc.property(
            expressionStatementStrategy,
            (value: ExpressionStatement) => {
              const result = evalInContext(
                value.toString(),
                { ...INTERNAL_NODE_EXPRESSION_CONTEXT, ExpressionStatement }
              )

              return (
                result instanceof ExpressionStatement && result.equalTo(value)
              )
            }
          )
        )
      }
    )
  }
)
