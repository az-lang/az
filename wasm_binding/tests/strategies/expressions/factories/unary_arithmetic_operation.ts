import fc from 'fast-check'
import {
  Filler, SubstringPosition, UnaryArithmeticOperation
} from '../../../../pkg'
import {
  Expression,
  UnaryArithmeticOperationData,
  UnaryArithmeticOperator
} from '../../../types'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'

function unaryArithmeticOperationArgumentsToStrategy<
  Operand extends Expression, Output
> (
  factory: (
    operand: Operand,
    operator: UnaryArithmeticOperator,
    operatorPosition: SubstringPosition,
    operatorFillers: Filler[]
  ) => Output,
  operator: UnaryArithmeticOperator,
  operandStrategy: fc.Arbitrary<Operand>
) {
  return fc.tuple(
    operandStrategy, substringPositionStrategy, fillerArrayStrategy
  ).map(
    ([operand, operatorPosition, operatorFillers]) => (
      factory(operand, operator, operatorPosition, operatorFillers)
    )
  )
}

export function toUnaryArithmeticOperationDataStrategy<
  Operand extends Expression
> (
  operator: UnaryArithmeticOperator,
  operandStrategy: fc.Arbitrary<Operand>
): fc.Arbitrary<UnaryArithmeticOperationData<Operand>> {
  return unaryArithmeticOperationArgumentsToStrategy(
    (operand, operator, operatorPosition, operatorFillers) => (
      { operand, operator, operatorPosition, operatorFillers }
    ),
    operator,
    operandStrategy
  )
}

export function toUnaryArithmeticOperationStrategy<
  Operand extends Expression
> (operator: UnaryArithmeticOperator, operandStrategy: fc.Arbitrary<Operand>) {
  return unaryArithmeticOperationArgumentsToStrategy(
    (operand, operator, operatorPosition, operatorFillers) => {
      const result = new UnaryArithmeticOperation(
        operand, operator, operatorPosition, operatorFillers
      )
      result.validateContents()
      return result
    },
    operator,
    operandStrategy
  )
}
