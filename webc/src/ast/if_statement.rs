use serde::Serialize;

use crate::{lexer::source::TokensIter, AstResult};

use super::node::{AstNode, AstNodeData, NodeParser};

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstIfStatement {
    pub condition: Box<AstNodeData>,
    pub body: Vec<AstNodeData>,
    pub else_body: Option<Vec<AstNodeData>>,
}

impl NodeParser for AstIfStatement {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstIfStatement> for AstNode {
    fn from(node: AstIfStatement) -> Self {
        Self::IfStatement(node)
    }
}
