use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Clone, Eq, PartialEq, Serialize)]
pub enum AstError {
    #[error("Unexpected token")]
    UnexpectedToken,
}

pub type AstResult<T> = Result<T, AstError>;
