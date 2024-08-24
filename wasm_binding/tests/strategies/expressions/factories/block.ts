import fc from 'fast-check'
import { Block, Filler, SubstringPosition } from '../../../../pkg'
import { BlockData, Expression, Statement } from '../../../types'
import { fillerArrayStrategy } from '../../filler_array'
import { toStatementArrayStrategy } from '../../statements/factories'
import { substringPositionStrategy } from '../../substring_position'

function blockArgumentsToStrategy<ExpressionT extends Expression, Output> (
  factory: (
    statements: Statement[],
    expression: ExpressionT | null,
    openBracePosition: SubstringPosition,
    closeBracePosition: SubstringPosition,
    openBraceFillers: Filler[],
    closeBraceFillers: Filler[]
  ) => Output,
  expressionStrategy: fc.Arbitrary<ExpressionT>
): fc.Arbitrary<Output> {
  return fc.tuple(
    toStatementArrayStrategy(expressionStrategy),
    fc.option(expressionStrategy),
    substringPositionStrategy,
    substringPositionStrategy,
    fillerArrayStrategy,
    fillerArrayStrategy
  ).map(
    (
      [
        statements,
        expression,
        openBracePosition,
        closeBracePosition,
        openBraceFillers,
        closeBraceFillers
      ]
    ) => factory(
      statements,
      expression,
      openBracePosition,
      closeBracePosition,
      openBraceFillers,
      closeBraceFillers
    )
  )
}

export function toBlockDataStrategy<ExpressionT extends Expression> (
  expressionStrategy: fc.Arbitrary<ExpressionT>
): fc.Arbitrary<BlockData<ExpressionT>> {
  return blockArgumentsToStrategy(
    (
      statements,
      expression,
      openBracePosition,
      closeBracePosition,
      openBraceFillers,
      closeBraceFillers
    ) => (
      {
        statements,
        expression,
        openBracePosition,
        closeBracePosition,
        openBraceFillers,
        closeBraceFillers
      }
    ),
    expressionStrategy
  )
}

export function toBlockStrategy<ExpressionT extends Expression> (
  expressionStrategy: fc.Arbitrary<ExpressionT>
) {
  return blockArgumentsToStrategy(
    (
      statements,
      expression,
      openBracePosition,
      closeBracePosition,
      openBraceFillers,
      closeBraceFillers
    ) => {
      const result = new Block(
        statements,
        expression,
        openBracePosition,
        closeBracePosition,
        openBraceFillers,
        closeBraceFillers
      )
      result.validateContents()
      return result
    },
    expressionStrategy
  )
}
