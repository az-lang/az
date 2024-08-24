import fc from 'fast-check'
import { TokenContent, TokenKind } from '../../pkg'
import {
  commentBlockStringStrategy,
  commentLineStringStrategy,
  floatingPointLiteralValueStrategy,
  identifierStringStrategy,
  integerLiteralValueStrategy,
  whitespaceStringStrategy
} from './token_content_state'

export const STATELESS_TOKEN_KINDS = [
  TokenKind.ARROW,
  TokenKind.ASSIGNMENT,
  TokenKind.ASTERISK,
  TokenKind.CLOSE_BRACE,
  TokenKind.CLOSE_PARENTHESIS,
  TokenKind.COLON,
  TokenKind.COMMA,
  TokenKind.DOT,
  TokenKind.EQUAL_TO,
  TokenKind.GREATER_THAN,
  TokenKind.GREATER_THAN_OR_EQUAL_TO,
  TokenKind.LESS_THAN,
  TokenKind.LESS_THAN_OR_EQUAL_TO,
  TokenKind.MINUS,
  TokenKind.NEWLINE,
  TokenKind.NOT_EQUAL_TO,
  TokenKind.OPEN_BRACE,
  TokenKind.OPEN_PARENTHESIS,
  TokenKind.PLUS,
  TokenKind.SEMICOLON,
  TokenKind.SLASH
]
const statelessTokenKindStrategy = fc.constantFrom(...STATELESS_TOKEN_KINDS)
export const tokenContentDataStrategy = fc.oneof(
  statelessTokenKindStrategy.map((kind) => ({ kind, state: undefined })),
  statelessTokenKindStrategy.map((kind) => ({ kind, state: null })),
  commentLineStringStrategy.map(
    (state) => ({ kind: TokenKind.COMMENT_LINE, state })
  ),
  commentBlockStringStrategy.map(
    (state) => ({ kind: TokenKind.COMMENT_BLOCK, state })
  ),
  identifierStringStrategy.map(
    (state) => ({ kind: TokenKind.IDENTIFIER, state })
  ),
  fc.oneof(
    fc.tuple(
      fc.constantFrom(
        TokenKind.I8,
        TokenKind.I16,
        TokenKind.I32,
        TokenKind.I64,
        TokenKind.ISIZE,
        TokenKind.U8,
        TokenKind.U16,
        TokenKind.U32,
        TokenKind.U64,
        TokenKind.USIZE
      ),
      integerLiteralValueStrategy
    ),
    fc.tuple(
      fc.constantFrom(TokenKind.F32, TokenKind.F64),
      floatingPointLiteralValueStrategy
    )
  ).map(([kind, state]) => ({ kind, state })),
  whitespaceStringStrategy.map(
    (state) => ({ kind: TokenKind.WHITESPACE, state })
  )
)
export const tokenContentStrategy = tokenContentDataStrategy.map(
  ({ kind, state }) => new TokenContent(kind, state)
)
