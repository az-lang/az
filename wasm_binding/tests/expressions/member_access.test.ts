import fc from 'fast-check'
import { MemberAccess } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import { memberAccessDataStrategy, memberAccessStrategy } from './strategies'
import { INTERNAL_NODE_EXPRESSION_CONTEXT } from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            memberAccessDataStrategy,
            ({ object, member, operatorPosition, operatorFillers }) => (
              new MemberAccess(
                object, member, operatorPosition, operatorFillers
              ) instanceof MemberAccess
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
            memberAccessDataStrategy,
            ({ object, member, operatorPosition, operatorFillers }) => (
              new MemberAccess(
                object, member, operatorPosition, operatorFillers
              ).equalTo(
                new MemberAccess(
                  object, member, operatorPosition, operatorFillers
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
            memberAccessStrategy,
            (value: MemberAccess) => (
              new MemberAccess(
                value.object,
                value.member,
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
    testEquivalenceOfEqualTo(memberAccessStrategy)
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
            memberAccessStrategy,
            (value: MemberAccess) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            memberAccessStrategy,
            (value: MemberAccess) => {
              const result = MemberAccess.fromJSON(value.toJSON())

              return result instanceof MemberAccess && result.equalTo(value)
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
            memberAccessStrategy,
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
            memberAccessStrategy,
            (value: MemberAccess) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return result instanceof MemberAccess && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)
