use logos::Lexer;

use crate::WebcToken;

/// Consumes a single character from the lexer and extends the span (token size) by it.
///
/// **NOTE:** If the character is a UTF-8 multi-byte character, it will consume the entire character.
pub fn consume_char(lex: &mut Lexer<WebcToken>) -> Option<char> {
    let mut bump = 1;
    let remainder = lex.remainder();

    let char = next_char(lex)?;

    while !remainder.is_char_boundary(bump) && bump < remainder.len() {
        bump += 1;
    }
    lex.bump(bump);

    Some(char)
}

/// Returns the character after the current token (or None if there is no characters left).
pub fn next_char(lex: &Lexer<WebcToken>) -> Option<char> {
    lex.remainder().chars().next()
}

pub fn is_new_line(ch: char) -> bool {
    if 0x0a as char <= ch && ch <= 0x0d as char {
        return true;
    }

    if ch == '\u{0085}' || ch == '\u{2028}' || ch == '\u{2029}' {
        return true;
    }
    false
}
