use serde::Serialize;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Serialize)]
pub struct WebcSourcePosition {
    line: usize,
    column: usize,
}

impl std::fmt::Display for WebcSourcePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct WebcSourceMeta {
    file_path: String,
}

pub struct WebcSource {
    src: String,
    meta: WebcSourceMeta,
}

impl WebcSource {
    pub fn meta(&self) -> &WebcSourceMeta {
        &self.meta
    }
}
