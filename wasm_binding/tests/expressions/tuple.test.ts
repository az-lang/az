import fc from 'fast-check'
import { Tuple } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import { tupleDataStrategy, tupleStrategy } from './strategies'
import { INTERNAL_NODE_EXPRESSION_CONTEXT } from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            tupleDataStrategy,
            (
              {
                elements,
                openParenthesisPosition,
                commaPositions,
                closeParenthesisPosition,
                openParenthesisFillers,
                commaFillers,
                closeParenthesisFillers
              }
            ) => (
              new Tuple(
                elements,
                openParenthesisPosition,
                commaPositions,
                closeParenthesisPosition,
                openParenthesisFillers,
                commaFillers,
                closeParenthesisFillers
              ) instanceof Tuple
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
            tupleDataStrategy,
            (
              {
                elements,
                openParenthesisPosition,
                commaPositions,
                closeParenthesisPosition,
                openParenthesisFillers,
                commaFillers,
                closeParenthesisFillers
              }
            ) => (
              new Tuple(
                elements,
                openParenthesisPosition,
                commaPositions,
                closeParenthesisPosition,
                openParenthesisFillers,
                commaFillers,
                closeParenthesisFillers
              ).equalTo(
                new Tuple(
                  elements,
                  openParenthesisPosition,
                  commaPositions,
                  closeParenthesisPosition,
                  openParenthesisFillers,
                  commaFillers,
                  closeParenthesisFillers
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
            tupleStrategy,
            (value: Tuple) => (
              new Tuple(
                value.elements,
                value.openParenthesisPosition,
                value.commaPositions,
                value.closeParenthesisPosition,
                value.openParenthesisFillers,
                value.commaFillers,
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
    testEquivalenceOfEqualTo(tupleStrategy)
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
            tupleStrategy,
            (value: Tuple) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            tupleStrategy,
            (value: Tuple) => {
              const result = Tuple.fromJSON(value.toJSON())

              return result instanceof Tuple && result.equalTo(value)
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
            tupleStrategy, (value) => typeof value.toString() === typeof ''
          )
        )
      }
    )
    test(
      '`eval` round-trip',
      () => {
        fc.assert(
          fc.property(
            tupleStrategy,
            (value: Tuple) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return result instanceof Tuple && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)
