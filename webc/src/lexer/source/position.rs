use std::ops::Range;

use serde::Serialize;

#[derive(Debug, Default, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Serialize)]
pub struct WebcSourcePosition {
    line: usize,
    column: usize,
}

impl WebcSourcePosition {
    /// Creates a new source position
    ///
    /// **WARNING**: The line and column numbers are 0-indexed
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    /// Returns the line number, starting from 1
    pub fn display_line(&self) -> usize {
        self.line + 1
    }

    /// Returns the column number, starting from 1
    pub fn display_column(&self) -> usize {
        self.column + 1
    }

    pub fn advance(mut self, src: &str, range: Range<usize>) -> Self {
        let mut iter = src[range].chars();

        while let Some(c) = iter.next() {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }

        self
    }
}

impl std::fmt::Display for WebcSourcePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.display_line(), self.display_column())
    }
}
