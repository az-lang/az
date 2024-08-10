import { expressionStrategy } from '../expressions'
import {
  toExpressionStatementDataStrategy, toExpressionStatementStrategy
} from './factories'

export const expressionStatementDataStrategy = (
  toExpressionStatementDataStrategy(expressionStrategy)
)
export const expressionStatementStrategy = toExpressionStatementStrategy(
  expressionStrategy
)
