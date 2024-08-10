import fc from 'fast-check'
import { Call } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import { callDataStrategy, callStrategy } from './strategies'
import { INTERNAL_NODE_EXPRESSION_CONTEXT } from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            callDataStrategy,
            (
              {
                callable,
                arguments_,
                openParenthesisPosition,
                commaPositions,
                closeParenthesisPosition,
                openParenthesisFillers,
                commaFillers,
                closeParenthesisFillers
              }
            ) => (
              new Call(
                callable,
                arguments_,
                openParenthesisPosition,
                commaPositions,
                closeParenthesisPosition,
                openParenthesisFillers,
                commaFillers,
                closeParenthesisFillers
              ) instanceof Call
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
            callDataStrategy,
            (
              {
                callable,
                arguments_,
                openParenthesisPosition,
                commaPositions,
                closeParenthesisPosition,
                openParenthesisFillers,
                commaFillers,
                closeParenthesisFillers
              }
            ) => (
              new Call(
                callable,
                arguments_,
                openParenthesisPosition,
                commaPositions,
                closeParenthesisPosition,
                openParenthesisFillers,
                commaFillers,
                closeParenthesisFillers
              ).equalTo(
                new Call(
                  callable,
                  arguments_,
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
            callStrategy,
            (value: Call) => (
              new Call(
                value.callable,
                value.arguments_,
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
    testEquivalenceOfEqualTo(callStrategy)
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
            callStrategy, (value: Call) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            callStrategy,
            (value: Call) => {
              const result = Call.fromJSON(value.toJSON())

              return result instanceof Call && result.equalTo(value)
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
            callStrategy, (value) => typeof value.toString() === typeof ''
          )
        )
      }
    )
    test(
      '`eval` round-trip',
      () => {
        fc.assert(
          fc.property(
            callStrategy,
            (value: Call) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return result instanceof Call && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)
