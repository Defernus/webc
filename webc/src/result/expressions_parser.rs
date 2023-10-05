use serde::Serialize;
use thiserror::Error;

use crate::lexer::{
    source::{WebcSourceMeta, WebcSourcePosition},
    tokens::TokenData,
};

#[derive(Error, Debug, Clone, Eq, PartialEq, Serialize)]
pub enum ExpressionError {
    #[error("Expression is not implemented yet")]
    Unimplemented,
}

pub type ExpressionResult<T> = Result<T, ExpressionError>;

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct ExpressionErrorData {
    error: ExpressionError,
    start: TokenData,
    end: TokenData,
}

impl ExpressionErrorData {
    pub fn new(error: ExpressionError, start: TokenData, end: TokenData) -> Self {
        ExpressionErrorData { error, start, end }
    }

    pub fn error(&self) -> &ExpressionError {
        &self.error
    }

    /// The token where the error starts
    pub fn start(&self) -> &TokenData {
        &self.start
    }

    /// The token where the error ends
    pub fn end(&self) -> &TokenData {
        &self.end
    }

    /// Index in the source code where the error starts
    pub fn start_position(&self) -> WebcSourcePosition {
        self.start.start()
    }

    /// Index in the source code where the error ends
    pub fn end_position(&self) -> WebcSourcePosition {
        self.end.end()
    }
}

impl std::fmt::Display for ExpressionErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} at {}", self.error, self.start.start(),)
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize)]
pub struct ExpressionErrors {
    errors: Vec<ExpressionError>,
}

impl ExpressionErrors {
    /// Add an error to the list of errors
    pub fn push(&mut self, error: ExpressionError) {
        self.errors.push(error);
    }
}

impl std::fmt::Display for ExpressionErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut errors = String::new();
        for error in &self.errors {
            errors.push_str(&format!("{}\n", error));
        }
        write!(f, "{}", errors)
    }
}

impl std::error::Error for ExpressionErrors {}
