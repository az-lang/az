import fc from 'fast-check'
import { Assignment, Filler, SubstringPosition } from '../../../../pkg'
import { AssignmentData, Expression } from '../../../types'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'

function assignmentArgumentsToStrategy<
  Target extends Expression, Value extends Expression, Output
> (
  factory: (
    target: Target,
    value: Value,
    operatorPosition: SubstringPosition,
    operatorFillers: Filler[]
  ) => Output,
  targetStrategy: fc.Arbitrary<Target>,
  valueStrategy: fc.Arbitrary<Value>
): fc.Arbitrary<Output> {
  return fc.tuple(
    targetStrategy,
    valueStrategy,
    substringPositionStrategy,
    fillerArrayStrategy
  ).map(
    ([target, value, operatorPosition, operatorFillers]) => (
      factory(target, value, operatorPosition, operatorFillers)
    )
  )
}

export function toAssignmentDataStrategy<
  Target extends Expression, Value extends Expression
> (
  targetStrategy: fc.Arbitrary<Target>, valueStrategy: fc.Arbitrary<Value>
): fc.Arbitrary<AssignmentData<Target, Value>> {
  return assignmentArgumentsToStrategy(
    (target, value, operatorPosition, operatorFillers) => (
      { target, value, operatorPosition, operatorFillers }
    ),
    targetStrategy,
    valueStrategy
  )
}

export function toAssignmentStrategy<
  Target extends Expression, Value extends Expression
> (
  targetStrategy: fc.Arbitrary<Target>, valueStrategy: fc.Arbitrary<Value>
) {
  return assignmentArgumentsToStrategy(
    (target, value, operatorPosition, operatorFillers) => {
      const result = new Assignment(
        target, value, operatorPosition, operatorFillers
      )
      result.validateContents()
      return result
    },
    targetStrategy,
    valueStrategy
  )
}
