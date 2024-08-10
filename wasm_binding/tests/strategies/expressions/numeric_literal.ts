import fc from 'fast-check'
import {
  Filler,
  NumericLiteral,
  NumericLiteralType,
  SubstringPosition
} from '../../../pkg'
import { fillerArrayStrategy } from '../filler_array'
import { substringPositionStrategy } from '../substring_position'
import {
  floatingPointLiteralValueStrategy, integerLiteralValueStrategy
} from '../token_content_state'

function numericLiteralArgumentsToStrategy<Output> (
  factory: (
    value: string,
    type_: NumericLiteralType,
    position: SubstringPosition,
    fillers: Filler[]
  ) => Output
) {
  return fc.oneof(
    fc.tuple(
      integerLiteralValueStrategy,
      fc.constantFrom(
        NumericLiteralType.I8,
        NumericLiteralType.I16,
        NumericLiteralType.I32,
        NumericLiteralType.I64,
        NumericLiteralType.ISIZE,
        NumericLiteralType.U8,
        NumericLiteralType.U16,
        NumericLiteralType.U32,
        NumericLiteralType.U64,
        NumericLiteralType.USIZE
      ),
      substringPositionStrategy,
      fillerArrayStrategy
    ),
    fc.tuple(
      floatingPointLiteralValueStrategy,
      fc.constantFrom(NumericLiteralType.F32, NumericLiteralType.F64),
      substringPositionStrategy,
      fillerArrayStrategy
    )
  ).map(
    ([value, type_, position, fillers]) => (
      factory(value, type_, position, fillers)
    )
  )
}

export const numericLiteralDataStrategy = numericLiteralArgumentsToStrategy(
  (value, type_, position, fillers) => ({ value, type_, position, fillers })
)
export const numericLiteralStrategy = numericLiteralArgumentsToStrategy(
  (value, type_, position, fillers) => (
    new NumericLiteral(value, type_, position, fillers)
  )
)
