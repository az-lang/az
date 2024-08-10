import fc from 'fast-check'
import { BinaryArithmeticOperation } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import {
  binaryArithmeticOperationDataStrategy, binaryArithmeticOperationStrategy
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
            binaryArithmeticOperationDataStrategy,
            ({ left, right, operator, operatorPosition, operatorFillers }) => (
              new BinaryArithmeticOperation(
                left, right, operator, operatorPosition, operatorFillers
              ) instanceof BinaryArithmeticOperation
            )
          )
        )
      })
    test(
      'repeated',
      () => {
        fc.assert(
          fc.property(
            binaryArithmeticOperationDataStrategy,
            ({ left, right, operator, operatorPosition, operatorFillers }) => (
              new BinaryArithmeticOperation(
                left, right, operator, operatorPosition, operatorFillers
              ).equalTo(
                new BinaryArithmeticOperation(
                  left, right, operator, operatorPosition, operatorFillers
                )
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
            binaryArithmeticOperationStrategy,
            (value: BinaryArithmeticOperation) => (
              new BinaryArithmeticOperation(
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
  }
)

describe(
  '`equalTo` tests',
  () => {
    testEquivalenceOfEqualTo(binaryArithmeticOperationStrategy)
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
            binaryArithmeticOperationStrategy,
            (value: BinaryArithmeticOperation) => (
              typeof value.toJSON() === typeof {}
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
            binaryArithmeticOperationStrategy,
            (value: BinaryArithmeticOperation) => {
              const result = BinaryArithmeticOperation.fromJSON(value.toJSON())

              return (
                result instanceof BinaryArithmeticOperation
                && result.equalTo(value)
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
            binaryArithmeticOperationStrategy,
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
            binaryArithmeticOperationStrategy,
            (value: BinaryArithmeticOperation) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return (
                result instanceof BinaryArithmeticOperation
                && result.equalTo(value)
              )
            }
          )
        )
      }
    )
  }
)
