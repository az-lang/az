import fc from 'fast-check'
import {
  ByteCount,
  CharacterPosition,
  Filler,
  FillerContent,
  FillerKind,
  NumericLiteral,
  NumericLiteralType,
  SubstringPosition,
  Utf8Count
} from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import {
  numericLiteralDataStrategy, numericLiteralStrategy
} from './strategies'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            numericLiteralDataStrategy,
            ({ value, type_, position, fillers }) => (
              new NumericLiteral(
                value, type_, position, fillers
              ) instanceof NumericLiteral
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
            numericLiteralDataStrategy,
            ({ value, type_, position, fillers }) => (
              new NumericLiteral(
                value, type_, position, fillers
              ).equalTo(new NumericLiteral(value, type_, position, fillers))
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
            numericLiteralStrategy,
            (value: NumericLiteral) => (
              new NumericLiteral(
                value.value, value.type_, value.position, value.fillers
              ).equalTo(value)
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
    testEquivalenceOfEqualTo(numericLiteralStrategy)
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
            numericLiteralStrategy,
            (value: NumericLiteral) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            numericLiteralStrategy,
            (value: NumericLiteral) => {
              const result = NumericLiteral.fromJSON(value.toJSON())

              return result instanceof NumericLiteral && result.equalTo(value)
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
            numericLiteralStrategy,
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
            numericLiteralStrategy,
            (value: NumericLiteral) => {
              const result = evalInContext(
                value.toString(),
                {
                  ByteCount,
                  CharacterPosition,
                  Filler,
                  FillerContent,
                  FillerKind,
                  NumericLiteral,
                  NumericLiteralType,
                  SubstringPosition,
                  Utf8Count
                }
              )

              return result instanceof NumericLiteral && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)
