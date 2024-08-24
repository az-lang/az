import fc from 'fast-check'
import { BinaryComparison } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import {
  binaryComparisonDataStrategy, binaryComparisonStrategy
} from './strategies'
import { INTERNAL_NODE_EXPRESSION_CONTEXT } from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            binaryComparisonDataStrategy,
            ({ left, right, operator, operatorPosition, operatorFillers }) => (
              new BinaryComparison(
                left, right, operator, operatorPosition, operatorFillers
              ) instanceof BinaryComparison
            )
          )
        )
      })
    test(
      'repeated',
      () => {
        fc.assert(
          fc.property(
            binaryComparisonDataStrategy,
            ({ left, right, operator, operatorPosition, operatorFillers }) => (
              new BinaryComparison(
                left, right, operator, operatorPosition, operatorFillers
              ).equalTo(
                new BinaryComparison(
                  left, right, operator, operatorPosition, operatorFillers
                )
              )
            )
          )
        )
      })
    fc.assert(
      fc.property(
        binaryComparisonStrategy,
        (value: BinaryComparison) => (
          new BinaryComparison(
            value.left,
            value.right,
            value.operator,
            value.operatorPosition,
            value.operatorFillers
          ).equalTo(value)
        )
      )
    )
  }
)

describe(
  '`equalTo` tests',
  () => {
    testEquivalenceOfEqualTo(binaryComparisonStrategy)
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
            binaryComparisonStrategy,
            (value: BinaryComparison) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            binaryComparisonStrategy,
            (value: BinaryComparison) => {
              const result = BinaryComparison.fromJSON(value.toJSON())

              return (
                result instanceof BinaryComparison && result.equalTo(value)
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
            binaryComparisonStrategy,
            (value) => typeof value.toString() === typeof ''
          )
        )
      }
    )
    test(
      '`eval` round-trip', () => {
        fc.assert(
          fc.property(
            binaryComparisonStrategy,
            (value: BinaryComparison) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return (
                result instanceof BinaryComparison && result.equalTo(value)
              )
            }
          )
        )
      }
    )
  }
)
