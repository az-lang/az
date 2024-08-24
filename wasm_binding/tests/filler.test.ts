import fc from 'fast-check'
import {
  ByteCount,
  CharacterPosition,
  Filler,
  FillerContent,
  FillerKind,
  SubstringPosition,
  Utf8Count
} from '../pkg'
import { fillerDataStrategy, fillerStrategy } from './strategies'
import { evalInContext, testEquivalenceOfEqualTo } from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            fillerDataStrategy,
            ({ content, position }) => (
              new Filler(content, position) instanceof Filler
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
            fillerDataStrategy,
            ({ content, position }) => (
              new Filler(content, position).equalTo(
                new Filler(content, position)
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
            fillerStrategy,
            (value: Filler) => (
              new Filler(value.content, value.position).equalTo(value)
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
    testEquivalenceOfEqualTo(fillerStrategy)
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
            fillerStrategy,
            (value: Filler) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            fillerStrategy,
            (value: Filler) => {
              const result = Filler.fromJSON(value.toJSON())

              return result instanceof Filler && result.equalTo(value)
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
            fillerStrategy, (value) => typeof value.toString() === typeof ''
          )
        )
      }
    )
    test(
      '`eval` round-trip',
      () => {
        fc.assert(
          fc.property(
            fillerStrategy,
            (value: Filler) => {
              const result = evalInContext(
                value.toString(),
                {
                  ByteCount,
                  CharacterPosition,
                  Filler,
                  FillerContent,
                  FillerKind,
                  SubstringPosition,
                  Utf8Count
                }
              )

              return result instanceof Filler && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)
