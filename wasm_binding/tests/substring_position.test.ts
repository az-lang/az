import fc from 'fast-check'
import {
  ByteCount,
  CharacterPosition,
  SubstringPosition,
  Utf8Count
} from '../pkg'
import {
  substringPositionDataStrategy,
  substringPositionStrategy
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
            substringPositionDataStrategy,
            ({ start, end }) => (
              new SubstringPosition(start, end) instanceof SubstringPosition
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
            substringPositionDataStrategy,
            ({ start, end }) => (
              new SubstringPosition(start, end).equalTo(
                new SubstringPosition(start, end)
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
            substringPositionStrategy,
            (value: SubstringPosition) => (
              new SubstringPosition(value.start, value.end).equalTo(value)
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
    testEquivalenceOfEqualTo(substringPositionStrategy)
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
            substringPositionStrategy,
            (value: SubstringPosition) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            substringPositionStrategy,
            (value: SubstringPosition) => {
              const result = SubstringPosition.fromJSON(value.toJSON())

              return (
                result instanceof SubstringPosition && result.equalTo(value)
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
            substringPositionStrategy,
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
            substringPositionStrategy,
            (value: SubstringPosition) => {
              const result = evalInContext(
                value.toString(),
                {
                  ByteCount,
                  CharacterPosition,
                  SubstringPosition,
                  Utf8Count
                }
              )

              return (
                result instanceof SubstringPosition && result.equalTo(value)
              )
            }
          )
        )
      }
    )
  }
)
