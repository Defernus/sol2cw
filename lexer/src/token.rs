use crate::parsers::{
    bits::{parse_int_bits, parse_uint_bits},
    bytes_len::parse_bytes_len,
    multiline_comments::parse_multiline_comment,
    parse_m_n::{parse_fixed_m_n, parse_ufixed_m_n},
    strings::parse_string_token,
};
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

    #[regex(r#"bytes[1-9][0-9]*"#, parse_bytes_len)]
    BytesM(u32),

    #[regex(r#"uint[1-9][0-9]*"#, parse_uint_bits)]
    UintM(u32),

    #[regex(r#"int[1-9][0-9]*"#, parse_int_bits)]
    IntM(u32),

    #[regex(r#"ufixed[1-9][0-9]*x[1-9][0-9]*"#, parse_ufixed_m_n)]
    UFixedMxN((u32, u32)),

    #[regex(r#"fixed[1-9][0-9]*x[1-9][0-9]*"#, parse_fixed_m_n)]
    FixedMxN((u32, u32)),

    #[end]
    EOS,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub value: String,
    pub is_unicode: bool,
}
