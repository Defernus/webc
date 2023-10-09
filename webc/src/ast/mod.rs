use serde::Serialize;

use crate::{
    lexer::{source::TokensIter, tokens::TokenData},
    AstError, AstResult,
};

use self::{node::NodeParser, scope::AstScope};

pub mod assignment;
pub mod binary_operation;
pub mod function_call;
pub mod function_declaration;
pub mod identifier;
pub mod if_statement;
pub mod left_expression;
pub mod literal;
pub mod loop_interrupt;
pub mod loop_statement;
pub mod node;
pub mod return_statement;
pub mod right_expression;
pub mod scope;
pub mod type_declaration;
pub mod type_definition;
pub mod variable_declaration;

#[derive(Debug, Clone, Serialize)]
pub struct WebcAst {
    root_scope: AstScope,
}

impl WebcAst {
    pub fn parse(mut tokens: TokensIter) -> AstResult<Self> {
        let root_scope =
            AstScope::parse_top_level(&mut tokens).ok_or(AstError::UnexpectedToken)??;

        Ok(Self { root_scope })
    }
}
