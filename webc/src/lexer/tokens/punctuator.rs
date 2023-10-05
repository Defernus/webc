use serde::Serialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize)]
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
