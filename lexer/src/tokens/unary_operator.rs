#[derive(Debug, PartialEq, Clone)]
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
