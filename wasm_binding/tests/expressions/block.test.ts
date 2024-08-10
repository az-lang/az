import fc from 'fast-check'
import { Block } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import { blockDataStrategy, blockStrategy } from './strategies'
import { INTERNAL_NODE_EXPRESSION_CONTEXT } from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            blockDataStrategy,
            (
              {
                statements,
                expression,
                openBracePosition,
                closeBracePosition,
                openBraceFillers,
                closeBraceFillers
              }
            ) => (
              new Block(
                statements,
                expression,
                openBracePosition,
                closeBracePosition,
                openBraceFillers,
                closeBraceFillers
              ) instanceof Block
            )
          )
        )
      })
    test(
      'repeated',
      () => {
        fc.assert(
          fc.property(
            blockDataStrategy,
            (
              {
                statements,
                expression,
                openBracePosition,
                closeBracePosition,
                openBraceFillers,
                closeBraceFillers
              }
            ) => (
              new Block(
                statements,
                expression,
                openBracePosition,
                closeBracePosition,
                openBraceFillers,
                closeBraceFillers
              ).equalTo(
                new Block(
                  statements,
                  expression,
                  openBracePosition,
                  closeBracePosition,
                  openBraceFillers,
                  closeBraceFillers
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
            blockStrategy,
            (value: Block) => (
              new Block(
                value.statements,
                value.expression,
                value.openBracePosition,
                value.closeBracePosition,
                value.openBraceFillers,
                value.closeBraceFillers
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
    testEquivalenceOfEqualTo(blockStrategy)
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
            blockStrategy,
            (value: Block) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            blockStrategy,
            (value: Block) => {
              const result = Block.fromJSON(value.toJSON())

              return result instanceof Block && result.equalTo(value)
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
            blockStrategy, (value) => typeof value.toString() === typeof ''
          )
        )
      }
    )
    test(
      '`eval` round-trip',
      () => {
        fc.assert(
          fc.property(
            blockStrategy,
            (value: Block) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return result instanceof Block && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)
