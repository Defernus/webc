use serde::Serialize;

use crate::{lexer::source::TokensIter, AstResult};

use super::node::{AstNode, NodeParser};

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub enum AstTypeDefinition {
    Struct(),
    Enum(),
    Reference(),
}

impl NodeParser for AstTypeDefinition {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstTypeDefinition> for AstNode {
    fn from(node: AstTypeDefinition) -> Self {
        Self::TypeDefinition(node)
    }
}
