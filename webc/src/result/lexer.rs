use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Default, Clone, Eq, PartialEq, Serialize)]
pub enum LexerError {
    #[default]
    #[error("Unexpected token")]
    UnexpectedToken,
    #[error("Failed to parse bit number")]
    FailedToParseBitNumber,
    #[error("Bit number too big: {0}")]
    BitNumberTooBig(u32),
    #[error("Bit number is zero")]
    BitNumberIsZero,
    #[error("Bit number is not multiple of 8: {0}")]
    BitNumberIsNotMultipleOf8(u32),
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    #[error("Multiline comment is not closed")]
    OpenMultilineComment,
}

pub type LexerResult<T> = Result<T, LexerError>;
