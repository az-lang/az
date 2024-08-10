import fc from 'fast-check'
import { Utf8Count } from '../pkg'
import { utf8CountDataStrategy, utf8CountStrategy } from './strategies'
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
            utf8CountDataStrategy,
            ({ value }) => new Utf8Count(value) instanceof Utf8Count
          )
        )
      }
    )
    test(
      'repeated',
      () => {
        fc.assert(
          fc.property(
            utf8CountDataStrategy,
            ({ value }) => new Utf8Count(value).equalTo(new Utf8Count(value))
          )
        )
      }
    )
    test(
      'round-trip',
      () => {
        fc.assert(
          fc.property(
            utf8CountStrategy,
            (value: Utf8Count) => new Utf8Count(value.valueOf()).equalTo(value)
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
      utf8CountStrategy, Utf8Count.MIN, Utf8Count.MAX
    )
    testRichComparableAlternativesToLessThan(utf8CountStrategy)
  }
)

describe(
  '`<=` tests',
  () => {
    testBoundedPartialOrderingOfLessThanOrEqualTo(
      utf8CountStrategy, Utf8Count.MIN, Utf8Count.MAX
    )
    testRichComparableAlternativesToLessThanOrEqualTo(utf8CountStrategy)
  }
)

describe(
  '`>` tests',
  () => {
    testBoundedStrictOrderingOfGreaterThan(
      utf8CountStrategy, Utf8Count.MIN, Utf8Count.MAX
    )
    testRichComparableAlternativesToGreaterThan(utf8CountStrategy)
  }
)

describe(
  '`>=` tests',
  () => {
    testBoundedPartialOrderingOfGreaterThanOrEqualTo(
      utf8CountStrategy, Utf8Count.MIN, Utf8Count.MAX
    )
    testRichComparableAlternativesToGreaterThanOrEqual(utf8CountStrategy)
  }
)

describe(
  '`equalTo` tests',
  () => {
    testEquivalenceOfEqualTo(utf8CountStrategy)
    testRichComparableAlternativesToEqualTo(utf8CountStrategy)
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
            utf8CountStrategy, (value) => typeof value.toString() === typeof ''
          )
        )
      }
    )
    test(
      '`eval` round-trip',
      () => {
        fc.assert(
          fc.property(
            utf8CountStrategy,
            (value: Utf8Count) => {
              const result = evalInContext(value.toString(), { Utf8Count })

              return result instanceof Utf8Count && result.equalTo(value)
            }
          )
        )
      }
    )
  }
)
