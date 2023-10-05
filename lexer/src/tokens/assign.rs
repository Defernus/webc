#[derive(Debug, PartialEq, Clone)]
pub enum Assign {
    /// =
    Assign,
    /// |=
    AssignBitOr,
    /// ^=
    AssignBitXor,
    /// &=
    AssignBitAnd,
    /// <<=
    AssignShl,
    /// >>=
    AssignSar,
    /// >>>=
    AssignShr,
    /// +=
    AssignAdd,
    /// -=
    AssignSub,
    /// *=
    AssignMul,
    /// /=
    AssignDiv,
    /// %=
    AssignMod,
}
