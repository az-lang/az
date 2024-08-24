import fc from 'fast-check'
import { FillerContent, FillerKind } from '../pkg'
import { fillerContentDataStrategy, fillerContentStrategy } from './strategies'
import { evalInContext, testEquivalenceOfEqualTo } from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            fillerContentDataStrategy,
            ({ kind, state }) => (
              new FillerContent(kind, state) instanceof FillerContent
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
            fillerContentDataStrategy,
            ({ kind, state }) => (
              new FillerContent(kind, state).equalTo(
                new FillerContent(kind, state)
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
            fillerContentStrategy,
            (value: FillerContent) => (
              value.kind === FillerKind.NEWLINE
                ? new FillerContent(value.kind, undefined)
                : new FillerContent(value.kind, value.state)
            ).equalTo(value)
          )
        )
      }
    )
  }
)

describe(
  '`equalTo` tests',
  () => {
    testEquivalenceOfEqualTo(fillerContentStrategy)
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
            fillerContentStrategy,
            (value: FillerContent) => (
              typeof value.toJSON() === typeof (
                value.kind === FillerKind.NEWLINE ? '' : {}
              )
            )
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            fillerContentStrategy,
            (value: FillerContent) => {
              const result = FillerContent.fromJSON(value.toJSON())

              return result instanceof FillerContent && result.equalTo(value)
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
            fillerContentStrategy,
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
            fillerContentStrategy,
            (value: FillerContent) => {
              const result = evalInContext(
                value.toString(), { FillerContent, FillerKind }
              )

              return result instanceof FillerContent && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)
