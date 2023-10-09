use serde::Serialize;

use crate::{lexer::source::TokensIter, AstResult};

use super::node::{AstNode, NodeParser};

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub enum AstLiteral {
    String(String),
    Int(String),
    Boolean(bool),
    Null,
}

impl NodeParser for AstLiteral {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstLiteral> for AstNode {
    fn from(node: AstLiteral) -> Self {
        Self::Literal(node)
    }
}
