use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Clone, Eq, PartialEq, Serialize)]
pub enum CompilerError {
    #[error("Not implemented yet for current architecture")]
    Unimplemented,
}

pub type CompilerResult<T> = Result<T, CompilerError>;
