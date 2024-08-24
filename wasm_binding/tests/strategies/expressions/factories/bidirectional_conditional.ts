import fc from 'fast-check'
import {
  Block,
  BidirectionalConditional,
  Filler,
  SubstringPosition,
  UnidirectionalConditional
} from '../../../../pkg'
import { BidirectionalConditionalData, Expression } from '../../../types'
import { nonCommentFillerStrategy } from '../../filler'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'
import { toBlockStrategy } from './block'
import {
  toUnidirectionalConditionalStrategy
} from './unidirectional_conditional'
import { toNonLexicallyConflictingExpression } from './utils'

function conditionalArgumentsToStrategy<
  Antecedent extends Expression,
  ConsequentExpression extends Expression,
  AlternativeExpression extends Expression,
  Output
> (
  factory: (
    antecedent: Antecedent,
    consequent: Block,
    alternative: Block | BidirectionalConditional | UnidirectionalConditional,
    antecedentOpenerPosition: SubstringPosition,
    alternativeOpenerPosition: SubstringPosition,
    antecedentOpenerFillers: Filler[],
    alternativeOpenerFillers: Filler[]
  ) => Output,
  baseAntecedentStrategy: fc.Arbitrary<Antecedent>,
  consequentExpressionStrategy: fc.Arbitrary<ConsequentExpression>,
  alternativeExpressionStrategy: fc.Arbitrary<AlternativeExpression>
) {
  const antecedentStrategy = fc.tuple(
    baseAntecedentStrategy, nonCommentFillerStrategy
  ).map(
    ([expression, nonCommentFiller]) => toNonLexicallyConflictingExpression(
      expression, nonCommentFiller
    )
  )
  const consequentStrategy = toBlockStrategy(consequentExpressionStrategy)
  const finalAlternativeStrategy = fc.oneof(
    toBlockStrategy(alternativeExpressionStrategy),
    fc.tuple(
      toUnidirectionalConditionalStrategy(
        alternativeExpressionStrategy, alternativeExpressionStrategy
      ),
      nonCommentFillerStrategy
    ).map(
      ([unidirectionalConditional, nonCommentFiller]) => (
        unidirectionalConditional.openerFillers.length === 0
          ? new UnidirectionalConditional(
            unidirectionalConditional.antecedent,
            unidirectionalConditional.consequent,
            unidirectionalConditional.openerPosition,
            [nonCommentFiller]
          )
          : unidirectionalConditional
      )
    )
  )
  const variants: fc.Arbitrary<
    [
      Antecedent,
      Block,
      (BidirectionalConditional | Block | UnidirectionalConditional),
      SubstringPosition,
      SubstringPosition,
      Filler[],
      Filler[]
    ]
  >[] = [
    fc.tuple(
      antecedentStrategy,
      consequentStrategy,
      finalAlternativeStrategy,
      substringPositionStrategy,
      substringPositionStrategy,
      fillerArrayStrategy,
      fillerArrayStrategy
    )
  ]
  for (let _ = 0; _ < 3; _++) {
    variants.push(
      fc.tuple(
        antecedentStrategy,
        consequentStrategy,
        fc.tuple(variants[variants.length - 1]!, nonCommentFillerStrategy).map(
          (
            [
              [
                antecedent,
                consequent,
                alternative,
                antecedentOpenerPosition,
                alternativeOpenerPosition,
                antecedentOpenerFillers,
                alternativeOpenerFillers
              ],
              nonCommentFiller
            ]
          ) => new BidirectionalConditional(
            antecedent,
            consequent,
            alternative,
            antecedentOpenerPosition,
            alternativeOpenerPosition,
            antecedentOpenerFillers.length === 0
              ? [nonCommentFiller]
              : antecedentOpenerFillers,
            alternativeOpenerFillers
          )
        ),
        substringPositionStrategy,
        substringPositionStrategy,
        fillerArrayStrategy,
        fillerArrayStrategy
      )
    )
  }
  return fc.oneof(...variants).map(
    (
      [
        antecedent,
        consequent,
        alternative,
        antecedentOpenerPosition,
        alternativeOpenerPosition,
        antecedentOpenerFillers,
        alternativeOpenerFillers
      ]
    ) => factory(
      antecedent,
      consequent,
      alternative,
      antecedentOpenerPosition,
      alternativeOpenerPosition,
      antecedentOpenerFillers,
      alternativeOpenerFillers
    )
  )
}

export function toBidirectionalConditionalDataStrategy<
  Antecedent extends Expression,
  ConsequentExpression extends Expression,
  AlternativeExpression extends Expression
> (
  baseAntecedentStrategy: fc.Arbitrary<Antecedent>,
  consequentExpressionStrategy: fc.Arbitrary<ConsequentExpression>,
  alternativeExpressionStrategy: fc.Arbitrary<AlternativeExpression>
): fc.Arbitrary<BidirectionalConditionalData<Antecedent>> {
  return conditionalArgumentsToStrategy(
    (
      antecedent,
      consequent,
      alternative,
      antecedentOpenerPosition,
      alternativeOpenerPosition,
      antecedentOpenerFillers,
      alternativeOpenerFillers
    ) => (
      {
        antecedent,
        consequent,
        alternative,
        antecedentOpenerPosition,
        alternativeOpenerPosition,
        antecedentOpenerFillers,
        alternativeOpenerFillers
      }
    ),
    baseAntecedentStrategy,
    consequentExpressionStrategy,
    alternativeExpressionStrategy
  )
}

export function toBidirectionalConditionalStrategy<
  Antecedent extends Expression,
  ConsequentExpression extends Expression,
  AlternativeExpression extends Expression
> (
  baseAntecedentStrategy: fc.Arbitrary<Antecedent>,
  consequentExpressionStrategy: fc.Arbitrary<ConsequentExpression>,
  alternativeExpressionStrategy: fc.Arbitrary<AlternativeExpression>
) {
  return conditionalArgumentsToStrategy(
    (
      antecedent,
      consequent,
      alternative,
      antecedentOpenerPosition,
      alternativeOpenerPosition,
      antecedentOpenerFillers,
      alternativeOpenerFillers
    ) => {
      const result = new BidirectionalConditional(
        antecedent,
        consequent,
        alternative,
        antecedentOpenerPosition,
        alternativeOpenerPosition,
        antecedentOpenerFillers,
        alternativeOpenerFillers
      )
      result.validateContents()
      return result
    },
    baseAntecedentStrategy,
    consequentExpressionStrategy,
    alternativeExpressionStrategy
  )
}
