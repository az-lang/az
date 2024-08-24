import fc from 'fast-check'
import { ByteCount } from '../pkg'
import { byteCountDataStrategy, byteCountStrategy } from './strategies'
import {
  evalInContext,
  testBoundedPartialOrderingOfGreaterThanOrEqualTo,
  testBoundedPartialOrderingOfLessThanOrEqualTo,
  testBoundedStrictOrderingOfGreaterThan,
  testBoundedStrictOrderingOfLessThan,
  testEquivalenceOfEqualTo,
  testRichComparableAlternativesToEqualTo,
  testRichComparableAlternativesToGreaterThan,
  testRichComparableAlternativesToGreaterThanOrEqual,
  testRichComparableAlternativesToLessThan,
  testRichComparableAlternativesToLessThanOrEqualTo
} from './utils'

describe(
  '`constructor` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            byteCountDataStrategy,
            ({ value }) => new ByteCount(value) instanceof ByteCount
          )
        )
      }
    )
    test(
      'repeated',
      () => {
        fc.assert(
          fc.property(
            byteCountDataStrategy,
            ({ value }) => new ByteCount(value).equalTo(new ByteCount(value))
          )
        )
      }
    )
    test(
      'round-trip',
      () => {
        fc.assert(
          fc.property(
            byteCountStrategy,
            (value: ByteCount) => new ByteCount(value.valueOf()).equalTo(value)
          )
        )
      }
    )
  }
)

describe(
  '`<` tests',
  () => {
    testBoundedStrictOrderingOfLessThan(
      byteCountStrategy, ByteCount.MIN, ByteCount.MAX
    )
    testRichComparableAlternativesToLessThan(byteCountStrategy)
  }
)

describe(
  '`<=` tests',
  () => {
    testBoundedPartialOrderingOfLessThanOrEqualTo(
      byteCountStrategy, ByteCount.MIN, ByteCount.MAX
    )
    testRichComparableAlternativesToLessThanOrEqualTo(byteCountStrategy)
  }
)

describe(
  '`>` tests',
  () => {
    testBoundedStrictOrderingOfGreaterThan(
      byteCountStrategy, ByteCount.MIN, ByteCount.MAX
    )
    testRichComparableAlternativesToGreaterThan(byteCountStrategy)
  }
)

describe(
  '`>=` tests',
  () => {
    testBoundedPartialOrderingOfGreaterThanOrEqualTo(
      byteCountStrategy, ByteCount.MIN, ByteCount.MAX
    )
    testRichComparableAlternativesToGreaterThanOrEqual(byteCountStrategy)
  }
)

describe(
  '`equalTo` tests',
  () => {
    testEquivalenceOfEqualTo(byteCountStrategy)
    testRichComparableAlternativesToEqualTo(byteCountStrategy)
  }
)

describe(
  '`toString` tests',
  () => {
    test(
      'basic',
      () => {
        fc.assert(
          fc.property(
            byteCountStrategy, (value) => typeof value.toString() === typeof ''
          )
        )
      }
    )
    test(
      '`eval` round-trip',
      () => {
        fc.assert(
          fc.property(
            byteCountStrategy,
            (value: ByteCount) => {
              const result = evalInContext(value.toString(), { ByteCount })

              return result instanceof ByteCount && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)
