use serde::Serialize;

use crate::{lexer::source::TokensIter, AstResult};

use super::node::{AstNode, AstNodeData, NodeParser};

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub enum RightExpression {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstRightExpression {
    pub operator: RightExpression,
    pub operand: Box<AstNodeData>,
}

impl NodeParser for AstRightExpression {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstRightExpression> for AstNode {
    fn from(node: AstRightExpression) -> Self {
        Self::RightExpression(node)
    }
}
