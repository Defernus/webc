#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    /// null
    Null,
    /// true
    True,
    /// false
    False,
    Number,
}
