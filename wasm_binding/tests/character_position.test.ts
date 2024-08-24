import fc from 'fast-check'
import { ByteCount, CharacterPosition, Utf8Count } from '../pkg'
import {
  characterPositionDataStrategy,
  characterPositionStrategy
} from './strategies'
import { evalInContext, testEquivalenceOfEqualTo } from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            characterPositionDataStrategy,
            ({ byte, utf8 }) => (
              new CharacterPosition(byte, utf8) instanceof CharacterPosition
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
            characterPositionDataStrategy,
            ({ byte, utf8 }) => (
              new CharacterPosition(byte, utf8).equalTo(
                new CharacterPosition(byte, utf8)
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
            characterPositionStrategy,
            (value: CharacterPosition) => (
              new CharacterPosition(value.byte, value.utf8).equalTo(value)
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
    testEquivalenceOfEqualTo(characterPositionStrategy)
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
            characterPositionStrategy,
            (value: CharacterPosition) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            characterPositionStrategy,
            (value: CharacterPosition) => {
              const result = CharacterPosition.fromJSON(value.toJSON())

              return (
                result instanceof CharacterPosition && result.equalTo(value)
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
            characterPositionStrategy,
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
            characterPositionStrategy,
            (value: CharacterPosition) => {
              const result = evalInContext(
                value.toString(), { ByteCount, CharacterPosition, Utf8Count }
              )

              return (
                result instanceof CharacterPosition && result.equalTo(value)
              )
            }
          )
        )
      }
    )
  }
)
