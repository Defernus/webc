use serde::Serialize;

use crate::{lexer::source::TokensIter, AstResult};

use super::node::{AstNode, NodeParser};

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstIdentifier {
    pub name: String,
}

impl NodeParser for AstIdentifier {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstIdentifier> for AstNode {
    fn from(node: AstIdentifier) -> Self {
        Self::Identifier(node)
    }
}
