use crate::parsers::{
    bits::{parse_int_bits, parse_uint_bits},
    bytes::parse_bytes,
    parse_m_n::{parse_fixed_m_n, parse_ufixed_m_n},
    strings::{parse_hex_string_literal, parse_string_literal},
};
use crate::LexerError;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexerError)]
pub enum Token {
    #[token("(", |_| Punctuator::LParen)]
    #[token(")", |_| Punctuator::RParen)]
    #[token("[", |_| Punctuator::LBrack)]
    #[token("]", |_| Punctuator::RBrack)]
    #[token("{", |_| Punctuator::LBrace)]
    #[token("}", |_| Punctuator::RBrace)]
    #[token(":", |_| Punctuator::Colon)]
    #[token(";", |_| Punctuator::Semicolon)]
    #[token(".", |_| Punctuator::Period)]
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

    #[token(",", |_| BinaryOperator::Comma)]
    #[token("||", |_| BinaryOperator::Or)]
    #[token("&&", |_| BinaryOperator::And)]
    #[token("|", |_| BinaryOperator::BitOr)]
    #[token("^", |_| BinaryOperator::BitXor)]
    #[token("&", |_| BinaryOperator::BitAnd)]
    #[token("<<", |_| BinaryOperator::SHL)]
    #[token(">>", |_| BinaryOperator::SAR)]
    #[token(">>>", |_| BinaryOperator::SHR)]
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
    #[token("delete", |_| UnaryOperator::Delete)]
    UnaryOperator(UnaryOperator),

    #[token(":=", |_| InlineAssemblyOperator::Assign)]
    InlineAssemblyOperator(InlineAssemblyOperator),

    #[token("abstract", |_| Keyword::Abstract)]
    #[token("anonymous", |_| Keyword::Anonymous)]
    #[token("as", |_| Keyword::As)]
    #[token("assembly", |_| Keyword::Assembly)]
    #[token("break", |_| Keyword::Break)]
    #[token("catch", |_| Keyword::Catch)]
    #[token("constant", |_| Keyword::Constant)]
    #[token("constructor", |_| Keyword::Constructor)]
    #[token("continue", |_| Keyword::Continue)]
    #[token("contract", |_| Keyword::Contract)]
    #[token("do", |_| Keyword::Do)]
    #[token("else", |_| Keyword::Else)]
    #[token("enum", |_| Keyword::Enum)]
    #[token("emit", |_| Keyword::Emit)]
    #[token("event", |_| Keyword::Event)]
    #[token("external", |_| Keyword::External)]
    #[token("fallback", |_| Keyword::Fallback)]
    #[token("for", |_| Keyword::For)]
    #[token("function", |_| Keyword::Function)]
    #[token("hex", |_| Keyword::Hex)]
    #[token("if", |_| Keyword::If)]
    #[token("indexed", |_| Keyword::Indexed)]
    #[token("interface", |_| Keyword::Interface)]
    #[token("internal", |_| Keyword::Internal)]
    #[token("immutable", |_| Keyword::Immutable)]
    #[token("import", |_| Keyword::Import)]
    #[token("is", |_| Keyword::Is)]
    #[token("library", |_| Keyword::Library)]
    #[token("mapping", |_| Keyword::Mapping)]
    #[token("memory", |_| Keyword::Memory)]
    #[token("modifier", |_| Keyword::Modifier)]
    #[token("new", |_| Keyword::New)]
    #[token("override", |_| Keyword::Override)]
    #[token("payable", |_| Keyword::Payable)]
    #[token("public", |_| Keyword::Public)]
    #[token("pragma", |_| Keyword::Pragma)]
    #[token("private", |_| Keyword::Private)]
    #[token("pure", |_| Keyword::Pure)]
    #[token("receive", |_| Keyword::Receive)]
    #[token("return", |_| Keyword::Return)]
    #[token("returns", |_| Keyword::Returns)]
    #[token("storage", |_| Keyword::Storage)]
    #[token("calldata", |_| Keyword::CallData)]
    #[token("struct", |_| Keyword::Struct)]
    #[token("throw", |_| Keyword::Throw)]
    #[token("try", |_| Keyword::Try)]
    #[token("type", |_| Keyword::Type)]
    #[token("unchecked", |_| Keyword::Unchecked)]
    #[token("unicode", |_| Keyword::Unicode)]
    #[token("using", |_| Keyword::Using)]
    #[token("view", |_| Keyword::View)]
    #[token("virtual", |_| Keyword::Virtual)]
    #[token("while", |_| Keyword::While)]
    Keyword(Keyword),

    #[token("wei", |_| SubDenominations::SubWei)]
    #[token("gwei", |_| SubDenominations::SubGwei)]
    #[token("ether", |_| SubDenominations::SubEther)]
    #[token("seconds", |_| SubDenominations::SubSecond)]
    #[token("minutes", |_| SubDenominations::SubMinute)]
    #[token("hours", |_| SubDenominations::SubHour)]
    #[token("days", |_| SubDenominations::SubDay)]
    #[token("weeks", |_| SubDenominations::SubWeek)]
    #[token("years", |_| SubDenominations::SubYear)]
    SubDenomination(SubDenominations),

    #[token("uint", |_| TypeKeyword::UInt)]
    #[token("int", |_| TypeKeyword::Int)]
    #[token("bytes", |_| TypeKeyword::Bytes)]
    #[regex(r#"bytes[1-9][0-9]*"#, parse_bytes)]
    #[regex(r#"uint[1-9][0-9]*"#, parse_uint_bits)]
    #[regex(r#"int[1-9][0-9]*"#, parse_int_bits)]
    #[token("ufixed", |_| TypeKeyword::UFixed)]
    #[token("fixed", |_| TypeKeyword::Fixed)]
    #[regex(r#"ufixed[1-9][0-9]*x[1-9][0-9]*"#, parse_ufixed_m_n)]
    #[regex(r#"fixed[1-9][0-9]*x[1-9][0-9]*"#, parse_fixed_m_n)]
    #[token("string", |_| TypeKeyword::String)]
    #[token("address", |_| TypeKeyword::Address)]
    #[token("bool", |_| TypeKeyword::Bool)]
    Type(TypeKeyword),

    #[token("true", |_| Literal::True)]
    #[token("false", |_| Literal::False)]
    #[regex(r#"//[^\n]*"#, |_| Literal::Comment)]
    #[regex(r#"/\*[\s\S]*\*/"#, |_| Literal::Comment)]
    #[regex(r#"[0-9]+"#, |_| Literal::Number)]
    #[regex(r#"0[xX][0-9a-fA-F]+"#, |_| Literal::Number)]
    #[regex(r#".[0-9]+"#, |_| Literal::Number)]
    #[regex(r#"[0-9]+.[0-9]+"#, |_| Literal::Number)]
    #[regex(r#""([^"\\]|\\.)*""#, parse_string_literal)]
    #[regex(r#"'([^"\\]|\\.)*'"#, parse_string_literal)]
    #[regex(r#"hex"[0-9a-fA-F]*""#, parse_hex_string_literal)]
    #[regex(r#"hex'[0-9a-fA-F]*'"#, parse_hex_string_literal)]
    Literal(Literal),

    #[token("after", |_| ReservedKeyword::After)]
    #[token("alias", |_| ReservedKeyword::Alias)]
    #[token("apply", |_| ReservedKeyword::Apply)]
    #[token("auto", |_| ReservedKeyword::Auto)]
    #[token("byte", |_| ReservedKeyword::Byte)]
    #[token("case", |_| ReservedKeyword::Case)]
    #[token("copyof", |_| ReservedKeyword::CopyOf)]
    #[token("default", |_| ReservedKeyword::Default)]
    #[token("define", |_| ReservedKeyword::Define)]
    #[token("final", |_| ReservedKeyword::Final)]
    #[token("implements", |_| ReservedKeyword::Implements)]
    #[token("in", |_| ReservedKeyword::In)]
    #[token("inline", |_| ReservedKeyword::Inline)]
    #[token("let", |_| ReservedKeyword::Let)]
    #[token("macro", |_| ReservedKeyword::Macro)]
    #[token("match", |_| ReservedKeyword::Match)]
    #[token("mutable", |_| ReservedKeyword::Mutable)]
    #[token("null", |_| ReservedKeyword::NullLiteral)]
    #[token("of", |_| ReservedKeyword::Of)]
    #[token("partial", |_| ReservedKeyword::Partial)]
    #[token("promise", |_| ReservedKeyword::Promise)]
    #[token("reference", |_| ReservedKeyword::Reference)]
    #[token("relocatable", |_| ReservedKeyword::Relocatable)]
    #[token("sealed", |_| ReservedKeyword::Sealed)]
    #[token("sizeof", |_| ReservedKeyword::Sizeof)]
    #[token("static", |_| ReservedKeyword::Static)]
    #[token("supports", |_| ReservedKeyword::Supports)]
    #[token("switch", |_| ReservedKeyword::Switch)]
    #[token("typedef", |_| ReservedKeyword::Typedef)]
    #[token("typeof", |_| ReservedKeyword::TypeOf)]
    #[token("var", |_| ReservedKeyword::Var)]
    Reserved(ReservedKeyword),

    #[regex(r#"[\$a-zA-Z_][\$a-zA-Z0-9_]*"#)]
    Identifier,

    #[token("leave", |_| YulSpecificToken::Leave)]
    YulSpecificToken(YulSpecificToken),

    #[regex(r#"[ \t\r\n\f]+"#)]
    Whitespace,

    #[end]
    EOS,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    True,
    False,
    Number,
    String(String),
    UnicodeString(String),
    HexString(String),
    Comment,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CompareOperator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Comma,
    Or,
    And,
    BitOr,
    BitXor,
    BitAnd,
    SHL,
    SAR,
    SHR,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Not,
    BitNot,
    Inc,
    Dec,
    Delete,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InlineAssemblyOperator {
    Assign,
}

#[derive(Debug, PartialEq, Clone)]
pub enum YulSpecificToken {
    Leave,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Assign {
    Assign,
    AssignBitOr,
    AssignBitXor,
    AssignBitAnd,
    AssignShl,
    AssignSar,
    AssignShr,
    AssignAdd,
    AssignSub,
    AssignMul,
    AssignDiv,
    AssignMod,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Punctuator {
    RightArrow,
    LParen,
    RParen,
    LBrack,
    RBrack,
    LBrace,
    RBrace,
    Colon,
    Semicolon,
    Period,
    Conditional,
    DoubleArrow,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SubDenominations {
    SubWei,
    SubGwei,
    SubEther,
    SubSecond,
    SubMinute,
    SubHour,
    SubDay,
    SubWeek,
    SubYear,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeKeyword {
    UInt,
    Int,
    Bytes,
    BytesM(u32),
    UIntM(u32),
    IntM(u32),
    UFixed,
    Fixed,
    UFixedMxN((u32, u32)),
    FixedMxN((u32, u32)),
    String,
    Address,
    Bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Abstract,
    Anonymous,
    As,
    Assembly,
    Break,
    Catch,
    Constant,
    Constructor,
    Continue,
    Contract,
    Do,
    Else,
    Enum,
    Emit,
    Event,
    External,
    Fallback,
    For,
    Function,
    Hex,
    If,
    Indexed,
    Interface,
    Internal,
    Immutable,
    Import,
    Is,
    Library,
    Mapping,
    Memory,
    Modifier,
    New,
    Override,
    Payable,
    Public,
    Pragma,
    Private,
    Pure,
    Receive,
    Return,
    Returns,
    Storage,
    CallData,
    Struct,
    Throw,
    Try,
    Type,
    Unchecked,
    Unicode,
    Using,
    View,
    Virtual,
    While,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReservedKeyword {
    After,
    Alias,
    Apply,
    Auto,
    Byte,
    Case,
    CopyOf,
    Default,
    Define,
    Final,
    Implements,
    In,
    Inline,
    Let,
    Macro,
    Match,
    Mutable,
    NullLiteral,
    Of,
    Partial,
    Promise,
    Reference,
    Relocatable,
    Sealed,
    Sizeof,
    Static,
    Supports,
    Switch,
    Typedef,
    TypeOf,
    Var,
}
