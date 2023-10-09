use serde::Serialize;

use crate::{lexer::source::TokensIter, AstResult};

use super::node::{AstNode, AstNodeData, NodeParser};

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub enum LeftExpression {
    Increment,
    Decrement,
    Reference,
    Pointer,
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstLeftExpression {
    pub operator: LeftExpression,
    pub operand: Box<AstNodeData>,
}

impl NodeParser for AstLeftExpression {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstLeftExpression> for AstNode {
    fn from(node: AstLeftExpression) -> Self {
        Self::LeftExpression(node)
    }
}
