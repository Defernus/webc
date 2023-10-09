use serde::Serialize;

use crate::{lexer::source::TokensIter, AstResult};

use super::node::{AstNode, AstNodeData, NodeParser};

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstReturnStatement {
    pub expression: Box<AstNodeData>,
}

impl NodeParser for AstReturnStatement {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstReturnStatement> for AstNode {
    fn from(node: AstReturnStatement) -> Self {
        Self::ReturnStatement(node)
    }
}
