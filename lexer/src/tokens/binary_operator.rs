#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    /// ||
    Or,
    /// &&
    And,
    /// |
    BitOr,
    /// ^
    BitXor,
    /// &
    BitAnd,
    /// <<
    Shl,
    /// >>
    SAR,
    /// >>>
    Shr,
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// %
    Mod,
    /// **
    Exp,
}
