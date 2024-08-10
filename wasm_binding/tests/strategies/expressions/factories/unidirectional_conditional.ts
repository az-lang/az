import fc from 'fast-check'
import {
  Block, Filler, SubstringPosition, UnidirectionalConditional
} from '../../../../pkg'
import { Expression, UnidirectionalConditionalData } from '../../../types'
import { nonCommentFillerStrategy } from '../../filler'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'
import { toBlockStrategy } from './block'
import { toNonLexicallyConflictingExpression } from './utils'

function unidirectionalConditionalArgumentsToStrategy<
  Antecedent extends Expression,
  ConsequentExpression extends Expression,
  Output
> (
  factory: (
    antecedent: Antecedent,
    consequent: Block,
    openerPosition: SubstringPosition,
    openerFillers: Filler[],
  ) => Output,
  baseAntecedentStrategy: fc.Arbitrary<Antecedent>,
  consequentExpressionStrategy: fc.Arbitrary<ConsequentExpression>
) {
  const antecedentStrategy = fc.tuple(
    baseAntecedentStrategy, nonCommentFillerStrategy
  ).map(
    ([expression, nonCommentFiller]) => toNonLexicallyConflictingExpression(
      expression, nonCommentFiller
    )
  )
  const consequentStrategy = toBlockStrategy(consequentExpressionStrategy)
  return fc.tuple(
    antecedentStrategy,
    consequentStrategy,
    substringPositionStrategy,
    fillerArrayStrategy
  ).map(
    ([antecedent, consequent, openerPosition, openerFillers]) => factory(
      antecedent, consequent, openerPosition, openerFillers
    )
  )
}

export function toUnidirectionalConditionalDataStrategy<
  Antecedent extends Expression,
  ConsequentExpression extends Expression
> (
  baseAntecedentStrategy: fc.Arbitrary<Antecedent>,
  consequentExpressionStrategy: fc.Arbitrary<ConsequentExpression>
): fc.Arbitrary<UnidirectionalConditionalData<Antecedent>> {
  return unidirectionalConditionalArgumentsToStrategy(
    (antecedent, consequent, openerPosition, openerFillers) => (
      { antecedent, consequent, openerPosition, openerFillers }
    ),
    baseAntecedentStrategy,
    consequentExpressionStrategy
  )
}

export function toUnidirectionalConditionalStrategy<
  Antecedent extends Expression,
  ConsequentExpression extends Expression
> (
  baseAntecedentStrategy: fc.Arbitrary<Antecedent>,
  consequentExpressionStrategy: fc.Arbitrary<ConsequentExpression>
) {
  return unidirectionalConditionalArgumentsToStrategy(
    (antecedent, consequent, openerPosition, openerFillers) => {
      const result = new UnidirectionalConditional(
        antecedent, consequent, openerPosition, openerFillers
      )
      result.validateContents()
      return result
    },
    baseAntecedentStrategy,
    consequentExpressionStrategy
  )
}
