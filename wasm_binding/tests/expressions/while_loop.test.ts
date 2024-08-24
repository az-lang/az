import fc from 'fast-check'
import { WhileLoop } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import { whileLoopDataStrategy, whileLoopStrategy } from './strategies'
import { INTERNAL_NODE_EXPRESSION_CONTEXT } from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            whileLoopDataStrategy,
            ({ condition, body, openerPosition, openerFillers }) => (
              new WhileLoop(
                condition, body, openerPosition, openerFillers
              ) instanceof WhileLoop
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
            whileLoopDataStrategy,
            ({ condition, body, openerPosition, openerFillers }) => (
              new WhileLoop(
                condition, body, openerPosition, openerFillers
              ).equalTo(
                new WhileLoop(condition, body, openerPosition, openerFillers)
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
            whileLoopStrategy,
            (value: WhileLoop) => (
              new WhileLoop(
                value.condition,
                value.body,
                value.openerPosition,
                value.openerFillers
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
    testEquivalenceOfEqualTo(whileLoopStrategy)
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
            whileLoopStrategy,
            (value: WhileLoop) => (
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
            whileLoopStrategy,
            (value: WhileLoop) => {
              const result = WhileLoop.fromJSON(value.toJSON())

              return (
                result instanceof WhileLoop
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
            whileLoopStrategy,
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
            whileLoopStrategy,
            (value: WhileLoop) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return (
                result instanceof WhileLoop
                && result.equalTo(value)
              )
            }
          )
        )
      }
    )
  }
)
