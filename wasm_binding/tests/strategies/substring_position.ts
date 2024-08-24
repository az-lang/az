import fc from 'fast-check'
import { CharacterPosition, SubstringPosition } from '../../pkg'
import { characterPositionStrategy } from './character_position'

function substringPositionArgumentsToStrategy<Output> (
  factory: (start: CharacterPosition, end: CharacterPosition) => Output
): fc.Arbitrary<Output> {
  return fc.tuple(
    characterPositionStrategy, characterPositionStrategy
  ).map(([start, end]) => factory(start, end))
}

export const substringPositionDataStrategy = (
  substringPositionArgumentsToStrategy((start, end) => ({ start, end }))
)
export const substringPositionStrategy = fc.tuple(
  characterPositionStrategy, characterPositionStrategy
).map(
  ([start, end]: [CharacterPosition, CharacterPosition]) => (
    new SubstringPosition(start, end)
  )
)
