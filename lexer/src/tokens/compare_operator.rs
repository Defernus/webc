#[derive(Debug, PartialEq, Clone)]
pub enum CompareOperator {
    /// ==
    Equal,
    /// !=
    NotEqual,
    /// <
    LessThan,
    /// >
    GreaterThan,
    /// <=
    LessThanOrEqual,
    /// >=
    GreaterThanOrEqual,
}
