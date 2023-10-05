use logos::Lexer;

use crate::{LexerError, LexerResult};

use super::{
    utils::{consume_char, is_new_line, next_char},
    WebcToken,
};

pub fn parse_multiline_comment(lex: &mut Lexer<WebcToken>) -> LexerResult<()> {
    while let Some(ch) = consume_char(lex) {
        if ch == '*' {
            if let Some('/') = next_char(lex) {
                consume_char(lex);
                return Ok(());
            }
        }
    }

    Err(LexerError::OpenMultilineComment)
}

pub fn parse_singleline_comment(lex: &mut Lexer<WebcToken>) -> LexerResult<()> {
    while let Some(ch) = next_char(lex) {
        if is_new_line(ch) {
            return Ok(());
        }
        consume_char(lex);
    }

    Ok(())
}
