import fc from 'fast-check'

export function evalInContext (
  code: string, context: Record<string, unknown>
): unknown {
  return (
    function (code: string): unknown {
      return eval(code) // eslint-disable-line no-eval
    }.call(
      context,
      Object.keys(context).map((key) => `const ${key} = this.${key};`).join('')
      + code
    )
  )
}

export interface Equatable<Other> {
  equalTo(other: Other): boolean
}

export function testBoundedPartialOrderingOfGreaterThanOrEqualTo<
  T extends Equatable<T>
>
(
  strategy: fc.Arbitrary<T>, minValue: T, maxValue: T
): void {
  test(
    'reflexivity',
    () => {
      fc.assert(fc.property(strategy, (value) => value >= value)) // eslint-disable-line no-self-compare
    }
  )
  test(
    'antisymmetry',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (
            !(first >= second && second >= first) || first.equalTo(second)
          )
        )
      )
    }
  )
  test(
    'transitivity',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          strategy,
          (first, second, third) => (
            !(first <= second && second <= third) || first <= third
          )
        )
      )
    }
  )
  test(
    'boundaries',
    () => {
      fc.assert(
        fc.property(
          strategy, (value) => maxValue >= value && value >= minValue
        )
      )
    }
  )
}

export function testBoundedPartialOrderingOfLessThanOrEqualTo<
  T extends Equatable<T>
> (strategy: fc.Arbitrary<T>, minValue: T, maxValue: T): void {
  test(
    'reflexivity',
    () => {
      fc.assert(fc.property(strategy, (value) => value <= value)) // eslint-disable-line no-self-compare
    }
  )
  test(
    'antisymmetry',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (
            !(first <= second && second <= first) || first.equalTo(second)
          )
        )
      )
    }
  )
  test(
    'transitivity',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          strategy,
          (first, second, third) => (
            !(first <= second && second <= third) || first <= third
          )
        )
      )
    }
  )
  test(
    'boundaries',
    () => {
      fc.assert(
        fc.property(
          strategy, (value) => minValue <= value && value <= maxValue
        )
      )
    }
  )
}

export function testBoundedStrictOrderingOfGreaterThan<T> (
  strategy: fc.Arbitrary<T>, minValue: T, maxValue: T
): void {
  test(
    'irreflexivity',
    () => {
      fc.assert(fc.property(strategy, (value) => !(value > value))) // eslint-disable-line no-self-compare
    }
  )
  test(
    'asymmetry',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => !(first > second) || !(second > first)
        )
      )
    }
  )
  test(
    'transitivity',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          strategy,
          (first, second, third) => (
            !(first > second && second > third) || first > third
          )
        )
      )
    }
  )
  test(
    'boundaries',
    () => {
      fc.assert(
        fc.property(
          strategy, (value) => !(minValue > value) && !(value > maxValue)
        )
      )
    }
  )
}

export function testBoundedStrictOrderingOfLessThan<T> (
  strategy: fc.Arbitrary<T>, minValue: T, maxValue: T
): void {
  test(
    'irreflexivity',
    () => {
      fc.assert(fc.property(strategy, (value) => !(value < value))) // eslint-disable-line no-self-compare
    }
  )
  test(
    'asymmetry',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => !(first < second) || !(second < first)
        )
      )
    }
  )
  test(
    'transitivity',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          strategy,
          (first, second, third) => (
            !(first < second && second < third) || first < third
          )
        )
      )
    }
  )
  test(
    'boundaries',
    () => {
      fc.assert(
        fc.property(
          strategy, (value) => !(value < minValue) && !(maxValue < value)
        )
      )
    }
  )
}

export function testEquivalenceOfEqualTo<
  T extends Equatable<T>
> (strategy: fc.Arbitrary<T>): void {
  test(
    'reflexivity',
    () => {
      fc.assert(fc.property(strategy, (value) => value.equalTo(value)))
    }
  )
  test(
    'symmetry',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (
            first.equalTo(second) === second.equalTo(first)
          )
        )
      )
    }
  )
  test(
    'transitivity',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          strategy,
          (first, second, third) => (
            !(first.equalTo(second) && second.equalTo(third))
            || first.equalTo(third)
          )
        )
      )
    }
  )
}

export function testRichComparableAlternativesToEqualTo<
  T extends Equatable<T>
> (strategy: fc.Arbitrary<T>): void {
  test(
    'alternatives',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (
            first.equalTo(second) === !(first < second || second < first)
          )
        )
      )
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (
            first.equalTo(second) === !(first > second || second > first)
          )
        )
      )
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (
            first.equalTo(second) === (first >= second && second >= first)
          )
        )
      )
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (
            first.equalTo(second) === (first <= second && second <= first)
          )
        )
      )
    }
  )
}

export function testRichComparableAlternativesToGreaterThan<
  T extends Equatable<T>
> (strategy: fc.Arbitrary<T>): void {
  test(
    'alternatives',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (first > second) === (second < first)
        )
      )
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (
            (first > second) === (
              first >= second && !first.equalTo(second)
            )
          )
        )
      )
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (first > second) === !(first <= second)
        )
      )
    }
  )
}

export function testRichComparableAlternativesToGreaterThanOrEqual<
  T extends Equatable<T>
> (strategy: fc.Arbitrary<T>): void {
  test(
    'alternatives',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (first >= second) === (second <= first)
        )
      )
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (
            (first >= second) === (first > second || first.equalTo(second))
          )
        )
      )
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (first >= second) === !(first < second)
        )
      )
    }
  )
}

export function testRichComparableAlternativesToLessThan<
  T extends Equatable<T>
> (strategy: fc.Arbitrary<T>): void {
  test(
    'alternatives',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (first < second) === (second > first)
        )
      )
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (
            (first < second) === (first <= second && !first.equalTo(second))
          )
        )
      )
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (first < second) === !(first >= second)
        )
      )
    }
  )
}

export function testRichComparableAlternativesToLessThanOrEqualTo<
  T extends Equatable<T>
> (strategy: fc.Arbitrary<T>): void {
  test(
    'alternatives',
    () => {
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (first <= second) === (second >= first)
        )
      )
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (
            (first <= second) === (first < second || first.equalTo(second))
          )
        )
      )
      fc.assert(
        fc.property(
          strategy,
          strategy,
          (first, second) => (first <= second) === !(first > second)
        )
      )
    }
  )
}
