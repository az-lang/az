import fc from 'fast-check'
import { TokenContent, TokenKind } from '../pkg'
import {
  STATELESS_TOKEN_KINDS, tokenContentDataStrategy, tokenContentStrategy
} from './strategies'
import { evalInContext, testEquivalenceOfEqualTo } from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            tokenContentDataStrategy,
            ({ kind, state }) => (
              new TokenContent(kind, state) instanceof TokenContent
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
            tokenContentDataStrategy,
            ({ kind, state }) => (
              new TokenContent(kind, state).equalTo(
                new TokenContent(kind, state)
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
            tokenContentStrategy,
            (value: TokenContent) => (
              STATELESS_TOKEN_KINDS.includes(value.kind)
                ? new TokenContent(value.kind, undefined)
                : new TokenContent(value.kind, value.state)
            ).equalTo(value)
          )
        )
      }
    )
  }
)

describe(
  '`equalTo` tests',
  () => {
    testEquivalenceOfEqualTo(tokenContentStrategy)
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
            tokenContentStrategy,
            (value: TokenContent) => (
              typeof value.toJSON() === typeof (
                STATELESS_TOKEN_KINDS.includes(value.kind) ? '' : {}
              )
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
            tokenContentStrategy,
            (value: TokenContent) => {
              const result = TokenContent.fromJSON(value.toJSON())

              return result instanceof TokenContent && result.equalTo(value)
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
            tokenContentStrategy,
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
            tokenContentStrategy,
            (value: TokenContent) => {
              const result = evalInContext(
                value.toString(), { TokenContent, TokenKind }
              )

              return result instanceof TokenContent && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)
