use serde::Serialize;
use thiserror::Error;

pub use ast::*;
pub use compiler::*;
pub use expressions_parser::*;
pub use lexer::*;

mod ast;
mod compiler;
mod expressions_parser;
mod lexer;

#[derive(Error, Debug, Clone, Serialize, PartialEq, Eq)]
pub enum WebcError {
    #[error("Failed to parse config: {0}")]
    ConfigParsing(String),
    #[error("Failed to parse tokens: {0}")]
    LexerError(#[from] LexerError),
    #[error("Failed to parse AST: {0}")]
    AstError(#[from] AstError),
    #[error("Failed to parse expressions:\n{0}")]
    ExpressionErrors(#[from] ExpressionErrors),
    #[error("Failed to compile: {0}")]
    CompilerError(#[from] CompilerError),
}

pub type WebcResult<T> = Result<T, WebcError>;
