use serde::Serialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize)]
pub enum Literal {
    /// null
    Null,
    /// true
    True,
    /// false
    False,
    Number,
}
