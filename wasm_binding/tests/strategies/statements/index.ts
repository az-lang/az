import { expressionStrategy } from '../expressions'
import { toStatementArrayStrategy } from './factories'

export {
  expressionStatementDataStrategy, expressionStatementStrategy
} from './expression_statement'

export const statementArrayStrategy = toStatementArrayStrategy(
  expressionStrategy
)
