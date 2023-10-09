use serde::Serialize;

use crate::{lexer::source::TokensIter, AstResult};

use super::node::{AstNode, AstNodeData, NodeParser};

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstFunctionDeclaration {
    pub name: String,
    pub arguments: Vec<String>,
    pub body: Vec<AstNodeData>,
}

impl NodeParser for AstFunctionDeclaration {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstFunctionDeclaration> for AstNode {
    fn from(node: AstFunctionDeclaration) -> Self {
        Self::FunctionDeclaration(node)
    }
}
