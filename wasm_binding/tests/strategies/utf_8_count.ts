import fc from 'fast-check'
import { Utf8Count } from '../../pkg'

function utf8CountArgumentsToStrategy<Output> (
  factory: (value: number) => Output
): fc.Arbitrary<Output> {
  return fc.integer(
    { min: Utf8Count.MIN.valueOf(), max: Utf8Count.MAX.valueOf() }
  ).map(
    value => factory(value)
  )
}

export const utf8CountDataStrategy = utf8CountArgumentsToStrategy(
  value => ({ value })
)
export const utf8CountStrategy = utf8CountArgumentsToStrategy(
  value => new Utf8Count(value)
)
