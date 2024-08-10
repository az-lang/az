import fc from 'fast-check'
import { Script, TokenCollection } from '../pkg'
import { INTERNAL_NODE_EXPRESSION_CONTEXT } from './expressions/utils'
import { scriptDataStrategy, scriptStrategy } from './strategies'
import { evalInContext, testEquivalenceOfEqualTo } from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            scriptDataStrategy,
            ({ statements, fillers }) => (
              new Script(statements, fillers) instanceof Script
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
            scriptDataStrategy,
            ({ statements, fillers }) => (
              new Script(statements, fillers).equalTo(
                new Script(statements, fillers)
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
            scriptStrategy,
            (value: Script) => (
              new Script(value.statements, value.fillers).equalTo(value)
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
    testEquivalenceOfEqualTo(scriptStrategy)
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
            scriptStrategy,
            (value: Script) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            scriptStrategy,
            (value: Script) => {
              const result = Script.fromJSON(value.toJSON())

              return result instanceof Script && result.equalTo(value)
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
            scriptStrategy, (value) => typeof value.toString() === typeof ''
          )
        )
      }
    )
    test(
      '`eval` round-trip',
      () => {
        fc.assert(
          fc.property(
            scriptStrategy,
            (value: Script) => {
              const result = evalInContext(
                value.toString(),
                { ...INTERNAL_NODE_EXPRESSION_CONTEXT, Script }
              )

              return result instanceof Script && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)

describe(
  '`tokenize` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            scriptStrategy,
            (value: Script) => value.tokenize() instanceof TokenCollection
          )
        )
      }
    )
    test(
      '`fromTokens` round-trip',
      () => {
        fc.assert(
          fc.property(
            scriptStrategy,
            (value: Script) => {
              const result = Script.fromTokens(value.tokenize())

              return result instanceof Script && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)

describe(
  '`format` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            scriptStrategy,
            (value: Script) => value.format() === undefined // eslint-disable-line @typescript-eslint/no-confusing-void-expression,@typescript-eslint/no-unnecessary-condition
          )
        )
      }
    )
  }
)
