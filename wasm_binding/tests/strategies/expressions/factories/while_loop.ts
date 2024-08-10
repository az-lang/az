import fc from 'fast-check'
import {
  Block, Filler, SubstringPosition, WhileLoop
} from '../../../../pkg'
import { Expression, WhileLoopData } from '../../../types'
import { nonCommentFillerStrategy } from '../../filler'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'
import { toBlockStrategy } from './block'
import { toNonLexicallyConflictingExpression } from './utils'

function whileLoopArgumentsToStrategy<
  Condition extends Expression,
  BodyExpression extends Expression,
  Output
> (
  factory: (
    condition: Condition,
    body: Block,
    openerPosition: SubstringPosition,
    openerFillers: Filler[],
  ) => Output,
  baseConditionStrategy: fc.Arbitrary<Condition>,
  bodyExpressionStrategy: fc.Arbitrary<BodyExpression>
) {
  const conditionStrategy = fc.tuple(
    baseConditionStrategy, nonCommentFillerStrategy
  ).map(
    ([expression, nonCommentFiller]) => toNonLexicallyConflictingExpression(
      expression, nonCommentFiller
    )
  )
  const bodyStrategy = toBlockStrategy(bodyExpressionStrategy)
  return fc.tuple(
    conditionStrategy,
    bodyStrategy,
    substringPositionStrategy,
    fillerArrayStrategy
  ).map(
    ([condition, body, openerPosition, openerFillers]) => factory(
      condition, body, openerPosition, openerFillers
    )
  )
}

export function toWhileLoopDataStrategy<
  Condition extends Expression,
  BodyExpression extends Expression
> (
  baseConditionStrategy: fc.Arbitrary<Condition>,
  bodyExpressionStrategy: fc.Arbitrary<BodyExpression>
): fc.Arbitrary<WhileLoopData<Condition>> {
  return whileLoopArgumentsToStrategy(
    (condition, body, openerPosition, openerFillers) => (
      { condition, body, openerPosition, openerFillers }
    ),
    baseConditionStrategy,
    bodyExpressionStrategy
  )
}

export function toWhileLoopStrategy<
  Condition extends Expression,
  BodyExpression extends Expression
> (
  baseConditionStrategy: fc.Arbitrary<Condition>,
  bodyExpressionStrategy: fc.Arbitrary<BodyExpression>
) {
  return whileLoopArgumentsToStrategy(
    (condition, body, openerPosition, openerFillers) => {
      const result = new WhileLoop(
        condition, body, openerPosition, openerFillers
      )
      result.validateContents()
      return result
    },
    baseConditionStrategy,
    bodyExpressionStrategy
  )
}
