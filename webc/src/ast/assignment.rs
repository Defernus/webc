use serde::Serialize;

use crate::{
    lexer::{source::TokensIter, tokens::AssignOperator},
    AstResult,
};

use super::node::{AstNode, AstNodeData, NodeParser};

/// AST node representing an assignment
#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstAssignment {
    pub operator: AssignOperator,
    pub assignee: Box<AstNodeData>,
    pub value: Box<AstNodeData>,
}

impl NodeParser for AstAssignment {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstAssignment> for AstNode {
    fn from(node: AstAssignment) -> Self {
        Self::Assignment(node)
    }
}
