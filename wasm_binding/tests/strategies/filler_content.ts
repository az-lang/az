import fc from 'fast-check'
import { FillerContent, FillerKind } from '../../pkg'
import {
  commentBlockStringStrategy,
  commentLineStringStrategy,
  whitespaceStringStrategy
} from './token_content_state'

export const nonCommentFillerContentDataStrategy = fc.oneof(
  fc.oneof(
    fc.tuple().map(() => ({ kind: FillerKind.NEWLINE, state: undefined })),
    fc.tuple().map(() => ({ kind: FillerKind.NEWLINE, state: null }))
  ),
  whitespaceStringStrategy.map(
    (state) => ({ kind: FillerKind.WHITESPACE, state })
  )
)
export const fillerContentDataStrategy = fc.oneof(
  nonCommentFillerContentDataStrategy,
  commentLineStringStrategy.map(
    (state) => ({ kind: FillerKind.COMMENT_LINE, state })
  ),
  commentBlockStringStrategy.map(
    (state) => ({ kind: FillerKind.COMMENT_BLOCK, state })
  )
)
export const fillerContentStrategy = fillerContentDataStrategy.map(
  ({ kind, state }) => new FillerContent(kind, state)
)
export const nonCommentFillerContentStrategy = (
  nonCommentFillerContentDataStrategy.map(
    ({ kind, state }) => new FillerContent(kind, state)
  )
)
