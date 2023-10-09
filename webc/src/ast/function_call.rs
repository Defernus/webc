use serde::Serialize;

use crate::{lexer::source::TokensIter, AstResult};

use super::node::{AstNode, AstNodeData, NodeParser};

/// AST node representing a function call
#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstFunctionCall {
    /// function name
    pub name: String,
    /// function arguments
    pub arguments: Vec<AstNodeData>,
    /// if method - the object to call the method on
    pub method: Option<Box<AstNodeData>>,
}

impl NodeParser for AstFunctionCall {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstFunctionCall> for AstNode {
    fn from(node: AstFunctionCall) -> Self {
        Self::FunctionCall(node)
    }
}
