import fc from 'fast-check'
import { Filler, FillerContent, SubstringPosition } from '../../pkg'
import {
  fillerContentStrategy, nonCommentFillerContentStrategy
} from './filler_content'
import { substringPositionStrategy } from './substring_position'

function fillerArgumentsToStrategy<Output> (
  factory: (
    content: FillerContent, position: SubstringPosition
  ) => Output,
  contentStrategy: fc.Arbitrary<FillerContent>
): fc.Arbitrary<Output> {
  return fc.tuple(
    contentStrategy, substringPositionStrategy
  ).map(([content, position]) => factory(content, position))
}

function fillerFromData (content: FillerContent, position: SubstringPosition) {
  const result = new Filler(content, position)
  result.validateContents()
  return result
}

export const fillerDataStrategy = fillerArgumentsToStrategy(
  (content, position) => ({ content, position }),
  fillerContentStrategy
)
export const fillerStrategy = fillerArgumentsToStrategy(
  (content, position) => fillerFromData(content, position),
  fillerContentStrategy
)
export const nonCommentFillerStrategy = fillerArgumentsToStrategy(
  (content, position) => fillerFromData(content, position),
  nonCommentFillerContentStrategy
)
