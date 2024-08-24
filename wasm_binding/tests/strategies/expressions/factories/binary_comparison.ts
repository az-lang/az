import fc from 'fast-check'
import { BinaryComparison, Filler, SubstringPosition } from '../../../../pkg'
import {
  BinaryComparisonData, BinaryComparisonOperator, Expression
} from '../../../types'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'

function binaryComparisonArgumentsToStrategy<
  LeftOperand extends Expression, RightOperand extends Expression, Output
> (
  factory: (
    left: LeftOperand,
    right: RightOperand,
    operator: BinaryComparisonOperator,
    operatorPosition: SubstringPosition,
    operatorFillers: Filler[]
  ) => Output,
  operator: BinaryComparisonOperator,
  leftOperandStrategy: fc.Arbitrary<LeftOperand>,
  rightOperandStrategy: fc.Arbitrary<RightOperand>
) {
  return fc.tuple(
    leftOperandStrategy,
    rightOperandStrategy,
    substringPositionStrategy,
    fillerArrayStrategy
  ).map(
    ([left, right, operatorPosition, operatorFillers]) => (
      factory(left, right, operator, operatorPosition, operatorFillers)
    )
  )
}

export function toBinaryComparisonDataStrategy<
  LeftOperand extends Expression, RightOperand extends Expression
> (
  operator: BinaryComparisonOperator,
  leftOperandStrategy: fc.Arbitrary<LeftOperand>,
  rightOperandStrategy: fc.Arbitrary<RightOperand>
): fc.Arbitrary<BinaryComparisonData<LeftOperand, RightOperand>> {
  return binaryComparisonArgumentsToStrategy(
    (left, right, operator, operatorPosition, operatorFillers) => (
      { left, right, operator, operatorPosition, operatorFillers }
    ),
    operator,
    leftOperandStrategy,
    rightOperandStrategy
  )
}

export function toBinaryComparisonStrategy<
  LeftOperand extends Expression, RightOperand extends Expression
> (
  operator: BinaryComparisonOperator,
  leftOperandStrategy: fc.Arbitrary<LeftOperand>,
  rightOperandStrategy: fc.Arbitrary<RightOperand>
) {
  return binaryComparisonArgumentsToStrategy(
    (left, right, operator, operatorPosition, operatorFillers) => {
      const result = new BinaryComparison(
        left, right, operator, operatorPosition, operatorFillers
      )
      result.validateContents()
      return result
    },
    operator,
    leftOperandStrategy,
    rightOperandStrategy
  )
}
