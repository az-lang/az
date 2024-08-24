import fc from 'fast-check'
import { Return } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import { returnDataStrategy, returnStrategy } from './strategies'
import { INTERNAL_NODE_EXPRESSION_CONTEXT } from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            returnDataStrategy,
            ({ expression, operatorPosition, operatorFillers }) => (
              new Return(
                expression, operatorPosition, operatorFillers
              ) instanceof Return
            )
          )
        )
      })
    test(
      'repeated',
      () => {
        fc.assert(
          fc.property(
            returnDataStrategy,
            ({ expression, operatorPosition, operatorFillers }) => (
              new Return(
                expression, operatorPosition, operatorFillers
              ).equalTo(
                new Return(expression, operatorPosition, operatorFillers)
              )
            )
          )
        )
      })
    test(
      'round-trip',
      () => {
        fc.assert(
          fc.property(
            returnStrategy,
            (value: Return) => (
              new Return(
                value.expression, value.operatorPosition, value.operatorFillers
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
    testEquivalenceOfEqualTo(returnStrategy)
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
            returnStrategy,
            (value: Return) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            returnStrategy,
            (value: Return) => {
              const result = Return.fromJSON(value.toJSON())

              return result instanceof Return && result.equalTo(value)
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
            returnStrategy, (value) => typeof value.toString() === typeof ''
          )
        )
      }
    )
    test(
      '`eval` round-trip',
      () => {
        fc.assert(
          fc.property(
            returnStrategy,
            (value: Return) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return result instanceof Return && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)
