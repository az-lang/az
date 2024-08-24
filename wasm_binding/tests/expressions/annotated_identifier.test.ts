import fc from 'fast-check'
import { AnnotatedIdentifier } from '../../pkg'
import { evalInContext, testEquivalenceOfEqualTo } from '../utils'
import {
  annotatedIdentifierDataStrategy, annotatedIdentifierStrategy
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
            annotatedIdentifierDataStrategy,
            (
              { identifier, annotation, operatorPosition, operatorFillers }
            ) => (
              new AnnotatedIdentifier(
                identifier, annotation, operatorPosition, operatorFillers
              ) instanceof AnnotatedIdentifier
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
            annotatedIdentifierDataStrategy,
            (
              { identifier, annotation, operatorPosition, operatorFillers }
            ) => (
              new AnnotatedIdentifier(
                identifier, annotation, operatorPosition, operatorFillers
              ).equalTo(
                new AnnotatedIdentifier(
                  identifier, annotation, operatorPosition, operatorFillers
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
            annotatedIdentifierStrategy,
            (value: AnnotatedIdentifier) => (
              new AnnotatedIdentifier(
                value.identifier,
                value.annotation,
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
    testEquivalenceOfEqualTo(annotatedIdentifierStrategy)
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
            annotatedIdentifierStrategy,
            (value: AnnotatedIdentifier) => typeof value.toJSON() === typeof {}
          )
        )
      }
    )
    test(
      '`fromJSON` round-trip',
      () => {
        fc.assert(
          fc.property(
            annotatedIdentifierStrategy,
            (value: AnnotatedIdentifier) => {
              const result = AnnotatedIdentifier.fromJSON(value.toJSON())

              return (
                result instanceof AnnotatedIdentifier && result.equalTo(value)
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
            annotatedIdentifierStrategy,
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
            annotatedIdentifierStrategy,
            (value: AnnotatedIdentifier) => {
              const result = evalInContext(
                value.toString(), INTERNAL_NODE_EXPRESSION_CONTEXT
              )

              return (
                result instanceof AnnotatedIdentifier && result.equalTo(value)
              )
            }
          )
        )
      }
    )
  }
)
