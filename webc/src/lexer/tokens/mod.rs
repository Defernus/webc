use logos::Logos;
use serde::Serialize;

use crate::LexerError;
pub use assign::*;
pub use binary_operator::*;
use comment::{parse_multiline_comment, parse_singleline_comment};
pub use compare_operator::*;
pub use keyword::*;
pub use literal::*;
pub use punctuator::*;
pub use type_keyword::*;
pub use unary_operator::*;

use super::source::{WebcSourceMeta, WebcSourcePosition};

mod assign;
mod binary_operator;
mod comment;
mod compare_operator;
mod keyword;
mod literal;
mod punctuator;
mod type_keyword;
mod unary_operator;
mod utils;

#[derive(Logos, Debug, PartialEq, Clone, Serialize, Eq, PartialOrd, Ord)]
#[logos(error = LexerError)]
#[logos(skip r"[ \t\r\n]+")]
pub enum WebcToken {
    #[token("(", |_| Punctuator::LParen)]
    #[token(")", |_| Punctuator::RParen)]
    #[token("[", |_| Punctuator::LBrack)]
    #[token("]", |_| Punctuator::RBrack)]
    #[token("{", |_| Punctuator::LBrace)]
    #[token("}", |_| Punctuator::RBrace)]
    #[token(":", |_| Punctuator::Colon)]
    #[token(";", |_| Punctuator::Semicolon)]
    #[token(".", |_| Punctuator::Period)]
    #[token(",", |_| Punctuator::Comma)]
    #[token("?", |_| Punctuator::Conditional)]
    #[token("=>", |_| Punctuator::DoubleArrow)]
    #[token("->", |_| Punctuator::RightArrow)]
    Punctuator(Punctuator),

    #[token("=", |_| Assign::Assign)]
    #[token("|=", |_| Assign::AssignBitOr)]
    #[token("^=", |_| Assign::AssignBitXor)]
    #[token("&=", |_| Assign::AssignBitAnd)]
    #[token("<<=", |_| Assign::AssignShl)]
    #[token(">>=", |_| Assign::AssignSar)]
    #[token(">>>=", |_| Assign::AssignShr)]
    #[token("+=", |_| Assign::AssignAdd)]
    #[token("-=", |_| Assign::AssignSub)]
    #[token("*=", |_| Assign::AssignMul)]
    #[token("/=", |_| Assign::AssignDiv)]
    #[token("%=", |_| Assign::AssignMod)]
    Assign(Assign),

    #[token("||", |_| BinaryOperator::Or)]
    #[token("&&", |_| BinaryOperator::And)]
    #[token("|", |_| BinaryOperator::BitOr)]
    #[token("^", |_| BinaryOperator::BitXor)]
    #[token("&", |_| BinaryOperator::BitAnd)]
    #[token("<<", |_| BinaryOperator::Shl)]
    #[token(">>", |_| BinaryOperator::SAR)]
    #[token(">>>", |_| BinaryOperator::Shr)]
    #[token("+", |_| BinaryOperator::Add)]
    #[token("-", |_| BinaryOperator::Sub)]
    #[token("*", |_| BinaryOperator::Mul)]
    #[token("/", |_| BinaryOperator::Div)]
    #[token("%", |_| BinaryOperator::Mod)]
    #[token("**", |_| BinaryOperator::Exp)]
    BinaryOperator(BinaryOperator),

    #[token("==", |_| CompareOperator::Equal)]
    #[token("!=", |_| CompareOperator::NotEqual)]
    #[token("<", |_| CompareOperator::LessThan)]
    #[token(">", |_| CompareOperator::GreaterThan)]
    #[token("<=", |_| CompareOperator::LessThanOrEqual)]
    #[token(">=", |_| CompareOperator::GreaterThanOrEqual)]
    CompareOperator(CompareOperator),

    #[token("!", |_| UnaryOperator::Not)]
    #[token("~", |_| UnaryOperator::BitNot)]
    #[token("++", |_| UnaryOperator::Inc)]
    #[token("--", |_| UnaryOperator::Dec)]
    UnaryOperator(UnaryOperator),

    #[token("default", |_| Keyword::Default)]
    #[token("in", |_| Keyword::In)]
    #[token("inline", |_| Keyword::Inline)]
    #[token("let", |_| Keyword::Let)]
    #[token("macro", |_| Keyword::Macro)]
    #[token("match", |_| Keyword::Match)]
    #[token("mut", |_| Keyword::Mutable)]
    #[token("sizeof", |_| Keyword::Sizeof)]
    #[token("break", |_| Keyword::Break)]
    #[token("const", |_| Keyword::Constant)]
    #[token("continue", |_| Keyword::Continue)]
    #[token("else", |_| Keyword::Else)]
    #[token("enum", |_| Keyword::Enum)]
    #[token("external", |_| Keyword::External)]
    #[token("for", |_| Keyword::For)]
    #[token("fn", |_| Keyword::Function)]
    #[token("if", |_| Keyword::If)]
    #[token("public", |_| Keyword::Public)]
    #[token("return", |_| Keyword::Return)]
    #[token("storage", |_| Keyword::Storage)]
    #[token("struct", |_| Keyword::Struct)]
    #[token("type", |_| Keyword::Type)]
    #[token("while", |_| Keyword::While)]
    Keyword(Keyword),

    #[regex(r#"uint[1-9][0-9]*"#, parse_uint_bits)]
    #[regex(r#"int[1-9][0-9]*"#, parse_int_bits)]
    #[token("bool", |_| TypeKeyword::Bool)]
    Type(TypeKeyword),

    #[token("null", |_| Literal::Null)]
    #[token("true", |_| Literal::True)]
    #[token("false", |_| Literal::False)]
    #[regex(r#"[0-9]*\.[0-9_]+"#, |_| Literal::Number)]
    Literal(Literal),

    UnknownToken,

    #[regex(r#"[\$a-zA-Z_][\$a-zA-Z0-9_]*"#)]
    Identifier,

    #[regex(r#"//"#, parse_singleline_comment)]
    #[token("/*", parse_multiline_comment)]
    Comment,

    /// End of source
    Eos,
}

#[derive(Debug, PartialEq, Eq, Ord, Clone, Serialize, PartialOrd)]
pub struct TokenData {
    token: WebcToken,
    start: WebcSourcePosition,
    end: WebcSourcePosition,
}

impl TokenData {
    pub fn new(token: WebcToken, start: WebcSourcePosition, end: WebcSourcePosition) -> TokenData {
        TokenData { token, start, end }
    }

    /// Returns token
    pub fn token(&self) -> &WebcToken {
        &self.token
    }

    pub fn start(&self) -> WebcSourcePosition {
        self.start
    }

    pub fn end(&self) -> WebcSourcePosition {
        self.end
    }
}
