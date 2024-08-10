import fc from 'fast-check'
import { Filler, Grouping, SubstringPosition } from '../../../../pkg'
import { Expression, GroupingData } from '../../../types'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'

function groupingArgumentsToStrategy<
  ExpressionT extends Expression, Output
> (
  factory: (
    expression: ExpressionT,
    openParenthesisPosition: SubstringPosition,
    closeParenthesisPosition: SubstringPosition,
    openParenthesisFillers: Filler[],
    closeParenthesisFillers: Filler[]
  ) => Output,
  expressionStrategy: fc.Arbitrary<ExpressionT>
): fc.Arbitrary<Output> {
  return fc.tuple(
    expressionStrategy,
    substringPositionStrategy,
    substringPositionStrategy,
    fillerArrayStrategy,
    fillerArrayStrategy
  ).map(
    (
      [
        expression,
        openParenthesisPosition,
        closeParenthesisPosition,
        openParenthesisFillers,
        closeParenthesisFillers
      ]
    ) => factory(
      expression,
      openParenthesisPosition,
      closeParenthesisPosition,
      openParenthesisFillers,
      closeParenthesisFillers
    )
  )
}

export function toGroupingDataStrategy<ExpressionT extends Expression> (
  expressionStrategy: fc.Arbitrary<ExpressionT>
): fc.Arbitrary<GroupingData<ExpressionT>> {
  return groupingArgumentsToStrategy(
    (
      expression,
      openParenthesisPosition,
      closeParenthesisPosition,
      openParenthesisFillers,
      closeParenthesisFillers
    ) => (
      {
        expression,
        openParenthesisPosition,
        closeParenthesisPosition,
        openParenthesisFillers,
        closeParenthesisFillers
      }
    ),
    expressionStrategy
  )
}

export function toGroupingStrategy<ExpressionT extends Expression> (
  expressionStrategy: fc.Arbitrary<ExpressionT>
) {
  return groupingArgumentsToStrategy(
    (
      expression,
      openParenthesisPosition,
      closeParenthesisPosition,
      openParenthesisFillers,
      closeParenthesisFillers
    ) => {
      const result = new Grouping(
        expression,
        openParenthesisPosition,
        closeParenthesisPosition,
        openParenthesisFillers,
        closeParenthesisFillers
      )
      result.validateContents()
      return result
    },
    expressionStrategy
  )
}
