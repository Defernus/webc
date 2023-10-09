use serde::Serialize;

use crate::{lexer::source::TokensIter, AstResult};

use super::node::{AstNode, AstNodeData, NodeParser};

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstVariableDeclaration {
    pub name: String,
    pub value: Box<AstNodeData>,
    pub type_data: Box<AstNodeData>,
}

impl NodeParser for AstVariableDeclaration {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstVariableDeclaration> for AstNode {
    fn from(node: AstVariableDeclaration) -> Self {
        Self::VariableDeclaration(node)
    }
}
