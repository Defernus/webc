use logos::Lexer;
use serde::Serialize;

use crate::{LexerError, LexerResult};

use super::WebcToken;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize)]
pub enum TypeKeyword {
    /// uintN
    UIntM(u32),
    /// intN
    IntM(u32),
    /// bool
    Bool,
}

/// Parses bits for types like u16, i256, etc.
pub fn parse_bit_number(lex: &mut Lexer<WebcToken>, offset: usize) -> LexerResult<u32> {
    let slice = lex.slice();
    let slice = &slice[offset..];

    let num: u32 = slice
        .parse()
        .map_err(|_| LexerError::FailedToParseBitNumber)?;

    if num > 256 {
        return Err(LexerError::BitNumberTooBig(num));
    }

    if num == 0 {
        return Err(LexerError::BitNumberIsZero);
    }

    if num % 8 != 0 {
        return Err(LexerError::BitNumberIsNotMultipleOf8(num));
    }

    Ok(num)
}

pub(super) fn parse_int_bits(lex: &mut Lexer<WebcToken>) -> LexerResult<TypeKeyword> {
    Ok(TypeKeyword::IntM(parse_bit_number(lex, 3)?))
}

pub(super) fn parse_uint_bits(lex: &mut Lexer<WebcToken>) -> LexerResult<TypeKeyword> {
    Ok(TypeKeyword::UIntM(parse_bit_number(lex, 4)?))
}
