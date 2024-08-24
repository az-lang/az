import fc from 'fast-check'
import { ByteCount } from '../../pkg'

function byteCountArgumentsToStrategy<Output> (
  factory: (value: number) => Output
): fc.Arbitrary<Output> {
  return fc.integer(
    { min: ByteCount.MIN.valueOf(), max: ByteCount.MAX.valueOf() }
  ).map(
    value => factory(value)
  )
}

export const byteCountDataStrategy = byteCountArgumentsToStrategy(
  value => ({ value })
)
export const byteCountStrategy = byteCountArgumentsToStrategy(
  value => new ByteCount(value)
)
