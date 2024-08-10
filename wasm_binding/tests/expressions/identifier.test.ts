import fc from 'fast-check'
import {
  ByteCount,
  CharacterPosition,
  Filler,
  FillerContent,
  FillerKind,
  Identifier,
  SubstringPosition,
  Utf8Count
} from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import { identifierDataStrategy, identifierStrategy } from './strategies'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            identifierDataStrategy,
            ({ string, position, fillers }) => (
              new Identifier(string, position, fillers) instanceof Identifier
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
            identifierDataStrategy,
            ({ string, position, fillers }) => (
              new Identifier(string, position, fillers).equalTo(
                new Identifier(string, position, fillers)
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
            identifierStrategy,
            (value: Identifier) => (
              new Identifier(
                value.string, value.position, value.fillers
              ) instanceof Identifier
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
    testEquivalenceOfEqualTo(identifierStrategy)
  }
)

describe(
  '`toJSON` tests',
  () => {
    test(
      'basic', () => {
        fc.assert(
          fc.property(
            identifierStrategy,
            (value: Identifier) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            identifierStrategy,
            (value: Identifier) => {
              const result = Identifier.fromJSON(value.toJSON())

              return result instanceof Identifier && result.equalTo(value)
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
            identifierStrategy,
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
            identifierStrategy,
            (value: Identifier) => {
              const result = evalInContext(
                value.toString(),
                {
                  ByteCount,
                  CharacterPosition,
                  Filler,
                  FillerContent,
                  FillerKind,
                  Identifier,
                  SubstringPosition,
                  Utf8Count
                }
              )

              return result instanceof Identifier && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)
