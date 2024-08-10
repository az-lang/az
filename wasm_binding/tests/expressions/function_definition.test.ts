import fc from 'fast-check'
import { FunctionDefinition } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import {
  functionDefinitionDataStrategy, functionDefinitionStrategy
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
            functionDefinitionDataStrategy,
            (
              {
                parameters,
                returnType,
                body,
                openerPosition,
                openParenthesisPosition,
                commaPositions,
                closeParenthesisPosition,
                arrowPosition,
                openerFillers,
                openParenthesisFillers,
                commaFillers,
                closeParenthesisFillers,
                arrowFillers
              }
            ) => (
              new FunctionDefinition(
                parameters,
                returnType,
                body,
                openerPosition,
                openParenthesisPosition,
                commaPositions,
                closeParenthesisPosition,
                arrowPosition,
                openerFillers,
                openParenthesisFillers,
                commaFillers,
                closeParenthesisFillers,
                arrowFillers
              ) instanceof FunctionDefinition
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
            functionDefinitionDataStrategy,
            (
              {
                parameters,
                returnType,
                body,
                openerPosition,
                openParenthesisPosition,
                commaPositions,
                closeParenthesisPosition,
                arrowPosition,
                openerFillers,
                openParenthesisFillers,
                commaFillers,
                closeParenthesisFillers,
                arrowFillers
              }
            ) => (
              new FunctionDefinition(
                parameters,
                returnType,
                body,
                openerPosition,
                openParenthesisPosition,
                commaPositions,
                closeParenthesisPosition,
                arrowPosition,
                openerFillers,
                openParenthesisFillers,
                commaFillers,
                closeParenthesisFillers,
                arrowFillers
              ).equalTo(
                new FunctionDefinition(
                  parameters,
                  returnType,
                  body,
                  openerPosition,
                  openParenthesisPosition,
                  commaPositions,
                  closeParenthesisPosition,
                  arrowPosition,
                  openerFillers,
                  openParenthesisFillers,
                  commaFillers,
                  closeParenthesisFillers,
                  arrowFillers
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
            functionDefinitionStrategy,
            (value: FunctionDefinition) => (
              new FunctionDefinition(
                value.parameters,
                value.returnType,
                value.body,
                value.openerPosition,
                value.openParenthesisPosition,
                value.commaPositions,
                value.closeParenthesisPosition,
                value.arrowPosition,
                value.openerFillers,
                value.openParenthesisFillers,
                value.commaFillers,
                value.closeParenthesisFillers,
                value.arrowFillers
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
    testEquivalenceOfEqualTo(functionDefinitionStrategy)
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
            functionDefinitionStrategy,
            (value: FunctionDefinition) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            functionDefinitionStrategy,
            (value: FunctionDefinition) => {
              const result = FunctionDefinition.fromJSON(value.toJSON())

              return (
                result instanceof FunctionDefinition && result.equalTo(value)
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
            functionDefinitionStrategy,
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
            functionDefinitionStrategy,
            (value: FunctionDefinition) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return (
                result instanceof FunctionDefinition && result.equalTo(value)
              )
            }
          )
        )
      }
    )
  }
)
