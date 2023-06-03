use crate::parsers::{multiline_comments::parse_multiline_comment, strings::parse_string_token};
use crate::LexerError;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexerError)]
pub enum Token {
    #[regex(r#""([^"\\]|\\.)*""#, parse_string_token)]
    #[regex(r#"'([^"\\]|\\.)*'"#, parse_string_token)]
    LiteralString(StringLiteral),

    #[token("<")]
    LessThan,

    #[token("<=")]
    LessThanOrEqual,

    #[token("<<=")]
    AssignShl,

    #[token("<<")]
    Shl,

    #[token(">")]
    GreaterThan,

    #[token(">=")]
    GreaterThanOrEqual,

    #[token(">>=")]
    AssignShr,

    #[token(">>")]
    Shr,

    #[token(">>>=")]
    AssignSar,

    #[token(">>>")]
    Sar,

    #[token("=")]
    Assign,

    #[token("==")]
    Equal,

    #[token("=>")]
    DoubleArrow,

    #[token("!")]
    Not,

    #[token("!=")]
    NotEqual,

    #[token("+")]
    Add,

    #[token("++")]
    Inc,

    #[token("+=")]
    AssignAdd,

    #[token("-")]
    Sub,

    #[token("--")]
    Dec,

    #[token("-=")]
    AssignSub,

    #[token("->")]
    RightArrow,

    #[token("*")]
    Mul,

    #[token("*=")]
    AssignMul,

    #[token("**")]
    Exp,

    #[token("%")]
    Mod,

    #[token("%=")]
    AssignMod,

    #[token("/")]
    Div,

    #[token("/=")]
    AssignDiv,

    #[token("&")]
    BitAnd,

    #[token("&&")]
    And,

    #[token("&=")]
    AssignBitAnd,

    #[token("|")]
    BitOr,

    #[token("||")]
    Or,

    #[token("|=")]
    AssignBitOr,

    #[token("^")]
    BitXor,

    #[token("^=")]
    AssignBitXor,

    #[regex(r#"[0-9]+"#)]
    #[regex(r#"0[xX][0-9a-fA-F]+"#)]
    #[regex(r#".[0-9]+"#)]
    #[regex(r#"[0-9]+.[0-9]+"#)]
    Number,

    #[token(".")]
    Period,

    #[token(":")]
    Colon,

    #[token(":=")]
    AssemblyAssign,

    #[token(";")]
    Semicolon,

    #[token(",")]
    Comma,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBrack,

    #[token("]")]
    RBrack,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("?")]
    Conditional,

    #[token("~")]
    BitNot,

    #[regex(r#"[\$a-zA-Z_][\$a-zA-Z0-9_]*"#)]
    Identifier,

    #[regex(r#"//[^\n]*"#)]
    #[token("/*", parse_multiline_comment)]
    #[regex(r#"[ \t\r\n\f]+"#)]
    Whitespace,

    #[end]
    EOS,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub value: String,
    pub is_unicode: bool,
}
