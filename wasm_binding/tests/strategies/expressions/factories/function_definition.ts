import fc from 'fast-check'
import {
  AnnotatedIdentifier,
  Block,
  Filler,
  FunctionDefinition,
  SubstringPosition
} from '../../../../pkg'
import { Expression, FunctionDefinitionData } from '../../../types'
import { fillerArrayStrategy } from '../../filler_array'
import { substringPositionStrategy } from '../../substring_position'
import { toAnnotatedIdentifierStrategy } from './annotated_identifier'
import { toBlockStrategy } from './block'
import { MAX_EXPRESSIONS_ARRAY_LENGTH } from './constants'

function functionDefinitionArgumentsToStrategy<
  ParameterAnnotation extends Expression,
  ReturnType extends Expression,
  BodyExpression extends Expression,
  Output
> (
  factory: (
    parameters: AnnotatedIdentifier[],
    returnType: ReturnType,
    body: Block,
    openerPosition: SubstringPosition,
    openParenthesisPosition: SubstringPosition,
    commaPositions: SubstringPosition[],
    closeParenthesisPosition: SubstringPosition,
    arrowPosition: SubstringPosition,
    openerFillers: Filler[],
    openParenthesisFillers: Filler[],
    commaFillers: Filler[][],
    closeParenthesisFillers: Filler[],
    arrowFillers: Filler[]
  ) => Output,
  parameterAnnotationStrategy: fc.Arbitrary<ParameterAnnotation>,
  returnTypeStrategy: fc.Arbitrary<ReturnType>,
  bodyExpressionStrategy: fc.Arbitrary<BodyExpression>
) {
  const parametersStrategy = fc.array(
    toAnnotatedIdentifierStrategy(parameterAnnotationStrategy),
    { maxLength: MAX_EXPRESSIONS_ARRAY_LENGTH }
  )
  const bodyStrategy = toBlockStrategy(bodyExpressionStrategy)
  const commaPositionsStrategy = fc.array(
    substringPositionStrategy, { maxLength: MAX_EXPRESSIONS_ARRAY_LENGTH }
  )
  const commaFillersStrategy = fc.array(
    fillerArrayStrategy, { maxLength: MAX_EXPRESSIONS_ARRAY_LENGTH }
  )
  return fc.oneof(
    fc.tuple(
      parametersStrategy,
      returnTypeStrategy,
      bodyStrategy,
      substringPositionStrategy,
      substringPositionStrategy,
      commaPositionsStrategy,
      substringPositionStrategy,
      substringPositionStrategy,
      fillerArrayStrategy,
      fillerArrayStrategy,
      commaFillersStrategy,
      fillerArrayStrategy,
      fillerArrayStrategy
    ).map(
      (
        [
          parameters,
          returnType,
          body,
          openerPosition,
          openParenthesisPosition,
          commaPositions,
          closeParenthesisPosition,
          arrowPosition,
          openerFillers,
          openParenthesisFillers,
          commaFillers,
          closeParenthesisFillers,
          arrowFillers
        ]
      ) => {
        const parametersCount = Math.min(
          parameters.length, commaPositions.length
        )
        for (let _ = commaFillers.length; _ < parametersCount; _++) {
          commaFillers.push([])
        }
        return factory(
          parameters.slice(0, parametersCount),
          returnType,
          body,
          openerPosition,
          openParenthesisPosition,
          commaPositions.slice(0, parametersCount),
          closeParenthesisPosition,
          arrowPosition,
          openerFillers,
          openParenthesisFillers,
          commaFillers.slice(0, parametersCount),
          closeParenthesisFillers,
          arrowFillers
        )
      }),
    fc.tuple(
      parametersStrategy,
      returnTypeStrategy,
      bodyStrategy,
      substringPositionStrategy,
      substringPositionStrategy,
      commaPositionsStrategy,
      substringPositionStrategy,
      substringPositionStrategy,
      fillerArrayStrategy,
      fillerArrayStrategy,
      commaFillersStrategy,
      fillerArrayStrategy,
      fillerArrayStrategy
    ).map(
      (
        [
          parameters,
          returnType,
          body,
          openerPosition,
          openParenthesisPosition,
          commaPositions,
          closeParenthesisPosition,
          arrowPosition,
          openerFillers,
          openParenthesisFillers,
          commaFillers,
          closeParenthesisFillers,
          arrowFillers
        ]
      ) => {
        const parametersCount = Math.min(
          parameters.length, commaPositions.length
        )
        for (let _ = commaFillers.length; _ < parametersCount; _++) {
          commaFillers.push([])
        }
        return factory(
          parameters.slice(0, parametersCount),
          returnType,
          body,
          openerPosition,
          openParenthesisPosition,
          commaPositions.slice(0, Math.max(parametersCount - 1, 0)),
          closeParenthesisPosition,
          arrowPosition,
          openerFillers,
          openParenthesisFillers,
          commaFillers.slice(0, Math.max(parametersCount - 1, 0)),
          closeParenthesisFillers,
          arrowFillers
        )
      }
    )
  )
}

export function toFunctionDefinitionDataStrategy<
  ParameterAnnotation extends Expression,
  ReturnType extends Expression,
  BodyExpression extends Expression,
> (
  parameterAnnotationStrategy: fc.Arbitrary<ParameterAnnotation>,
  returnTypeStrategy: fc.Arbitrary<ReturnType>,
  bodyExpressionStrategy: fc.Arbitrary<BodyExpression>
): fc.Arbitrary<FunctionDefinitionData<ReturnType>> {
  return functionDefinitionArgumentsToStrategy(
    (
      parameters,
      returnType,
      body,
      openerPosition,
      openParenthesisPosition,
      commaPositions,
      closeParenthesisPosition,
      arrowPosition,
      openerFillers,
      openParenthesisFillers,
      commaFillers,
      closeParenthesisFillers,
      arrowFillers
    ) => (
      {
        parameters,
        returnType,
        body,
        openerPosition,
        openParenthesisPosition,
        commaPositions,
        closeParenthesisPosition,
        arrowPosition,
        openerFillers,
        openParenthesisFillers,
        commaFillers,
        closeParenthesisFillers,
        arrowFillers
      }
    ),
    parameterAnnotationStrategy,
    returnTypeStrategy,
    bodyExpressionStrategy
  )
}

export function toFunctionDefinitionStrategy<
  ParameterAnnotation extends Expression,
  ReturnType extends Expression,
  BodyExpression extends Expression,
> (
  parameterAnnotationStrategy: fc.Arbitrary<ParameterAnnotation>,
  returnTypeStrategy: fc.Arbitrary<ReturnType>,
  bodyExpressionStrategy: fc.Arbitrary<BodyExpression>
) {
  return functionDefinitionArgumentsToStrategy(
    (
      parameters,
      returnType,
      body,
      openerPosition,
      openParenthesisPosition,
      commaPositions,
      closeParenthesisPosition,
      arrowPosition,
      openerFillers,
      openParenthesisFillers,
      commaFillers,
      closeParenthesisFillers,
      arrowFillers
    ) => {
      const result = new FunctionDefinition(
        parameters,
        returnType,
        body,
        openerPosition,
        openParenthesisPosition,
        commaPositions,
        closeParenthesisPosition,
        arrowPosition,
        openerFillers,
        openParenthesisFillers,
        commaFillers,
        closeParenthesisFillers,
        arrowFillers
      )
      result.validateContents()
      return result
    },
    parameterAnnotationStrategy,
    returnTypeStrategy,
    bodyExpressionStrategy
  )
}
