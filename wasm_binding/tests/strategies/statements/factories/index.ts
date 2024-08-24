import fc from 'fast-check'
import { Expression } from '../../../types'
import {
  toExpressionStatementDataStrategy, toExpressionStatementStrategy
} from './expression_statement'

export { toExpressionStatementDataStrategy, toExpressionStatementStrategy }

export function toStatementStrategy (
  expressionStrategy: fc.Arbitrary<Expression>
) {
  return toExpressionStatementStrategy(expressionStrategy)
}

export function toStatementArrayStrategy (
  expressionStrategy: fc.Arbitrary<Expression>
) {
  return fc.array(toStatementStrategy(expressionStrategy), { maxLength: 4 })
}
