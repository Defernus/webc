use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Punctuator {
    /// ->
    RightArrow,
    /// (
    LParen,
    /// )
    RParen,
    /// [
    LBrack,
    /// ]
    RBrack,
    /// {
    LBrace,
    /// }
    RBrace,
    /// :
    Colon,
    /// ;
    Semicolon,
    /// .
    Period,
    /// ,
    Comma,
    /// ?
    Conditional,
    /// =>
    DoubleArrow,
}
