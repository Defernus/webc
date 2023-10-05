use serde::Serialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize)]
pub enum UnaryOperator {
    /// !
    Not,
    /// ~
    BitNot,
    /// ++
    Inc,
    /// --
    Dec,
}
