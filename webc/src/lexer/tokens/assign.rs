use serde::Serialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize)]
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
