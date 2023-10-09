use serde::Serialize;

use crate::{lexer::source::TokensIter, AstResult};

use super::{
    literal::AstLiteral,
    node::{AstNode, NodeParser},
    type_definition::AstTypeDefinition,
};

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstTypeDeclaration {
    name: String,
    type_data: Box<TypeData>,
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub enum TypeData {
    Definition(AstTypeDefinition),
    Literal(AstLiteral),
}

impl NodeParser for AstTypeDeclaration {
    fn parse(mut tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)> {
        todo!();
    }
}

impl From<AstTypeDeclaration> for AstNode {
    fn from(node: AstTypeDeclaration) -> Self {
        Self::TypeDeclaration(node)
    }
}
