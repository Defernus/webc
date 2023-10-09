use serde::Serialize;

use crate::{lexer::source::TokensIter, AstResult};

use super::node::{AstNode, NodeParser};

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub enum AstLoopInterrupt {
    Break,
    Continue,
}

impl NodeParser for AstLoopInterrupt {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstLoopInterrupt> for AstNode {
    fn from(node: AstLoopInterrupt) -> Self {
        Self::LoopInterrupt(node)
    }
}
