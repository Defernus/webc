use serde::Serialize;

use crate::{
    lexer::{source::TokensIter, tokens::Punctuator, WebcToken},
    AstResult,
};

use super::node::{AstNode, AstNodeData, NodeParser};

/// AST node representing a scope. A scope simply contains a list of nodes
#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstScope {
    nodes: Vec<AstNodeData>,
}

impl AstScope {
    pub fn parse_top_level(tokens: &mut TokensIter) -> Option<AstResult<Self>> {
        let mut nodes = Vec::new();

        while let Some(token) = tokens.next() {
            let node = match AstNodeData::parse(tokens) {
                Ok(node) => node,
                Err(err) => return Some(Err(err)),
            };
            nodes.push(node);
        }

        Some(Ok(Self { nodes }))
    }
}

impl NodeParser for AstScope {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        if !matches!(
            tokens.next()?.token(),
            WebcToken::Punctuator(Punctuator::LBrace),
        ) {
            return None;
        }

        let scope = Self::parse_top_level(&mut tokens)?;

        Some((scope, tokens))
    }
}

impl From<AstScope> for AstNode {
    fn from(node: AstScope) -> Self {
        Self::Scope(node)
    }
}
