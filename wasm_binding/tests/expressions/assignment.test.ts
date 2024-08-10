import fc from 'fast-check'
import { Assignment } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import { assignmentDataStrategy, assignmentStrategy } from './strategies'
import { INTERNAL_NODE_EXPRESSION_CONTEXT } from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            assignmentDataStrategy,
            ({ target, value, operatorPosition, operatorFillers }) => (
              new Assignment(
                target, value, operatorPosition, operatorFillers
              ) instanceof Assignment
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
            assignmentDataStrategy,
            ({ target, value, operatorPosition, operatorFillers }) => (
              new Assignment(
                target, value, operatorPosition, operatorFillers
              ).equalTo(
                new Assignment(
                  target, value, operatorPosition, operatorFillers
                )
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
            assignmentStrategy,
            (value: Assignment) => (
              new Assignment(
                value.target,
                value.value,
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
    testEquivalenceOfEqualTo(assignmentStrategy)
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
            assignmentStrategy,
            (value: Assignment) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            assignmentStrategy,
            (value: Assignment) => {
              const result = Assignment.fromJSON(value.toJSON())

              return result instanceof Assignment && result.equalTo(value)
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
            assignmentStrategy,
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
            assignmentStrategy,
            (value: Assignment) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return result instanceof Assignment && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)
