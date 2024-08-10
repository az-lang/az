use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::associativity::Associativity;
use crate::parsing::filler::Filler;
use crate::parsing::operators::{
    AnnotationOperator, AssignmentOperator, CallOperator,
    FunctionTypeOperator, MemberAccessOperator, ReturnOperator,
};
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::parsing::precedence::Precedence;
use crate::tokenization::{
    ByteSize, CharacterPosition, Token, TokenCollection, Tokenize, Utf8Size,
};

use super::annotated_identifier::{
    AnnotatedIdentifier, AnnotatedIdentifierContentsValidationError,
    AnnotatedIdentifierPositionsValidationError,
};
use super::assignment::{
    Assignment, AssignmentContentsValidationError,
    AssignmentPositionsValidationError,
};
use super::bidirectional_conditional::{
    BidirectionalConditional, BidirectionalConditionalContentsValidationError,
    BidirectionalConditionalPositionsValidationError,
};
use super::binary_arithmetic_operation::{
    BinaryArithmeticOperation,
    BinaryArithmeticOperationContentsValidationError,
    BinaryArithmeticOperationPositionsValidationError,
};
use super::binary_comparison::{
    BinaryComparison, BinaryComparisonContentsValidationError,
    BinaryComparisonPositionsValidationError,
};
use super::block::{
    Block, BlockContentsValidationError, BlockPositionsValidationError,
};
use super::call::{
    Call, CallContentsValidationError, CallPositionsValidationError,
};
use super::function_definition::{
    FunctionDefinition, FunctionDefinitionContentsValidationError,
    FunctionDefinitionPositionsValidationError,
};
use super::function_type::{
    FunctionType, FunctionTypeContentsValidationError,
    FunctionTypePositionsValidationError,
};
use super::grouping::{
    Grouping, GroupingContentsValidationError,
    GroupingPositionsValidationError,
};
use super::identifier::{
    Identifier, IdentifierContentsValidationError,
    IdentifierPositionsValidationError,
};
use super::member_access::{
    MemberAccess, MemberAccessContentsValidationError,
    MemberAccessPositionsValidationError,
};
use super::numeric_literal::{
    NumericLiteral, NumericLiteralContentsValidationError,
    NumericLiteralPositionsValidationError,
};
use super::return_::{
    Return, ReturnContentsValidationError, ReturnPositionsValidationError,
};
use super::tuple::{
    Tuple, TupleContentsValidationError, TuplePositionsValidationError,
};
use super::unary_arithmetic_operation::{
    UnaryArithmeticOperation, UnaryArithmeticOperationContentsValidationError,
    UnaryArithmeticOperationPositionsValidationError,
};
use super::unidirectional_conditional::{
    UnidirectionalConditional,
    UnidirectionalConditionalContentsValidationError,
    UnidirectionalConditionalPositionsValidationError,
};
use super::while_loop::{
    WhileLoop, WhileLoopContentsValidationError,
    WhileLoopPositionsValidationError,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Expression<StringType> {
    AnnotatedIdentifier(AnnotatedIdentifier<StringType>),
    Assignment(Assignment<StringType>),
    BidirectionalConditional(BidirectionalConditional<StringType>),
    BinaryArithmeticOperation(BinaryArithmeticOperation<StringType>),
    BinaryComparison(BinaryComparison<StringType>),
    Block(Block<StringType>),
    Call(Call<StringType>),
    FunctionDefinition(FunctionDefinition<StringType>),
    FunctionType(FunctionType<StringType>),
    Grouping(Grouping<StringType>),
    Identifier(Identifier<StringType>),
    MemberAccess(MemberAccess<StringType>),
    NumericLiteral(NumericLiteral<StringType>),
    Return(Return<StringType>),
    Tuple(Tuple<StringType>),
    UnaryArithmeticOperation(UnaryArithmeticOperation<StringType>),
    UnidirectionalConditional(UnidirectionalConditional<StringType>),
    WhileLoop(WhileLoop<StringType>),
}

impl<StringType> Expression<StringType> {
    pub(crate) fn to_first_start_character_position(
        &self,
    ) -> CharacterPosition {
        match self {
            Expression::AnnotatedIdentifier(value) => {
                value.to_first_start_character_position()
            }
            Expression::Assignment(value) => {
                value.to_first_start_character_position()
            }
            Expression::BidirectionalConditional(value) => {
                value.to_first_start_character_position()
            }
            Expression::BinaryArithmeticOperation(value) => {
                value.to_first_start_character_position()
            }
            Expression::BinaryComparison(value) => {
                value.to_first_start_character_position()
            }
            Expression::Block(value) => {
                value.to_first_start_character_position()
            }
            Expression::Call(value) => {
                value.to_first_start_character_position()
            }
            Expression::FunctionDefinition(value) => {
                value.to_first_start_character_position()
            }
            Expression::FunctionType(value) => {
                value.to_first_start_character_position()
            }
            Expression::Grouping(value) => {
                value.to_first_start_character_position()
            }
            Expression::Identifier(value) => {
                value.to_first_start_character_position()
            }
            Expression::MemberAccess(value) => {
                value.to_first_start_character_position()
            }
            Expression::NumericLiteral(value) => {
                value.to_first_start_character_position()
            }
            Expression::Return(value) => {
                value.to_first_start_character_position()
            }
            Expression::Tuple(value) => {
                value.to_first_start_character_position()
            }
            Expression::UnaryArithmeticOperation(value) => {
                value.to_first_start_character_position()
            }
            Expression::UnidirectionalConditional(value) => {
                value.to_first_start_character_position()
            }
            Expression::WhileLoop(value) => {
                value.to_first_start_character_position()
            }
        }
    }

    pub(super) fn to_binary_operation_associativity(
        &self,
    ) -> Option<Associativity> {
        match self {
            Expression::AnnotatedIdentifier(_) => {
                Some(Associativity::from(AnnotationOperator))
            }
            Expression::Assignment(_) => {
                Some(Associativity::from(AssignmentOperator))
            }
            Expression::BinaryArithmeticOperation(value) => {
                Some(Associativity::from(value.operator))
            }
            Expression::BinaryComparison(value) => {
                Some(Associativity::from(value.operator))
            }
            Expression::Call(_) => Some(Associativity::from(CallOperator)),
            Expression::FunctionType(_) => {
                Some(Associativity::from(FunctionTypeOperator))
            }
            Expression::MemberAccess(_) => {
                Some(Associativity::from(MemberAccessOperator))
            }
            Expression::BidirectionalConditional(_)
            | Expression::Block(_)
            | Expression::FunctionDefinition(_)
            | Expression::Grouping(_)
            | Expression::Identifier(_)
            | Expression::NumericLiteral(_)
            | Expression::Return(_)
            | Expression::Tuple(_)
            | Expression::UnaryArithmeticOperation(_)
            | Expression::UnidirectionalConditional(_)
            | Expression::WhileLoop(_) => None,
        }
    }

    pub(super) fn to_operation_precedence(&self) -> Option<Precedence> {
        match self {
            Expression::AnnotatedIdentifier(_) => {
                Some(Precedence::from(AnnotationOperator))
            }
            Expression::Assignment(_) => {
                Some(Precedence::from(AssignmentOperator))
            }
            Expression::BinaryArithmeticOperation(value) => {
                Some(Precedence::from(value.operator))
            }
            Expression::BinaryComparison(value) => {
                Some(Precedence::from(value.operator))
            }
            Expression::Call(_) => Some(Precedence::from(CallOperator)),
            Expression::FunctionType(_) => {
                Some(Precedence::from(FunctionTypeOperator))
            }
            Expression::MemberAccess(_) => {
                Some(Precedence::from(MemberAccessOperator))
            }
            Expression::Return(_) => Some(Precedence::from(ReturnOperator)),
            Expression::BidirectionalConditional(_)
            | Expression::Block(_)
            | Expression::FunctionDefinition(_)
            | Expression::Grouping(_)
            | Expression::Identifier(_)
            | Expression::NumericLiteral(_)
            | Expression::Tuple(_)
            | Expression::UnidirectionalConditional(_)
            | Expression::WhileLoop(_) => None,
            Expression::UnaryArithmeticOperation(value) => {
                Some(Precedence::from(value.operator))
            }
        }
    }
}

impl<StringType: AsRef<str>> Expression<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(crate) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<ExpressionContentsValidationError>> {
        match self {
            Expression::AnnotatedIdentifier(value) => {
                value.validate_contents_impl().map_err(|errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::AnnotatedIdentifier(
                                    error,
                                ),
                            )
                        })
                        .collect()
                })
            }
            Expression::Assignment(value) => value.validate_contents_impl().map_err(
                |errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::Assignment(
                                    error,
                                ),
                            )
                        })
                        .collect()
                },
            ),
            Expression::BidirectionalConditional(value) => value.validate_contents_impl().map_err(
                |errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::BidirectionalConditional(
                                    error,
                                ),
                            )
                        })
                        .collect()
                },
            ),
            Expression::BinaryArithmeticOperation(value) => {
                value.validate_contents_impl().map_err(
                    |errors| {
                        errors
                            .into_iter()
                            .map(|error| {
                                ExpressionContentsValidationError(
                                    ExpressionContentsValidationErrorKind::BinaryArithmeticOperation(
                                        error,
                                    ),
                                )
                            })
                            .collect()
                    },
                )
            }
            Expression::BinaryComparison(value) => {
                value.validate_contents_impl().map_err(
                    |errors| {
                        errors
                            .into_iter()
                            .map(|error| {
                                ExpressionContentsValidationError(
                                    ExpressionContentsValidationErrorKind::BinaryComparison(
                                        error,
                                    ),
                                )
                            })
                            .collect()
                    },
                )
            }
            Expression::Block(value) => {
                value.validate_contents_impl().map_err(|errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::Block(error),
                            )
                        })
                        .collect()
                })
            }
            Expression::Call(value) => {
                value.validate_contents_impl().map_err(|errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::Call(error),
                            )
                        })
                        .collect()
                })
            }
            Expression::FunctionDefinition(value) => value.validate_contents_impl().map_err(
                |errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::FunctionDefinition(
                                    error,
                                ),
                            )
                        })
                        .collect()
                },
            ),
            Expression::FunctionType(value) => value.validate_contents_impl().map_err(
                |errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::FunctionType(
                                    error,
                                ),
                            )
                        })
                        .collect()
                },
            ),
            Expression::Grouping(value) => value.validate_contents_impl().map_err(
                |errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::Grouping(
                                    error,
                                ),
                            )
                        })
                        .collect()
                },
            ),
            Expression::Identifier(value) => {
                value.validate_contents_impl(value.position.start).map_err(|errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::Identifier(
                                    error,
                                ),
                            )
                        })
                        .collect()
                })
            }
            Expression::MemberAccess(value) => {
                value.validate_contents_impl().map_err(|errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::MemberAccess(
                                    error,
                                ),
                            )
                        })
                        .collect()
                })
            }
            Expression::NumericLiteral(value) => {
                value.validate_contents_impl().map_err(|errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::NumericLiteral(
                                    error,
                                ),
                            )
                        })
                        .collect()
                })
            }
            Expression::Return(value) => {
                value.validate_contents_impl().map_err(|errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::Return(error),
                            )
                        })
                        .collect()
                })
            }
            Expression::Tuple(value) => {
                value.validate_contents_impl().map_err(|errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::Tuple(error),
                            )
                        })
                        .collect()
                })
            }
            Expression::UnaryArithmeticOperation(value) => {
                value.validate_contents_impl().map_err(|errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::UnaryArithmeticOperation(
                                    error,
                                ),
                            )
                        })
                        .collect()
                })
            }
            Expression::UnidirectionalConditional(value) => value.validate_contents_impl().map_err(
                |errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::UnidirectionalConditional(
                                    error,
                                ),
                            )
                        })
                        .collect()
                },
            ),
            Expression::WhileLoop(value) => value.validate_contents_impl().map_err(
                |errors| {
                    errors
                        .into_iter()
                        .map(|error| {
                            ExpressionContentsValidationError(
                                ExpressionContentsValidationErrorKind::WhileLoop(
                                    error,
                                ),
                            )
                        })
                        .collect()
                },
            ),
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> Expression<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    pub(crate) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<ExpressionPositionsValidationError> {
        match self {
            Expression::AnnotatedIdentifier(value) => value
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    ExpressionPositionsValidationError(
                        ExpressionPositionsValidationErrorKind::AnnotatedIdentifier(
                            error,
                        ),
                    )
                }),
            Expression::Assignment(value) => value
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    ExpressionPositionsValidationError(
                        ExpressionPositionsValidationErrorKind::Assignment(
                            error,
                        ),
                    )
                }),
            Expression::BinaryArithmeticOperation(value) => value
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    ExpressionPositionsValidationError(
                        ExpressionPositionsValidationErrorKind::BinaryArithmeticOperation(
                            error,
                        ),
                    )
                }),
            Expression::BinaryComparison(value) => value
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    ExpressionPositionsValidationError(
                        ExpressionPositionsValidationErrorKind::BinaryComparison(
                            error,
                        ),
                    )
                }),
            Expression::Block(value) => value
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    ExpressionPositionsValidationError(
                        ExpressionPositionsValidationErrorKind::Block(error),
                    )
                }),
            Expression::Call(value) => value
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    ExpressionPositionsValidationError(
                        ExpressionPositionsValidationErrorKind::Call(error),
                    )
                }),
            Expression::BidirectionalConditional(value) => value
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    ExpressionPositionsValidationError(
                        ExpressionPositionsValidationErrorKind::BidirectionalConditional(
                            error,
                        ),
                    )
                }),
            Expression::FunctionDefinition(value) => value
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    ExpressionPositionsValidationError(
                        ExpressionPositionsValidationErrorKind::FunctionDefinition(
                            error,
                        ),
                    )
                }),
            Expression::FunctionType(value) => value
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    ExpressionPositionsValidationError(
                        ExpressionPositionsValidationErrorKind::FunctionType(
                            error,
                        ),
                    )
                }),
            Expression::Grouping(value) => value
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    ExpressionPositionsValidationError(
                        ExpressionPositionsValidationErrorKind::Grouping(error),
                    )
                }),
            Expression::Identifier(value) => value
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    ExpressionPositionsValidationError(
                        ExpressionPositionsValidationErrorKind::Identifier(
                            error,
                        ),
                    )
                }),
            Expression::MemberAccess(value) => value
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    ExpressionPositionsValidationError(
                        ExpressionPositionsValidationErrorKind::MemberAccess(
                            error,
                        ),
                    )
                }),
            Expression::NumericLiteral(value) => value
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    ExpressionPositionsValidationError(
                        ExpressionPositionsValidationErrorKind::NumericLiteral(
                            error,
                        ),
                    )
                }),
            Expression::Return(value) => value.validate_positions_impl(
                expected_start_character_position,
            ).map_error(|error| {
                ExpressionPositionsValidationError(
                    ExpressionPositionsValidationErrorKind::Return(error),
                )
            }),
            Expression::Tuple(value) => value.validate_positions_impl(
                expected_start_character_position,
            ).map_error(|error| {
                ExpressionPositionsValidationError(
                    ExpressionPositionsValidationErrorKind::Tuple(error),
                )
            }),
            Expression::UnaryArithmeticOperation(value) => value.validate_positions_impl(
                expected_start_character_position,
            ).map_error(|error| {
                ExpressionPositionsValidationError(
                    ExpressionPositionsValidationErrorKind::UnaryArithmeticOperation(
                        error,
                    ),
                )
            }),
            Expression::UnidirectionalConditional(value) => value.validate_positions_impl(
                expected_start_character_position,
            ).map_error(|error| {
                ExpressionPositionsValidationError(
                    ExpressionPositionsValidationErrorKind::UnidirectionalConditional(
                        error,
                    ),
                )
            }),
            Expression::WhileLoop(value) => value.validate_positions_impl(
                expected_start_character_position,
            ).map_error(|error| {
                ExpressionPositionsValidationError(
                    ExpressionPositionsValidationErrorKind::WhileLoop(error),
                )
            }),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ExpressionContentsValidationError(
    ExpressionContentsValidationErrorKind,
);

#[derive(Debug)]
pub(crate) struct ExpressionPositionsValidationError(
    ExpressionPositionsValidationErrorKind,
);

#[derive(Debug)]
enum ExpressionContentsValidationErrorKind {
    AnnotatedIdentifier(AnnotatedIdentifierContentsValidationError),
    Assignment(AssignmentContentsValidationError),
    BidirectionalConditional(BidirectionalConditionalContentsValidationError),
    BinaryArithmeticOperation(
        BinaryArithmeticOperationContentsValidationError,
    ),
    BinaryComparison(BinaryComparisonContentsValidationError),
    Block(BlockContentsValidationError),
    Call(CallContentsValidationError),
    FunctionDefinition(FunctionDefinitionContentsValidationError),
    FunctionType(FunctionTypeContentsValidationError),
    Grouping(GroupingContentsValidationError),
    Identifier(IdentifierContentsValidationError),
    MemberAccess(MemberAccessContentsValidationError),
    NumericLiteral(NumericLiteralContentsValidationError),
    Return(ReturnContentsValidationError),
    Tuple(TupleContentsValidationError),
    UnaryArithmeticOperation(UnaryArithmeticOperationContentsValidationError),
    UnidirectionalConditional(
        UnidirectionalConditionalContentsValidationError,
    ),
    WhileLoop(WhileLoopContentsValidationError),
}

#[derive(Debug)]
enum ExpressionPositionsValidationErrorKind {
    AnnotatedIdentifier(AnnotatedIdentifierPositionsValidationError),
    Assignment(AssignmentPositionsValidationError),
    BidirectionalConditional(BidirectionalConditionalPositionsValidationError),
    BinaryArithmeticOperation(
        BinaryArithmeticOperationPositionsValidationError,
    ),
    BinaryComparison(BinaryComparisonPositionsValidationError),
    Block(BlockPositionsValidationError),
    Call(CallPositionsValidationError),
    FunctionDefinition(FunctionDefinitionPositionsValidationError),
    FunctionType(FunctionTypePositionsValidationError),
    Grouping(GroupingPositionsValidationError),
    Identifier(IdentifierPositionsValidationError),
    MemberAccess(MemberAccessPositionsValidationError),
    NumericLiteral(NumericLiteralPositionsValidationError),
    Return(ReturnPositionsValidationError),
    Tuple(TuplePositionsValidationError),
    UnaryArithmeticOperation(UnaryArithmeticOperationPositionsValidationError),
    UnidirectionalConditional(
        UnidirectionalConditionalPositionsValidationError,
    ),
    WhileLoop(WhileLoopPositionsValidationError),
}

impl std::fmt::Display for ExpressionContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            ExpressionContentsValidationErrorKind::AnnotatedIdentifier(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionContentsValidationErrorKind::Assignment(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionContentsValidationErrorKind::BidirectionalConditional(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionContentsValidationErrorKind::BinaryArithmeticOperation(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionContentsValidationErrorKind::BinaryComparison(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionContentsValidationErrorKind::Block(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionContentsValidationErrorKind::Call(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionContentsValidationErrorKind::FunctionDefinition(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionContentsValidationErrorKind::FunctionType(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionContentsValidationErrorKind::Grouping(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionContentsValidationErrorKind::Identifier(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionContentsValidationErrorKind::MemberAccess(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionContentsValidationErrorKind::NumericLiteral(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionContentsValidationErrorKind::Return(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionContentsValidationErrorKind::Tuple(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionContentsValidationErrorKind::UnaryArithmeticOperation(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionContentsValidationErrorKind::UnidirectionalConditional(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionContentsValidationErrorKind::WhileLoop(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for ExpressionPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            ExpressionPositionsValidationErrorKind::AnnotatedIdentifier(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionPositionsValidationErrorKind::Assignment(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionPositionsValidationErrorKind::BidirectionalConditional(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionPositionsValidationErrorKind::BinaryArithmeticOperation(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionPositionsValidationErrorKind::BinaryComparison(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionPositionsValidationErrorKind::Block(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionPositionsValidationErrorKind::Call(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionPositionsValidationErrorKind::FunctionDefinition(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionPositionsValidationErrorKind::FunctionType(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionPositionsValidationErrorKind::Grouping(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionPositionsValidationErrorKind::Identifier(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionPositionsValidationErrorKind::MemberAccess(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionPositionsValidationErrorKind::NumericLiteral(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ExpressionPositionsValidationErrorKind::Return(error) => {
                  std::fmt::Display::fmt(error, formatter)
            }
            ExpressionPositionsValidationErrorKind::Tuple(error) => {
                  std::fmt::Display::fmt(error, formatter)
            }
            ExpressionPositionsValidationErrorKind::UnaryArithmeticOperation(
              error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionPositionsValidationErrorKind::UnidirectionalConditional(
              error,
            ) => std::fmt::Display::fmt(error, formatter),
            ExpressionPositionsValidationErrorKind::WhileLoop(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for ExpressionContentsValidationError {}

impl std::error::Error for ExpressionPositionsValidationError {}

impl<
        StringType: AsRef<str> + Into<TokenStringType>,
        TokenStringType: From<&'static str>,
    > Tokenize<TokenStringType> for Expression<StringType>
where
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        match self {
            Expression::AnnotatedIdentifier(value) => value.tokenize(),
            Expression::Assignment(value) => value.tokenize(),
            Expression::BidirectionalConditional(value) => value.tokenize(),
            Expression::BinaryArithmeticOperation(value) => value.tokenize(),
            Expression::Block(value) => value.tokenize(),
            Expression::Call(value) => value.tokenize(),
            Expression::BinaryComparison(value) => value.tokenize(),
            Expression::FunctionDefinition(value) => value.tokenize(),
            Expression::FunctionType(value) => value.tokenize(),
            Expression::Grouping(value) => value.tokenize(),
            Expression::Identifier(value) => value.tokenize(),
            Expression::MemberAccess(value) => value.tokenize(),
            Expression::NumericLiteral(value) => value.tokenize(),
            Expression::Return(value) => value.tokenize(),
            Expression::Tuple(value) => value.tokenize(),
            Expression::UnaryArithmeticOperation(value) => value.tokenize(),
            Expression::UnidirectionalConditional(value) => value.tokenize(),
            Expression::WhileLoop(value) => value.tokenize(),
        }
    }
}

macro_rules! impl_expression_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Expression<$source_string_type>>
            for Expression<$target_string_type>
        {
            fn from(value: Expression<$source_string_type>) -> Self {
                match value {
                    Expression::AnnotatedIdentifier(value) => {
                        Expression::AnnotatedIdentifier(value.into())
                    }
                    Expression::Assignment(value) => {
                        Expression::Assignment(value.into())
                    }
                    Expression::BidirectionalConditional(value) => {
                        Expression::BidirectionalConditional(value.into())
                    }
                    Expression::BinaryArithmeticOperation(value) => {
                        Expression::BinaryArithmeticOperation(value.into())
                    }
                    Expression::Block(value) => {
                        Expression::Block(value.into())
                    }
                    Expression::Call(value) => Expression::Call(value.into()),
                    Expression::BinaryComparison(value) => {
                        Expression::BinaryComparison(value.into())
                    }
                    Expression::FunctionDefinition(value) => {
                        Expression::FunctionDefinition(value.into())
                    }
                    Expression::FunctionType(value) => {
                        Expression::FunctionType(value.into())
                    }
                    Expression::Grouping(value) => {
                        Expression::Grouping(value.into())
                    }
                    Expression::Identifier(value) => {
                        Expression::Identifier(value.into())
                    }
                    Expression::MemberAccess(value) => {
                        Expression::MemberAccess(value.into())
                    }
                    Expression::NumericLiteral(value) => {
                        Expression::NumericLiteral(value.into())
                    }
                    Expression::Return(value) => {
                        Expression::Return(value.into())
                    }
                    Expression::Tuple(value) => {
                        Expression::Tuple(value.into())
                    }
                    Expression::UnaryArithmeticOperation(value) => {
                        Expression::UnaryArithmeticOperation(value.into())
                    }
                    Expression::UnidirectionalConditional(value) => {
                        Expression::UnidirectionalConditional(value.into())
                    }
                    Expression::WhileLoop(value) => {
                        Expression::WhileLoop(value.into())
                    }
                }
            }
        }
    };
}

impl_expression_string_type_conversion!(&str, Arc<str>);
impl_expression_string_type_conversion!(&str, Box<str>);
impl_expression_string_type_conversion!(&str, Rc<str>);
impl_expression_string_type_conversion!(&str, String);
impl_expression_string_type_conversion!(Box<str>, Arc<str>);
impl_expression_string_type_conversion!(Box<str>, Rc<str>);
impl_expression_string_type_conversion!(Box<str>, String);
impl_expression_string_type_conversion!(String, Arc<str>);
impl_expression_string_type_conversion!(String, Box<str>);
impl_expression_string_type_conversion!(String, Rc<str>);
