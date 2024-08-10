import fc from 'fast-check'
import { UnaryArithmeticOperation } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import {
  unaryArithmeticOperationDataStrategy, unaryArithmeticOperationStrategy
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
            unaryArithmeticOperationDataStrategy,
            ({ operand, operator, operatorPosition, operatorFillers }) => (
              new UnaryArithmeticOperation(
                operand, operator, operatorPosition, operatorFillers
              ) instanceof UnaryArithmeticOperation
            )
          )
        )
      })
    test(
      'repeated',
      () => {
        fc.assert(
          fc.property(
            unaryArithmeticOperationDataStrategy,
            ({ operand, operator, operatorPosition, operatorFillers }) => (
              new UnaryArithmeticOperation(
                operand, operator, operatorPosition, operatorFillers
              ).equalTo(
                new UnaryArithmeticOperation(
                  operand, operator, operatorPosition, operatorFillers
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
            unaryArithmeticOperationStrategy,
            (value: UnaryArithmeticOperation) => (
              new UnaryArithmeticOperation(
                value.operand,
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
    testEquivalenceOfEqualTo(unaryArithmeticOperationStrategy)
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
            unaryArithmeticOperationStrategy,
            (value: UnaryArithmeticOperation) => (
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
            unaryArithmeticOperationStrategy,
            (value: UnaryArithmeticOperation) => {
              const result = UnaryArithmeticOperation.fromJSON(value.toJSON())

              return (
                result instanceof UnaryArithmeticOperation
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
            unaryArithmeticOperationStrategy,
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
            unaryArithmeticOperationStrategy,
            (value: UnaryArithmeticOperation) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return (
                result instanceof UnaryArithmeticOperation
                && result.equalTo(value)
              )
            }
          )
        )
      }
    )
  }
)
