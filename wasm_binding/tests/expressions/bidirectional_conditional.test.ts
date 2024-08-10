import fc from 'fast-check'
import { BidirectionalConditional } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import {
  bidirectionalConditionalDataStrategy, bidirectionalConditionalStrategy
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
            bidirectionalConditionalDataStrategy,
            (
              {
                antecedent,
                consequent,
                alternative,
                antecedentOpenerPosition,
                alternativeOpenerPosition,
                antecedentOpenerFillers,
                alternativeOpenerFillers
              }
            ) => (
              new BidirectionalConditional(
                antecedent,
                consequent,
                alternative,
                antecedentOpenerPosition,
                alternativeOpenerPosition,
                antecedentOpenerFillers,
                alternativeOpenerFillers
              ) instanceof BidirectionalConditional
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
            bidirectionalConditionalDataStrategy,
            (
              {
                antecedent,
                consequent,
                alternative,
                antecedentOpenerPosition,
                alternativeOpenerPosition,
                antecedentOpenerFillers,
                alternativeOpenerFillers
              }
            ) => (
              new BidirectionalConditional(
                antecedent,
                consequent,
                alternative,
                antecedentOpenerPosition,
                alternativeOpenerPosition,
                antecedentOpenerFillers,
                alternativeOpenerFillers
              ).equalTo(
                new BidirectionalConditional(
                  antecedent,
                  consequent,
                  alternative,
                  antecedentOpenerPosition,
                  alternativeOpenerPosition,
                  antecedentOpenerFillers,
                  alternativeOpenerFillers
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
            bidirectionalConditionalStrategy,
            (value: BidirectionalConditional) => (
              new BidirectionalConditional(
                value.antecedent,
                value.consequent,
                value.alternative,
                value.antecedentOpenerPosition,
                value.alternativeOpenerPosition,
                value.antecedentOpenerFillers,
                value.alternativeOpenerFillers
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
    testEquivalenceOfEqualTo(bidirectionalConditionalStrategy)
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
            bidirectionalConditionalStrategy,
            (value: BidirectionalConditional) => (
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
            bidirectionalConditionalStrategy,
            (value: BidirectionalConditional) => {
              const result = BidirectionalConditional.fromJSON(value.toJSON())

              return (
                result instanceof BidirectionalConditional
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
            bidirectionalConditionalStrategy,
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
            bidirectionalConditionalStrategy,
            (value: BidirectionalConditional) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return (
                result instanceof BidirectionalConditional
                && result.equalTo(value)
              )
            }
          )
        )
      }
    )
  }
)
