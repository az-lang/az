import fc from 'fast-check'

export const commentLineStringStrategy = fc.stringMatching(/^\/\/.*$/)
export const commentBlockStringStrategy = fc.stringMatching(
  /^\/\*([^*]*\*+[^*/])*[^*]*\*\/$/s
)
export const whitespaceStringStrategy = fc.stringMatching(/^[^\S\n]+$/)

export const identifierStringStrategy = fc.stringMatching(
  /^[a-zA-Z_][a-zA-Z0-9_]*$/
)

const digits = '0123456789'
const digitStrategy = fc.constantFrom(...digits)
const maybeNonStartingDigitsStrategy = fc.stringOf(digitStrategy)
const nonZeroDigitStrategy = fc.constantFrom(...digits.slice(1))
export const integerLiteralValueStrategy = fc.oneof(
  fc.constant('0'),
  fc.tuple(
    nonZeroDigitStrategy, maybeNonStartingDigitsStrategy
  ).map(([firstDigit, restDigits]) => firstDigit + restDigits)
)

function toFloatingPointLiteralValueStrategy () {
  const nonStartingDigitsStrategy = fc.stringOf(
    digitStrategy, { minLength: 1 }
  )
  const variants = [
    integerLiteralValueStrategy,
    fc.tuple(integerLiteralValueStrategy, maybeNonStartingDigitsStrategy).map(
      ([integerPart, fractionalPart]) => (integerPart + '.' + fractionalPart)
    ),
    nonStartingDigitsStrategy.map(String.prototype.concat.bind('.'))
  ]
  const exponentSeparatorStrategy = fc.constantFrom(...'eE')
  const exponentSignStrategy = fc.constantFrom(...'+-', '')
  variants.push(
    ...variants.map(
      (valueStrategy) => fc.tuple(
        valueStrategy,
        exponentSeparatorStrategy,
        exponentSignStrategy,
        nonStartingDigitsStrategy
      ).map(
        ([value, exponentSeparator, exponentSign, exponentPower]) => (
          value + exponentSeparator + exponentSign + exponentPower
        )
      )
    )
  )
  return fc.oneof(...variants)
}

export const floatingPointLiteralValueStrategy = (
  toFloatingPointLiteralValueStrategy()
)
