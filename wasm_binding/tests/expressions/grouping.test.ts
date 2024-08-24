import fc from 'fast-check'
import { Grouping } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import { groupingDataStrategy, groupingStrategy } from './strategies'
import { INTERNAL_NODE_EXPRESSION_CONTEXT } from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            groupingDataStrategy,
            (
              {
                expression,
                openParenthesisPosition,
                closeParenthesisPosition,
                openParenthesisFillers,
                closeParenthesisFillers
              }
            ) => (
              new Grouping(
                expression,
                openParenthesisPosition,
                closeParenthesisPosition,
                openParenthesisFillers,
                closeParenthesisFillers
              ) instanceof Grouping
            )
          )
        )
      })
    test(
      'repeated',
      () => {
        fc.assert(
          fc.property(
            groupingDataStrategy,
            (
              {
                expression,
                openParenthesisPosition,
                closeParenthesisPosition,
                openParenthesisFillers,
                closeParenthesisFillers
              }
            ) => (
              new Grouping(
                expression,
                openParenthesisPosition,
                closeParenthesisPosition,
                openParenthesisFillers,
                closeParenthesisFillers
              ).equalTo(
                new Grouping(
                  expression,
                  openParenthesisPosition,
                  closeParenthesisPosition,
                  openParenthesisFillers,
                  closeParenthesisFillers
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
            groupingStrategy,
            (value: Grouping) => (
              new Grouping(
                value.expression,
                value.openParenthesisPosition,
                value.closeParenthesisPosition,
                value.openParenthesisFillers,
                value.closeParenthesisFillers
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
    testEquivalenceOfEqualTo(groupingStrategy)
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
            groupingStrategy,
            (value: Grouping) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            groupingStrategy,
            (value: Grouping) => {
              const result = Grouping.fromJSON(value.toJSON())

              return result instanceof Grouping && result.equalTo(value)
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
            groupingStrategy, (value) => typeof value.toString() === typeof ''
          )
        )
      }
    )
    test(
      '`eval` round-trip',
      () => {
        fc.assert(
          fc.property(
            groupingStrategy,
            (value: Grouping) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return result instanceof Grouping && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)
