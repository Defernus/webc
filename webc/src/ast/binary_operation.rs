use serde::Serialize;

use crate::{
    lexer::{source::TokensIter, tokens::BinaryOperator},
    AstResult,
};

use super::node::{AstNode, AstNodeData, NodeParser};

/// AST node representing a binary operation
///
/// A binary operation is an operation that takes two operands, separated by an operator.
#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstBinaryOperation {
    pub operator: BinaryOperator,
    pub left: Box<AstNodeData>,
    pub right: Box<AstNodeData>,
}

impl NodeParser for AstBinaryOperation {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstBinaryOperation> for AstNode {
    fn from(node: AstBinaryOperation) -> Self {
        Self::BinaryOperation(node)
    }
}
