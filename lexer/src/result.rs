use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum LexerError {
    #[default]
    UnexpectedToken,
    FailedToParseBitNumber,
    BitNumberTooBig(u32),
    BitNumberIsZero,
    BitNumberIsNotMultipleOf8(u32),
    NotImplemented(String),
    OpenMultilineComment,
}

pub type LexerResult<T> = Result<T, LexerError>;
