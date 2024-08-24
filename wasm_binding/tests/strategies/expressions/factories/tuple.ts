import fc from 'fast-check'
import { Filler, SubstringPosition, Tuple } from '../../../../pkg'
import { Expression, TupleData } from '../../../types'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'
import { MAX_EXPRESSIONS_ARRAY_LENGTH } from './constants'

function tupleArgumentsToStrategy<Element extends Expression, Output> (
  factory: (
    elements: Element[],
    openParenthesisPosition: SubstringPosition,
    commaPositions: SubstringPosition[],
    closeParenthesisPosition: SubstringPosition,
    openParenthesisFillers: Filler[],
    commaFillers: Filler[][],
    closeParenthesisFillers: Filler[]
  ) => Output,
  elementStrategy: fc.Arbitrary<Element>
): fc.Arbitrary<Output> {
  const elementsStrategy = fc.array(
    elementStrategy, { maxLength: MAX_EXPRESSIONS_ARRAY_LENGTH }
  )
  const substringPositionArrayStrategy = fc.array(
    substringPositionStrategy, { maxLength: MAX_EXPRESSIONS_ARRAY_LENGTH }
  )
  const fillerArrayArrayStrategy = fc.array(
    fillerArrayStrategy, { maxLength: MAX_EXPRESSIONS_ARRAY_LENGTH }
  )
  return fc.oneof(
    fc.tuple(
      elementsStrategy,
      substringPositionStrategy,
      substringPositionArrayStrategy,
      substringPositionStrategy,
      fillerArrayStrategy,
      fillerArrayArrayStrategy,
      fillerArrayStrategy
    ).map(
      (
        [
          elements,
          openParenthesisPosition,
          commaPositions,
          closeParenthesisPosition,
          openParenthesisFillers,
          commaFillers,
          closeParenthesisFillers
        ]
      ) => {
        const elementsCount = Math.min(
          elements.length, commaPositions.length
        )
        for (let _ = commaFillers.length; _ < elementsCount; _++) {
          commaFillers.push([])
        }
        return factory(
          elements.slice(0, elementsCount),
          openParenthesisPosition,
          commaPositions.slice(0, elementsCount),
          closeParenthesisPosition,
          openParenthesisFillers,
          commaFillers.slice(0, elementsCount),
          closeParenthesisFillers
        )
      }
    ),
    fc.tuple(
      elementsStrategy,
      substringPositionStrategy,
      substringPositionArrayStrategy,
      substringPositionStrategy,
      fillerArrayStrategy,
      fillerArrayArrayStrategy,
      fillerArrayStrategy
    ).map(
      (
        [
          elements,
          openParenthesisPosition,
          commaPositions,
          closeParenthesisPosition,
          openParenthesisFillers,
          commaFillers,
          closeParenthesisFillers
        ]
      ) => {
        const elementsCount = Math.min(
          elements.length, commaPositions.length
        )
        for (let _ = commaFillers.length; _ < elementsCount; _++) {
          commaFillers.push([])
        }
        const commasCount = (
          elementsCount === 1 ? elementsCount : Math.max(elementsCount - 1, 0)
        )
        return factory(
          elements.slice(0, elementsCount),
          openParenthesisPosition,
          commaPositions.slice(0, commasCount),
          closeParenthesisPosition,
          openParenthesisFillers,
          commaFillers.slice(0, commasCount),
          closeParenthesisFillers
        )
      }
    )
  )
}

export function toTupleDataStrategy<Element extends Expression> (
  elementStrategy: fc.Arbitrary<Element>
): fc.Arbitrary<TupleData<Element>> {
  return tupleArgumentsToStrategy(
    (
      elements,
      openParenthesisPosition,
      commaPositions,
      closeParenthesisPosition,
      openParenthesisFillers,
      commaFillers,
      closeParenthesisFillers
    ) => (
      {
        elements,
        openParenthesisPosition,
        commaPositions,
        closeParenthesisPosition,
        openParenthesisFillers,
        commaFillers,
        closeParenthesisFillers
      }
    ),
    elementStrategy
  )
}

export function toTupleStrategy<Element extends Expression> (
  elementStrategy: fc.Arbitrary<Element>
) {
  return tupleArgumentsToStrategy(
    (
      elements,
      openParenthesisPosition,
      commaPositions,
      closeParenthesisPosition,
      openParenthesisFillers,
      commaFillers,
      closeParenthesisFillers
    ) => {
      const result = new Tuple(
        elements,
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
    elementStrategy
  )
}
