import fc from 'fast-check'
import {
  AnnotatedIdentifier, Filler,
  Identifier,
  SubstringPosition
} from '../../../../pkg'
import { AnnotatedIdentifierData, Expression } from '../../../types'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'
import { identifierStrategy } from '../identifier'

function annotatedIdentifierArgumentsToStrategy<
  Annotation extends Expression, Output
> (
  factory: (
    identifier: Identifier,
    annotation: Annotation,
    operatorPosition: SubstringPosition,
    operatorFillers: Filler[]
  ) => Output,
  annotationStrategy: fc.Arbitrary<Annotation>
): fc.Arbitrary<Output> {
  return fc.tuple(
    identifierStrategy,
    annotationStrategy,
    substringPositionStrategy,
    fillerArrayStrategy
  ).map(
    ([identifier, annotation, operatorPosition, operatorFillers]) => (
      factory(identifier, annotation, operatorPosition, operatorFillers)
    )
  )
}

export function toAnnotatedIdentifierDataStrategy<
  Annotation extends Expression
> (
  annotationStrategy: fc.Arbitrary<Annotation>
): fc.Arbitrary<AnnotatedIdentifierData<Annotation>> {
  return annotatedIdentifierArgumentsToStrategy(
    (identifier, annotation, operatorPosition, operatorFillers) => (
      { identifier, annotation, operatorPosition, operatorFillers }
    ),
    annotationStrategy
  )
}

export function toAnnotatedIdentifierStrategy<Annotation extends Expression> (
  annotationStrategy: fc.Arbitrary<Annotation>
) {
  return annotatedIdentifierArgumentsToStrategy(
    (identifier, annotation, operatorPosition, operatorFillers) => {
      const result = new AnnotatedIdentifier(
        identifier, annotation, operatorPosition, operatorFillers
      )
      result.validateContents()
      return result
    },
    annotationStrategy
  )
}
