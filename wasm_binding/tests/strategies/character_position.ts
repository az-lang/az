import fc from 'fast-check'
import { ByteCount, CharacterPosition, Utf8Count } from '../../pkg'
import { byteCountStrategy } from './byte_count'
import { utf8CountStrategy } from './utf_8_count'

function characterPositionArgumentsToStrategy<Output> (
  factory: (byte: ByteCount, utf8: Utf8Count) => Output
): fc.Arbitrary<Output> {
  return fc.tuple(byteCountStrategy, utf8CountStrategy).map(
    ([byte, utf8]) => factory(byte, utf8)
  )
}

export const characterPositionDataStrategy = (
  characterPositionArgumentsToStrategy((byte, utf8) => ({ byte, utf8 }))
)
export const characterPositionStrategy = characterPositionArgumentsToStrategy(
  (byte, utf8) => new CharacterPosition(byte, utf8)
)
