import fc from 'fast-check'
import { UnidirectionalConditional } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import {
  unidirectionalConditionalDataStrategy, unidirectionalConditionalStrategy
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
            unidirectionalConditionalDataStrategy,
            ({ antecedent, consequent, openerPosition, openerFillers }) => (
              new UnidirectionalConditional(
                antecedent, consequent, openerPosition, openerFillers
              ) instanceof UnidirectionalConditional
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
            unidirectionalConditionalDataStrategy,
            ({ antecedent, consequent, openerPosition, openerFillers }) => (
              new UnidirectionalConditional(
                antecedent, consequent, openerPosition, openerFillers
              ).equalTo(
                new UnidirectionalConditional(
                  antecedent, consequent, openerPosition, openerFillers
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
            unidirectionalConditionalStrategy,
            (value: UnidirectionalConditional) => (
              new UnidirectionalConditional(
                value.antecedent,
                value.consequent,
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
    testEquivalenceOfEqualTo(unidirectionalConditionalStrategy)
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
            unidirectionalConditionalStrategy,
            (value: UnidirectionalConditional) => (
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
            unidirectionalConditionalStrategy,
            (value: UnidirectionalConditional) => {
              const result = UnidirectionalConditional.fromJSON(value.toJSON())

              return (
                result instanceof UnidirectionalConditional
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
            unidirectionalConditionalStrategy,
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
            unidirectionalConditionalStrategy,
            (value: UnidirectionalConditional) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return (
                result instanceof UnidirectionalConditional
                && result.equalTo(value)
              )
            }
          )
        )
      }
    )
  }
)
