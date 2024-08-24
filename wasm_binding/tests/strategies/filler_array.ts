import fc from 'fast-check'
import {
  Filler, FillerContent, FillerKind, SubstringPosition
} from '../../pkg'
import { fillerStrategy } from './filler'

function mergeConsecutiveWhitespaceFillers (
  fillers: readonly Filler[]
): Filler[] {
  const result = []
  for (let index = 0; index < fillers.length; index++) {
    const filler = fillers[index]!
    if (filler.content.kind === FillerKind.WHITESPACE) {
      const start = index
      let state = filler.content.state as string
      while (
        index + 1 < fillers.length
        && fillers[index + 1]!.content.kind === FillerKind.WHITESPACE
      ) {
        index++
        state += fillers[index]!.content.state as string
      }
      if (start === index) {
        result.push(filler)
      } else {
        result.push(new Filler(
          new FillerContent(FillerKind.WHITESPACE, state),
          new SubstringPosition(
            filler.position.start, fillers[index]!.position.end
          )
        ))
      }
    } else {
      result.push(filler)
    }
  }
  return result
}

function toFillerArrayWithCommentLinesFollowedByNewlines (
  fillers: Filler[]
) {
  const result = []
  for (const filler of fillers) {
    result.push(filler)
    if (filler.content.kind === FillerKind.COMMENT_LINE) {
      result.push(new Filler(
        new FillerContent(FillerKind.NEWLINE, undefined),
        filler.position
      ))
    }
  }
  return result
}

export const fillerArrayStrategy = (
  fc.array(fillerStrategy, { maxLength: 5 }).map(
    mergeConsecutiveWhitespaceFillers
  ).map(toFillerArrayWithCommentLinesFollowedByNewlines)
)
