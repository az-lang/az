import fc from 'fast-check'
import { Filler, Script } from '../../pkg'
import { Statement } from '../types'
import { fillerArrayStrategy } from './filler_array'
import { statementArrayStrategy } from './statements'

function scriptArgumentsToStrategy<Output> (
  factory: (statements: Statement[], fillers: Filler[]) => Output
): fc.Arbitrary<Output> {
  return fc.tuple(statementArrayStrategy, fillerArrayStrategy).map(
    ([statements, fillers]) => factory(statements, fillers)
  )
}

export const scriptDataStrategy = scriptArgumentsToStrategy(
  (statements, fillers) => ({ statements, fillers })
)
export const scriptStrategy = scriptArgumentsToStrategy(
  (statements, fillers) => {
    const result = new Script(statements, fillers)
    result.validateContents()
    return result
  }
)
