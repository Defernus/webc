use serde::Serialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize)]
pub enum Keyword {
    /// default
    Default,
    /// in
    In,
    /// inline
    Inline,
    /// let
    Let,
    /// macro
    Macro,
    /// match
    Match,
    /// mut
    Mutable,
    /// sizeof
    Sizeof,
    /// break
    Break,
    /// const
    Constant,
    /// continue
    Continue,
    /// else
    Else,
    /// enum
    Enum,
    /// external
    External,
    /// for
    For,
    /// fn
    Function,
    /// if
    If,
    /// pub
    Public,
    /// return
    Return,
    /// storage
    Storage,
    /// struct
    Struct,
    /// type
    Type,
    /// while
    While,
}
