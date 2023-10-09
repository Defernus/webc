use std::{fs, path::Path, rc::Rc};

use logos::Lexer;
use serde::Serialize;

use crate::{LexerResult, WebcResult};

use self::position::WebcSourcePosition;

use super::{tokens::TokenData, WebcToken};

pub mod position;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct WebcSourceMeta {
    file_path: String,
}

pub struct WebcSource {
    src: String,
    meta: WebcSourceMeta,
}

impl WebcSource {
    pub fn from_file(path: &Path) -> WebcResult<Self> {
        let src = fs::read_to_string(path)?;

        let meta = WebcSourceMeta {
            file_path: path.to_str().unwrap().to_string(),
        };

        Ok(Self { src, meta })
    }

    pub fn meta(&self) -> &WebcSourceMeta {
        &self.meta
    }

    pub fn parse(&self) -> LexerResult<TokensIter> {
        let src = self.src.as_str();
        let lex: Lexer<WebcToken> = Lexer::new(src);

        let mut prev_pos = WebcSourcePosition::default();
        let mut prev_end: usize = 0;

        let tokens = lex
            .spanned()
            .map(move |(token, span)| {
                let token = token?;

                let range = prev_end..span.start;

                let start = prev_pos.advance(src, range);
                let end = start.advance(src, span.start..span.end);

                Ok(TokenData::new(token, start, end))
            })
            .collect::<LexerResult<Vec<_>>>()?;

        Ok(TokensIter::new(Rc::new(tokens)))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TokensIter {
    tokens: Rc<Vec<TokenData>>,
    index: usize,
}

impl TokensIter {
    pub fn new(tokens: Rc<Vec<TokenData>>) -> Self {
        Self { tokens, index: 0 }
    }
}

impl Iterator for TokensIter {
    type Item = TokenData;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.tokens.get(self.index).cloned();
        self.index += 1;
        token
    }
}

impl TokensIter {
    /// Returns the next token in the iterator without consuming it
    pub fn get_next(&self) -> Option<&TokenData> {
        self.tokens.get(self.index)
    }

    /// Returns the previous token in the iterator
    pub fn get_prev(&self) -> Option<&TokenData> {
        if self.index == 0 {
            return None;
        }
        self.tokens.get(self.index - 1)
    }
}
