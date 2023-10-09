use serde::Serialize;

use crate::{lexer::source::TokensIter, AstResult};

use super::node::{AstNode, AstNodeData, NodeParser};

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstLoopStatement {
    init: Box<AstNodeData>,
    condition: Box<AstNodeData>,
    increment: Box<AstNodeData>,
    body: Box<AstNodeData>,
}

impl NodeParser for AstLoopStatement {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstLoopStatement> for AstNode {
    fn from(node: AstLoopStatement) -> Self {
        Self::LoopStatement(node)
    }
}
