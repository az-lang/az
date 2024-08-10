import fc from 'fast-check'
import {
  AnnotatedIdentifier,
  AnnotationOperator,
  Assignment,
  AssignmentOperator,
  Associativity,
  BinaryAdditionOperator,
  BinaryArithmeticOperation,
  BinaryComparison,
  BinaryDivisionOperator,
  BinaryEqualToOperator,
  BinaryGreaterThanOperator,
  BinaryGreaterThanOrEqualToOperator,
  BinaryLessThanOperator,
  BinaryLessThanOrEqualToOperator,
  BinaryMultiplicationOperator,
  BinaryNotEqualToOperator,
  BinarySubtractionOperator,
  Block,
  Call,
  CallOperator,
  BidirectionalConditional,
  FunctionDefinition,
  Grouping,
  MemberAccess,
  MemberAccessOperator,
  Precedence,
  Return,
  ReturnOperator,
  Tuple,
  UnaryArithmeticOperation,
  UnaryNegationOperator,
  UnidirectionalConditional,
  WhileLoop
} from '../../../pkg'
import {
  AnnotatedIdentifierData,
  AssignmentData,
  BidirectionalConditionalData,
  BinaryArithmeticOperationData,
  BinaryArithmeticOperator,
  BinaryComparisonData,
  BinaryComparisonOperator,
  BinaryOperator,
  BlockData,
  CallData,
  Expression,
  FunctionDefinitionData,
  GroupingData,
  MemberAccessData,
  ReturnData,
  TupleData,
  UnaryArithmeticOperationData,
  UnidirectionalConditionalData,
  WhileLoopData
} from '../../types'
import { Equatable } from '../../utils'
import {
  toAnnotatedIdentifierDataStrategy,
  toAnnotatedIdentifierStrategy,
  toAssignmentDataStrategy,
  toAssignmentStrategy,
  toBidirectionalConditionalDataStrategy,
  toBidirectionalConditionalStrategy,
  toBinaryArithmeticOperationDataStrategy,
  toBinaryArithmeticOperationStrategy,
  toBinaryComparisonDataStrategy,
  toBinaryComparisonStrategy,
  toBlockDataStrategy,
  toBlockStrategy,
  toCallDataStrategy,
  toCallStrategy,
  toFunctionDefinitionDataStrategy,
  toFunctionDefinitionStrategy,
  toGroupingDataStrategy,
  toGroupingStrategy,
  toMemberAccessDataStrategy,
  toMemberAccessStrategy,
  toReturnDataStrategy,
  toReturnStrategy,
  toTupleDataStrategy,
  toTupleStrategy,
  toUnaryArithmeticOperationDataStrategy,
  toUnaryArithmeticOperationStrategy,
  toUnidirectionalConditionalDataStrategy,
  toUnidirectionalConditionalStrategy,
  toWhileLoopDataStrategy,
  toWhileLoopStrategy
} from './factories'
import { identifierStrategy } from './identifier'
import { numericLiteralStrategy } from './numeric_literal'

abstract class ExpressionKind< // eslint-disable-line @typescript-eslint/no-extraneous-class
  Data extends object,
  ExpressionT extends Expression,
  HasPrecedence extends boolean
> {
  constructor () {
    if (new.target === ExpressionKind) {
      throw new Error(
        `${ExpressionKind.constructor.name} is an abstract class.`
      )
    }
  }

  abstract get factoryArgumentMinPrecedenceIndices(): readonly number[]

  abstract get precedence(): (HasPrecedence extends true ? Precedence : null)

  abstract dataFactory(
    argumentStrategyArray: readonly fc.Arbitrary<Expression>[]
  ): fc.Arbitrary<Data>

  abstract factory(
    argumentStrategyArray: readonly fc.Arbitrary<Expression>[]
  ): fc.Arbitrary<ExpressionT>
}

function binaryOperatorToFactoryArgumentMinPrecedenceIndices (
  operator: BinaryOperator
): readonly [number, number] {
  const operatorPrecedenceIndex = toPrecedenceIndex(operator.PRECEDENCE)
  return (
    operator.ASSOCIATIVITY === Associativity.LEFT_TO_RIGHT
      ? [
          operatorPrecedenceIndex,
          operatorPrecedenceIndex + 1
        ]
      : [
          operatorPrecedenceIndex + 1,
          operatorPrecedenceIndex
        ]
  )
}

class AnnotatedIdentifierExpressionKind extends ExpressionKind<
  AnnotatedIdentifierData<Expression>, AnnotatedIdentifier, true
> {
  get factoryArgumentMinPrecedenceIndices (): readonly [number] {
    return [toPrecedenceIndex(AnnotationOperator.PRECEDENCE)]
  }

  get precedence (): Precedence {
    return AnnotationOperator.PRECEDENCE
  }

  override dataFactory (
    [annotationStrategy]: readonly [fc.Arbitrary<Expression>]
  ): fc.Arbitrary<AnnotatedIdentifierData<Expression>> {
    return toAnnotatedIdentifierDataStrategy(annotationStrategy)
  }

  override factory (
    [annotationStrategy]: readonly [fc.Arbitrary<Expression>]
  ): fc.Arbitrary<AnnotatedIdentifier> {
    return toAnnotatedIdentifierStrategy(annotationStrategy)
  }

  override toString (): string {
    return `new ${this.constructor.name}()`
  }
}

class AssignmentExpressionKind extends ExpressionKind<
  AssignmentData<Expression, Expression>, Assignment, true
> {
  get factoryArgumentMinPrecedenceIndices (): readonly [number, number] {
    return binaryOperatorToFactoryArgumentMinPrecedenceIndices(
      AssignmentOperator
    )
  }

  get precedence (): Precedence {
    return AssignmentOperator.PRECEDENCE
  }

  override dataFactory (
    [targetStrategy, valueStrategy]: readonly [
      fc.Arbitrary<Expression>, fc.Arbitrary<Expression>
    ]
  ): fc.Arbitrary<AssignmentData<Expression, Expression>> {
    return toAssignmentDataStrategy(targetStrategy, valueStrategy)
  }

  override factory (
    [targetStrategy, valueStrategy]: readonly [
      fc.Arbitrary<Expression>, fc.Arbitrary<Expression>
    ]
  ): fc.Arbitrary<Assignment> {
    return toAssignmentStrategy(targetStrategy, valueStrategy)
  }

  override toString (): string {
    return `new ${this.constructor.name}()`
  }
}

const NO_PRECEDENCE = null

class BidirectionalConditionalExpressionKind extends ExpressionKind<
  BidirectionalConditionalData<Expression>, BidirectionalConditional, false
> {
  get factoryArgumentMinPrecedenceIndices (): readonly [
    number, number, number
    ] {
    return [0, 0, 0]
  }

  get precedence (): null {
    return NO_PRECEDENCE
  }

  override dataFactory (
    [
      baseAntecedentStrategy,
      consequentExpressionStrategy,
      alternativeExpressionStrategy
    ]: readonly [
      fc.Arbitrary<Expression>,
      fc.Arbitrary<Expression>,
      fc.Arbitrary<Expression>
    ]
  ): fc.Arbitrary<BidirectionalConditionalData<Expression>> {
    return toBidirectionalConditionalDataStrategy(
      baseAntecedentStrategy,
      consequentExpressionStrategy,
      alternativeExpressionStrategy
    )
  }

  override factory (
    [
      baseAntecedentStrategy,
      consequentExpressionStrategy,
      alternativeExpressionStrategy
    ]: readonly [
      fc.Arbitrary<Expression>,
      fc.Arbitrary<Expression>,
      fc.Arbitrary<Expression>
    ]
  ): fc.Arbitrary<BidirectionalConditional> {
    return toBidirectionalConditionalStrategy(
      baseAntecedentStrategy,
      consequentExpressionStrategy,
      alternativeExpressionStrategy
    )
  }

  override toString (): string {
    return `new ${this.constructor.name}()`
  }
}

class BinaryArithmeticOperationExpressionKind extends ExpressionKind<
  BinaryArithmeticOperationData<Expression, Expression>,
  BinaryArithmeticOperation,
  true
> {
  operator: BinaryArithmeticOperator

  constructor (operator: BinaryArithmeticOperator) {
    super()
    this.operator = operator
  }

  get factoryArgumentMinPrecedenceIndices (): readonly [number, number] {
    return binaryOperatorToFactoryArgumentMinPrecedenceIndices(this.operator)
  }

  get precedence (): Precedence {
    return this.operator.PRECEDENCE
  }

  override dataFactory (
    [leftOperandStrategy, rightOperandStrategy]: readonly [
      fc.Arbitrary<Expression>, fc.Arbitrary<Expression>
    ]
  ): fc.Arbitrary<BinaryArithmeticOperationData<Expression, Expression>> {
    return toBinaryArithmeticOperationDataStrategy(
      this.operator, leftOperandStrategy, rightOperandStrategy
    )
  }

  override factory (
    [leftOperandStrategy, rightOperandStrategy]: readonly [
      fc.Arbitrary<Expression>, fc.Arbitrary<Expression>
    ]
  ): fc.Arbitrary<BinaryArithmeticOperation> {
    return toBinaryArithmeticOperationStrategy(
      this.operator, leftOperandStrategy, rightOperandStrategy
    )
  }

  override toString (): string {
    return `new ${this.constructor.name}(${this.operator.name})`
  }
}

class BinaryComparisonExpressionKind extends ExpressionKind<
  BinaryComparisonData<Expression, Expression>, BinaryComparison, true
> {
  operator: BinaryComparisonOperator

  constructor (operator: BinaryComparisonOperator) {
    super()
    this.operator = operator
  }

  get factoryArgumentMinPrecedenceIndices (): readonly [number, number] {
    return binaryOperatorToFactoryArgumentMinPrecedenceIndices(this.operator)
  }

  get precedence (): Precedence {
    return this.operator.PRECEDENCE
  }

  override dataFactory (
    [leftOperandStrategy, rightOperandStrategy]: readonly [
      fc.Arbitrary<Expression>, fc.Arbitrary<Expression>
    ]
  ): fc.Arbitrary<BinaryComparisonData<Expression, Expression>> {
    return toBinaryComparisonDataStrategy(
      this.operator, leftOperandStrategy, rightOperandStrategy
    )
  }

  override factory (
    [leftOperandStrategy, rightOperandStrategy]: readonly [
      fc.Arbitrary<Expression>, fc.Arbitrary<Expression>
    ]
  ): fc.Arbitrary<BinaryComparison> {
    return toBinaryComparisonStrategy(
      this.operator, leftOperandStrategy, rightOperandStrategy
    )
  }

  override toString (): string {
    return `new ${this.constructor.name}(${this.operator.name})`
  }
}

class BlockExpressionKind extends ExpressionKind<
  BlockData<Expression>, Block, false
> {
  get factoryArgumentMinPrecedenceIndices (): readonly [number] {
    return [0]
  }

  get precedence (): null {
    return NO_PRECEDENCE
  }

  override dataFactory (
    [expressionStrategy]: readonly [fc.Arbitrary<Expression>]
  ): fc.Arbitrary<BlockData<Expression>> {
    return toBlockDataStrategy(expressionStrategy)
  }

  override factory (
    [expressionStrategy]: readonly [fc.Arbitrary<Expression>]
  ): fc.Arbitrary<Block> {
    return toBlockStrategy(expressionStrategy)
  }

  override toString (): string {
    return `new ${this.constructor.name}()`
  }
}

class CallExpressionKind extends ExpressionKind<
  CallData<Expression, Expression>, Call, true
> {
  get factoryArgumentMinPrecedenceIndices (): readonly [number, number] {
    return [toPrecedenceIndex(this.precedence), 0]
  }

  get precedence (): Precedence {
    return CallOperator.PRECEDENCE
  }

  override dataFactory (
    [callableStrategy, argumentStrategy]: readonly [
      fc.Arbitrary<Expression>, fc.Arbitrary<Expression>
    ]
  ): fc.Arbitrary<CallData<Expression, Expression>> {
    return toCallDataStrategy(callableStrategy, argumentStrategy)
  }

  override factory (
    [callableStrategy, argumentStrategy]: readonly [
      fc.Arbitrary<Expression>, fc.Arbitrary<Expression>
    ]
  ): fc.Arbitrary<Call> {
    return toCallStrategy(callableStrategy, argumentStrategy)
  }

  override toString (): string {
    return `new ${this.constructor.name}()`
  }
}

class FunctionDefinitionExpressionKind extends ExpressionKind<
  FunctionDefinitionData<Expression>, FunctionDefinition, true
> {
  get factoryArgumentMinPrecedenceIndices (): readonly [
    number, number, number
    ] {
    return [toPrecedenceIndex(this.precedence), 0, 0]
  }

  get precedence (): Precedence {
    return AnnotationOperator.PRECEDENCE
  }

  override dataFactory (
    [parameterAnnotationStrategy, returnTypeStrategy, bodyExpressionStrategy]:
      readonly [
        fc.Arbitrary<Expression>,
        fc.Arbitrary<Expression>,
        fc.Arbitrary<Expression>
      ]
  ): fc.Arbitrary<FunctionDefinitionData<Expression>> {
    return toFunctionDefinitionDataStrategy(
      parameterAnnotationStrategy, returnTypeStrategy, bodyExpressionStrategy
    )
  }

  override factory (
    [parameterAnnotationStrategy, returnTypeStrategy, bodyExpressionStrategy]:
      readonly [
        fc.Arbitrary<Expression>,
        fc.Arbitrary<Expression>,
        fc.Arbitrary<Expression>
      ]
  ): fc.Arbitrary<FunctionDefinition> {
    return toFunctionDefinitionStrategy(
      parameterAnnotationStrategy, returnTypeStrategy, bodyExpressionStrategy
    )
  }

  override toString (): string {
    return `new ${this.constructor.name}()`
  }
}

class GroupingExpressionKind extends ExpressionKind<
  GroupingData<Expression>, Grouping, false
> {
  get factoryArgumentMinPrecedenceIndices (): readonly [number] {
    return [0]
  }

  get precedence (): null {
    return NO_PRECEDENCE
  }

  override dataFactory (
    [expressionStrategy]: readonly [fc.Arbitrary<Expression>]
  ): fc.Arbitrary<GroupingData<Expression>> {
    return toGroupingDataStrategy(expressionStrategy)
  }

  override factory (
    [expressionStrategy]: readonly [fc.Arbitrary<Expression>]
  ): fc.Arbitrary<Grouping> {
    return toGroupingStrategy(expressionStrategy)
  }

  override toString (): string {
    return `new ${this.constructor.name}()`
  }
}

class MemberAccessExpressionKind extends ExpressionKind<
  MemberAccessData<Expression>, MemberAccess, true
> {
  get factoryArgumentMinPrecedenceIndices (): readonly [number] {
    return [toPrecedenceIndex(this.precedence)]
  }

  get precedence (): Precedence {
    return MemberAccessOperator.PRECEDENCE
  }

  override dataFactory (
    [objectStrategy]: readonly [fc.Arbitrary<Expression>]
  ): fc.Arbitrary<MemberAccessData<Expression>> {
    return toMemberAccessDataStrategy(objectStrategy)
  }

  override factory (
    [objectStrategy]: readonly [fc.Arbitrary<Expression>]
  ): fc.Arbitrary<MemberAccess> {
    return toMemberAccessStrategy(objectStrategy)
  }

  override toString (): string {
    return `new ${this.constructor.name}()`
  }
}

class ReturnExpressionKind extends ExpressionKind<
  ReturnData<Expression>, Return, true
> {
  get factoryArgumentMinPrecedenceIndices (): readonly [number] {
    return [toPrecedenceIndex(this.precedence)]
  }

  get precedence (): Precedence {
    return ReturnOperator.PRECEDENCE
  }

  override dataFactory (
    [expressionStrategy]: readonly [fc.Arbitrary<Expression>]
  ): fc.Arbitrary<ReturnData<Expression>> {
    return toReturnDataStrategy(expressionStrategy)
  }

  override factory (
    [expressionStrategy]: readonly [fc.Arbitrary<Expression>]
  ): fc.Arbitrary<Return> {
    return toReturnStrategy(expressionStrategy)
  }
}

class TupleExpressionKind extends ExpressionKind<
  TupleData<Expression>, Tuple, false
> {
  get factoryArgumentMinPrecedenceIndices (): readonly [number] {
    return [0]
  }

  get precedence (): null {
    return NO_PRECEDENCE
  }

  override dataFactory (
    [elementStrategy]: readonly [fc.Arbitrary<Expression>]
  ): fc.Arbitrary<TupleData<Expression>> {
    return toTupleDataStrategy(elementStrategy)
  }

  override factory (
    [elementStrategy]: readonly [fc.Arbitrary<Expression>]
  ): fc.Arbitrary<Tuple> {
    return toTupleStrategy(elementStrategy)
  }

  override toString (): string {
    return `new ${this.constructor.name}()`
  }
}

type UnaryArithmeticOperator = typeof UnaryNegationOperator;

class UnaryArithmeticOperationExpressionKind extends ExpressionKind<
  UnaryArithmeticOperationData<Expression>, UnaryArithmeticOperation, true
> {
  operator: UnaryArithmeticOperator

  constructor (operator: UnaryArithmeticOperator) {
    super()
    this.operator = operator
  }

  get factoryArgumentMinPrecedenceIndices (): readonly [number] {
    return [toPrecedenceIndex(this.precedence)]
  }

  get precedence (): Precedence {
    return this.operator.PRECEDENCE
  }

  override dataFactory (
    [operandStrategy]: readonly [fc.Arbitrary<Expression>]
  ): fc.Arbitrary<UnaryArithmeticOperationData<Expression>> {
    return toUnaryArithmeticOperationDataStrategy(
      this.operator, operandStrategy
    )
  }

  override factory (
    [operandStrategy]: readonly [fc.Arbitrary<Expression>]
  ): fc.Arbitrary<UnaryArithmeticOperation> {
    return toUnaryArithmeticOperationStrategy(this.operator, operandStrategy)
  }
}

class UnidirectionalConditionalExpressionKind extends ExpressionKind<
  UnidirectionalConditionalData<Expression>, UnidirectionalConditional, false
> {
  get factoryArgumentMinPrecedenceIndices (): readonly [number, number] {
    return [0, 0]
  }

  get precedence (): null {
    return NO_PRECEDENCE
  }

  override dataFactory (
    [baseAntecedentStrategy, consequentExpressionStrategy]: readonly [
      fc.Arbitrary<Expression>, fc.Arbitrary<Expression>
    ]
  ): fc.Arbitrary<UnidirectionalConditionalData<Expression>> {
    return toUnidirectionalConditionalDataStrategy(
      baseAntecedentStrategy, consequentExpressionStrategy
    )
  }

  override factory (
    [baseAntecedentStrategy, consequentExpressionStrategy]: readonly [
      fc.Arbitrary<Expression>, fc.Arbitrary<Expression>
    ]
  ): fc.Arbitrary<UnidirectionalConditional> {
    return toUnidirectionalConditionalStrategy(
      baseAntecedentStrategy, consequentExpressionStrategy
    )
  }

  override toString (): string {
    return `new ${this.constructor.name}()`
  }
}

class WhileLoopExpressionKind extends ExpressionKind<
  WhileLoopData<Expression>, WhileLoop, false
> {
  get factoryArgumentMinPrecedenceIndices (): readonly [number, number] {
    return [0, 0]
  }

  get precedence (): null {
    return NO_PRECEDENCE
  }

  override dataFactory (
    [baseAntecedentStrategy, consequentExpressionStrategy]: readonly [
      fc.Arbitrary<Expression>, fc.Arbitrary<Expression>
    ]
  ): fc.Arbitrary<WhileLoopData<Expression>> {
    return toWhileLoopDataStrategy(
      baseAntecedentStrategy, consequentExpressionStrategy
    )
  }

  override factory (
    [baseAntecedentStrategy, consequentExpressionStrategy]: readonly [
      fc.Arbitrary<Expression>, fc.Arbitrary<Expression>
    ]
  ): fc.Arbitrary<WhileLoop> {
    return toWhileLoopStrategy(
      baseAntecedentStrategy, consequentExpressionStrategy
    )
  }

  override toString (): string {
    return `new ${this.constructor.name}()`
  }
}

const annotatedIdentifierExpressionKind = (
  new AnnotatedIdentifierExpressionKind()
)
const assignmentExpressionKind = new AssignmentExpressionKind()
const callExpressionKind = new CallExpressionKind()
const functionDefinitionExpressionKind = new FunctionDefinitionExpressionKind()
const memberAccessExpressionKind = new MemberAccessExpressionKind()
const returnExpressionKind = new ReturnExpressionKind()
const operationKinds
  : ExpressionKind<object, Expression, true>[] = [
    annotatedIdentifierExpressionKind,
    assignmentExpressionKind,
    callExpressionKind,
    functionDefinitionExpressionKind,
    memberAccessExpressionKind,
    returnExpressionKind
  ]
const binaryArithmeticOperationExpressionKinds = [
  BinaryAdditionOperator,
  BinaryDivisionOperator,
  BinaryMultiplicationOperator,
  BinarySubtractionOperator
].map(
  (operator) => new BinaryArithmeticOperationExpressionKind(operator)
)
operationKinds.push(...binaryArithmeticOperationExpressionKinds)
const binaryComparisonExpressionKinds = [
  BinaryEqualToOperator,
  BinaryGreaterThanOperator,
  BinaryGreaterThanOrEqualToOperator,
  BinaryLessThanOperator,
  BinaryLessThanOrEqualToOperator,
  BinaryNotEqualToOperator
].map(
  (operator) => new BinaryComparisonExpressionKind(operator)
)
operationKinds.push(...binaryComparisonExpressionKinds)
const UNARY_ARITHMETIC_OPERATORS = [
  UnaryNegationOperator
]
const unaryArithmeticOperationExpressionKinds = UNARY_ARITHMETIC_OPERATORS.map(
  (operator) => new UnaryArithmeticOperationExpressionKind(operator)
)
operationKinds.push(...unaryArithmeticOperationExpressionKinds)

const bidirectionalConditionalExpressionKind = (
  new BidirectionalConditionalExpressionKind()
)
const blockExpressionKind = new BlockExpressionKind()
const groupingExpressionKind = new GroupingExpressionKind()
const tupleExpressionKind = new TupleExpressionKind()
const unidirectionalConditionalExpressionKind = (
  new UnidirectionalConditionalExpressionKind()
)
const whileLoopExpressionKind = new WhileLoopExpressionKind()
const restInternalNodeExpressionKinds: (
  readonly ExpressionKind<object, Expression, false>[]
  ) = [
    bidirectionalConditionalExpressionKind,
    blockExpressionKind,
    groupingExpressionKind,
    tupleExpressionKind,
    unidirectionalConditionalExpressionKind
  ]

function toUniqueConsecutiveArray<T extends Equatable<T>> (
  array: readonly T[]
): T[] {
  if (array.length === 0) {
    return []
  }
  let cursor = array[0]!
  const result = [cursor]
  for (const element of array) {
    if (!element.equalTo(cursor)) {
      result.push(element)
      cursor = element
    }
  }
  return result
}

const precedences = toUniqueConsecutiveArray(
  operationKinds.map(
    (kind) => kind.precedence
  ).sort(
    (left, right) => left.lessThan(right) ? -1 : (right.lessThan(left) ? 1 : 0)
  )
)

type ExpressionKey = string

function precedenceIndexToExpressionKey (
  precedenceIndex: number | null
): ExpressionKey {
  return `node${
    precedenceIndex === null ? String(null) : precedenceIndex.toString()
  }`
}

const EXPRESSION_WITHOUT_PRECEDENCE_KEY = precedenceIndexToExpressionKey(
  NO_PRECEDENCE
)
const EXPRESSION_DEPTH_IDENTIFIER = 'expressionDepth'
const toPrecedenceIndex = (precedence: Precedence) => precedences.findIndex(
  (candidate) => candidate.equalTo(precedence)
)
const leafExpressionStrategy = fc.oneof(
  identifierStrategy, numericLiteralStrategy
)
const expressionStrategyMapping = fc.letrec(
  (tie) => {
    const keyToExpressionStrategy = (key: string) => (
      tie(key) as fc.Arbitrary<Expression>
    )
    const result: Record<ExpressionKey, fc.Arbitrary<Expression>> = {
      leaf: leafExpressionStrategy,
      node: fc.oneof(
        { depthIdentifier: EXPRESSION_DEPTH_IDENTIFIER },
        ...precedences.map(
          (_precedence, index) => keyToExpressionStrategy(
            precedenceIndexToExpressionKey(index)
          )
        ),
        keyToExpressionStrategy(EXPRESSION_WITHOUT_PRECEDENCE_KEY)
      ),
      tree: fc.oneof(
        { depthIdentifier: EXPRESSION_DEPTH_IDENTIFIER },
        keyToExpressionStrategy('leaf'),
        keyToExpressionStrategy('node')
      )
    }
    const expressionWithoutPrecedenceStrategy = fc.oneof(
      { depthIdentifier: EXPRESSION_DEPTH_IDENTIFIER },
      leafExpressionStrategy,
      ...restInternalNodeExpressionKinds.map(
        (kind) => kind.factory(
          kind.factoryArgumentMinPrecedenceIndices.map(
            (precedenceIndex) => (
              fc.oneof(
                { depthIdentifier: EXPRESSION_DEPTH_IDENTIFIER },
                leafExpressionStrategy,
                ...precedences.slice(precedenceIndex).map(
                  (_precedence, index) => keyToExpressionStrategy(
                    precedenceIndexToExpressionKey(precedenceIndex + index)
                  )
                ),
                keyToExpressionStrategy(EXPRESSION_WITHOUT_PRECEDENCE_KEY)
              )
            )
          )
        )
      ),
      keyToExpressionStrategy(EXPRESSION_WITHOUT_PRECEDENCE_KEY)
    )
    result[EXPRESSION_WITHOUT_PRECEDENCE_KEY] = (
      expressionWithoutPrecedenceStrategy
    )
    const expressionKindsByPrecedenceIndex = (
      new Array(precedences.length) as ExpressionKind<
        object, Expression, true
      >[][]
    )
    for (const kind of operationKinds) {
      const precedenceIndex = toPrecedenceIndex(kind.precedence)
      const group = expressionKindsByPrecedenceIndex[precedenceIndex] ??= []
      group.push(kind)
    }
    expressionKindsByPrecedenceIndex.forEach(
      (kinds, precedenceIndex) => {
        result[precedenceIndexToExpressionKey(precedenceIndex)] = fc.oneof(
          ...kinds.map(
            (kind) => kind.factory(
              kind.factoryArgumentMinPrecedenceIndices.map(
                (precedenceIndex) => (
                  fc.oneof(
                    { depthIdentifier: EXPRESSION_DEPTH_IDENTIFIER },
                    leafExpressionStrategy,
                    ...precedences.slice(precedenceIndex).map(
                      (_precedence, index) => keyToExpressionStrategy(
                        precedenceIndexToExpressionKey(precedenceIndex + index)
                      )
                    ),
                    expressionWithoutPrecedenceStrategy
                  )
                )
              )
            )
          )
        )
      }
    )
    return result
  }
)

export const expressionStrategy = expressionStrategyMapping['tree']!

export function binaryOperatorToOperandStrategyPair (
  operator: BinaryOperator
): readonly [fc.Arbitrary<Expression>, fc.Arbitrary<Expression>] {
  const baseOperandStrategy = precedenceToChildExpressionStrategy(
    operator.PRECEDENCE
  )
  return operator.ASSOCIATIVITY === Associativity.LEFT_TO_RIGHT
    ? [
        fc.oneof(
          { depthIdentifier: EXPRESSION_DEPTH_IDENTIFIER },
          precedenceToExpressionStrategy(operator.PRECEDENCE),
          baseOperandStrategy
        ),
        baseOperandStrategy
      ]
    : [
        baseOperandStrategy,
        fc.oneof(
          { depthIdentifier: EXPRESSION_DEPTH_IDENTIFIER },
          baseOperandStrategy,
          precedenceToExpressionStrategy(operator.PRECEDENCE)
        )
      ]
}

function precedenceToChildExpressionStrategy (
  precedence: Precedence
): fc.Arbitrary<Expression> {
  const precedenceIndex = toPrecedenceIndex(precedence)
  return fc.oneof(
    { depthIdentifier: EXPRESSION_DEPTH_IDENTIFIER },
    leafExpressionStrategy,
    ...precedences.slice(precedenceIndex + 1).map(
      (_precedence, index) => expressionStrategyMapping[
        precedenceIndexToExpressionKey(precedenceIndex + 1 + index)
      ]!
    ),
    expressionStrategyMapping[EXPRESSION_WITHOUT_PRECEDENCE_KEY]!
  )
}

export const precedenceToExpressionStrategy = (
  precedence: Precedence
): fc.Arbitrary<Expression> => (
  expressionStrategyMapping[
    precedenceIndexToExpressionKey(toPrecedenceIndex(precedence))
  ]!
)

function kindToExpressionStrategy<
  Data extends object,
  ExpressionT extends Expression,
  HasPrecedence extends boolean
> (
  kind: ExpressionKind<Data, ExpressionT, HasPrecedence>
): fc.Arbitrary<ExpressionT> {
  return kind.factory(
    kind.factoryArgumentMinPrecedenceIndices.map(
      (precedenceIndex) => (
        fc.oneof(
          { depthIdentifier: EXPRESSION_DEPTH_IDENTIFIER },
          leafExpressionStrategy,
          ...precedences.slice(precedenceIndex).map(
            (_precedence, index) => expressionStrategyMapping[
              precedenceIndexToExpressionKey(precedenceIndex + index)
            ]!
          ),
          expressionStrategyMapping[EXPRESSION_WITHOUT_PRECEDENCE_KEY]!
        )
      )
    )
  )
}

function kindToExpressionDataStrategy<
  Data extends object,
  ExpressionT extends Expression,
  HasPrecedence extends boolean
> (
  kind: ExpressionKind<Data, ExpressionT, HasPrecedence>
): fc.Arbitrary<Data> {
  return kind.dataFactory(
    kind.factoryArgumentMinPrecedenceIndices.map(
      (precedenceIndex) => (
        fc.oneof(
          { depthIdentifier: EXPRESSION_DEPTH_IDENTIFIER },
          leafExpressionStrategy,
          ...precedences.slice(precedenceIndex).map(
            (_precedence, index) => expressionStrategyMapping[
              precedenceIndexToExpressionKey(precedenceIndex + index)
            ]!
          ),
          expressionStrategyMapping[EXPRESSION_WITHOUT_PRECEDENCE_KEY]!
        )
      )
    )
  )
}

export const annotatedIdentifierDataStrategy = kindToExpressionDataStrategy(
  annotatedIdentifierExpressionKind
)
export const annotatedIdentifierStrategy = kindToExpressionStrategy(
  annotatedIdentifierExpressionKind
)
export const assignmentDataStrategy = kindToExpressionDataStrategy(
  assignmentExpressionKind
)
export const assignmentStrategy = kindToExpressionStrategy(
  assignmentExpressionKind
)
export const bidirectionalConditionalDataStrategy = (
  kindToExpressionDataStrategy(bidirectionalConditionalExpressionKind)
)
export const bidirectionalConditionalStrategy = (
  kindToExpressionStrategy(bidirectionalConditionalExpressionKind)
)
export const binaryArithmeticOperationDataStrategy = fc.oneof(
  ...binaryArithmeticOperationExpressionKinds.map(kindToExpressionDataStrategy)
)
export const binaryArithmeticOperationStrategy = fc.oneof(
  ...binaryArithmeticOperationExpressionKinds.map(kindToExpressionStrategy)
)
export const binaryComparisonDataStrategy = fc.oneof(
  ...binaryComparisonExpressionKinds.map(kindToExpressionDataStrategy)
)
export const binaryComparisonStrategy = fc.oneof(
  ...binaryComparisonExpressionKinds.map(kindToExpressionStrategy)
)
export const blockDataStrategy = kindToExpressionDataStrategy(
  blockExpressionKind
)
export const blockStrategy = kindToExpressionStrategy(blockExpressionKind)
export const callDataStrategy = kindToExpressionDataStrategy(
  callExpressionKind
)
export const callStrategy = kindToExpressionStrategy(callExpressionKind)
export const functionDefinitionDataStrategy = kindToExpressionDataStrategy(
  functionDefinitionExpressionKind
)
export const functionDefinitionStrategy = kindToExpressionStrategy(
  functionDefinitionExpressionKind
)
export const groupingDataStrategy = kindToExpressionDataStrategy(
  groupingExpressionKind
)
export const groupingStrategy = kindToExpressionStrategy(
  groupingExpressionKind
)
export const memberAccessDataStrategy = kindToExpressionDataStrategy(
  memberAccessExpressionKind
)
export const memberAccessStrategy = kindToExpressionStrategy(
  memberAccessExpressionKind
)
export const returnDataStrategy = kindToExpressionDataStrategy(
  returnExpressionKind
)
export const returnStrategy = kindToExpressionStrategy(returnExpressionKind)
export const tupleDataStrategy = kindToExpressionDataStrategy(
  tupleExpressionKind
)
export const tupleStrategy = kindToExpressionStrategy(tupleExpressionKind)
export const unaryArithmeticOperationDataStrategy = fc.oneof(
  ...unaryArithmeticOperationExpressionKinds.map(kindToExpressionDataStrategy)
)
export const unaryArithmeticOperationStrategy = fc.oneof(
  ...unaryArithmeticOperationExpressionKinds.map(kindToExpressionStrategy)
)
export const unidirectionalConditionalDataStrategy = (
  kindToExpressionDataStrategy(unidirectionalConditionalExpressionKind)
)
export const unidirectionalConditionalStrategy = (
  kindToExpressionStrategy(unidirectionalConditionalExpressionKind)
)
export const whileLoopDataStrategy = kindToExpressionDataStrategy(
  whileLoopExpressionKind
)
export const whileLoopStrategy = kindToExpressionStrategy(
  whileLoopExpressionKind
)
