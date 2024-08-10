import fc from 'fast-check'
import { Call, Filler, SubstringPosition } from '../../../../pkg'
import { CallData, Expression } from '../../../types'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'
import { MAX_EXPRESSIONS_ARRAY_LENGTH } from './constants'

function callArgumentsToStrategy<
  Callable extends Expression, Argument extends Expression, Output
> (
  factory: (
    callable: Callable,
    arguments_: Argument[],
    openParenthesisPosition: SubstringPosition,
    commaPositions: SubstringPosition[],
    closeParenthesisPosition: SubstringPosition,
    openParenthesisFillers: Filler[],
    commaFillers: Filler[][],
    closeParenthesisFillers: Filler[]
  ) => Output,
  callableStrategy: fc.Arbitrary<Callable>,
  argumentStrategy: fc.Arbitrary<Argument>
) {
  const argumentsStrategy = fc.array(
    argumentStrategy, { maxLength: MAX_EXPRESSIONS_ARRAY_LENGTH }
  )
  const commaFillersStrategy = fc.array(
    fillerArrayStrategy, { maxLength: MAX_EXPRESSIONS_ARRAY_LENGTH }
  )
  const commaPositionsStrategy = fc.array(
    substringPositionStrategy, { maxLength: MAX_EXPRESSIONS_ARRAY_LENGTH }
  )
  return fc.oneof(
    fc.tuple(
      callableStrategy,
      argumentsStrategy,
      substringPositionStrategy,
      commaPositionsStrategy,
      substringPositionStrategy,
      fillerArrayStrategy,
      commaFillersStrategy,
      fillerArrayStrategy
    ).map(
      (
        [
          callable,
          arguments_,
          openParenthesisPosition,
          commaPositions,
          closeParenthesisPosition,
          openParenthesisFillers,
          commaFillers,
          closeParenthesisFillers
        ]
      ) => {
        const argumentsCount = Math.min(
          arguments_.length, commaPositions.length
        )
        for (let _ = commaFillers.length; _ < argumentsCount; _++) {
          commaFillers.push([])
        }
        return factory(
          callable,
          arguments_.slice(0, argumentsCount),
          openParenthesisPosition,
          commaPositions.slice(0, argumentsCount),
          closeParenthesisPosition,
          openParenthesisFillers,
          commaFillers.slice(0, argumentsCount),
          closeParenthesisFillers
        )
      }
    ),
    fc.tuple(
      callableStrategy,
      argumentsStrategy,
      substringPositionStrategy,
      commaPositionsStrategy,
      substringPositionStrategy,
      fillerArrayStrategy,
      commaFillersStrategy,
      fillerArrayStrategy
    ).map(
      (
        [
          callable,
          arguments_,
          openParenthesisPosition,
          commaPositions,
          closeParenthesisPosition,
          openParenthesisFillers,
          commaFillers,
          closeParenthesisFillers
        ]
      ) => {
        const argumentsCount = Math.min(
          arguments_.length, commaPositions.length
        )
        for (let _ = commaFillers.length; _ < argumentsCount; _++) {
          commaFillers.push([])
        }
        return factory(
          callable,
          arguments_.slice(0, argumentsCount),
          openParenthesisPosition,
          commaPositions.slice(0, Math.max(argumentsCount - 1, 0)),
          closeParenthesisPosition,
          openParenthesisFillers,
          commaFillers.slice(0, Math.max(argumentsCount - 1, 0)),
          closeParenthesisFillers
        )
      }
    )
  )
}

export function toCallDataStrategy<
  Callable extends Expression, Argument extends Expression
> (
  callableStrategy: fc.Arbitrary<Callable>,
  argumentStrategy: fc.Arbitrary<Argument>
): fc.Arbitrary<CallData<Callable, Argument>> {
  return callArgumentsToStrategy(
    (
      callable,
      arguments_,
      openParenthesisPosition,
      commaPositions,
      closeParenthesisPosition,
      openParenthesisFillers,
      commaFillers,
      closeParenthesisFillers
    ) => (
      {
        callable,
        arguments_,
        openParenthesisPosition,
        commaPositions,
        closeParenthesisPosition,
        openParenthesisFillers,
        commaFillers,
        closeParenthesisFillers
      }
    ),
    callableStrategy,
    argumentStrategy
  )
}

export function toCallStrategy<
  Callable extends Expression, Argument extends Expression
> (
  callableStrategy: fc.Arbitrary<Callable>,
  argumentStrategy: fc.Arbitrary<Argument>
) {
  return callArgumentsToStrategy(
    (
      callable,
      arguments_,
      openParenthesisPosition,
      commaPositions,
      closeParenthesisPosition,
      openParenthesisFillers,
      commaFillers,
      closeParenthesisFillers
    ) => {
      const result = new Call(
        callable,
        arguments_,
        openParenthesisPosition,
        commaPositions,
        closeParenthesisPosition,
        openParenthesisFillers,
        commaFillers,
        closeParenthesisFillers
      )
      result.validateContents()
      return result
    },
    callableStrategy,
    argumentStrategy
  )
}
