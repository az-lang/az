import fc from 'fast-check'
import { Filler, Identifier, SubstringPosition, isKeyword } from '../../../pkg'
import { fillerArrayStrategy } from '../filler_array'
import { substringPositionStrategy } from '../substring_position'
import { identifierStringStrategy } from '../token_content_state'

function identifierArgumentsToStrategy<Output> (
  factory: (
    string: string, position: SubstringPosition, fillers: Filler[]
  ) => Output
): fc.Arbitrary<Output> {
  return fc.tuple(
    identifierStringStrategy.filter((candidate) => !isKeyword(candidate)),
    substringPositionStrategy,
    fillerArrayStrategy
  ).map(([string, position, fillers]) => factory(string, position, fillers))
}

export const identifierDataStrategy = identifierArgumentsToStrategy(
  (string, position, fillers) => ({ string, position, fillers })
)
export const identifierStrategy = identifierArgumentsToStrategy(
  (string, position, fillers) => new Identifier(string, position, fillers)
)
