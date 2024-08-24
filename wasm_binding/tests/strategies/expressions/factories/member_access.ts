import fc from 'fast-check'
import {
  Filler, Identifier, MemberAccess, SubstringPosition
} from '../../../../pkg'
import { Expression, MemberAccessData } from '../../../types'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'
import { identifierStrategy } from '../identifier'

function memberAccessArgumentsToStrategy<ObjectT extends Expression, Output> (
  factory: (
    object: ObjectT,
    member: Identifier,
    operatorPosition: SubstringPosition,
    operatorFillers: Filler[]
  ) => Output,
  objectStrategy: fc.Arbitrary<ObjectT>
): fc.Arbitrary<Output> {
  return fc.tuple(
    objectStrategy,
    identifierStrategy,
    substringPositionStrategy,
    fillerArrayStrategy
  ).map(
    ([object, member, operatorPosition, operatorFillers]) => (
      factory(object, member, operatorPosition, operatorFillers)
    )
  )
}

export function toMemberAccessDataStrategy<ObjectT extends Expression> (
  objectStrategy: fc.Arbitrary<ObjectT>
): fc.Arbitrary<MemberAccessData<ObjectT>> {
  return memberAccessArgumentsToStrategy(
    (object, member, operatorPosition, operatorFillers) => (
      { object, member, operatorPosition, operatorFillers }
    ),
    objectStrategy
  )
}

export function toMemberAccessStrategy<ObjectT extends Expression> (
  objectStrategy: fc.Arbitrary<ObjectT>
) {
  return memberAccessArgumentsToStrategy(
    (object, member, operatorPosition, operatorFillers) => {
      const result = new MemberAccess(
        object, member, operatorPosition, operatorFillers
      )
      result.validateContents()
      return result
    },
    objectStrategy
  )
}
