use crate::parsers::{
    bits::{parse_int_bits, parse_uint_bits},
    bytes::parse_bytes,
    comments::{parse_multiline_comment, parse_singleline_comment},
    number::validate_number,
    parse_m_n::{parse_fixed_m_n, parse_ufixed_m_n},
    strings::{parse_hex_string_literal, parse_string_literal},
};
use crate::LexerError;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexerError)]
#[logos(skip r"[ \t\r\n]+")]
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
    #[token("delete", |_| UnaryOperator::Delete)]
    UnaryOperator(UnaryOperator),

    #[token(":=", |_| InlineAssemblyOperator::Assign)]
    InlineAssemblyOperator(InlineAssemblyOperator),

    #[token("after", |_| Keyword::After)]
    #[token("alias", |_| Keyword::Alias)]
    #[token("apply", |_| Keyword::Apply)]
    #[token("auto", |_| Keyword::Auto)]
    #[token("byte", |_| Keyword::Byte)]
    #[token("case", |_| Keyword::Case)]
    #[token("copyof", |_| Keyword::CopyOf)]
    #[token("default", |_| Keyword::Default)]
    #[token("define", |_| Keyword::Define)]
    #[token("final", |_| Keyword::Final)]
    #[token("implements", |_| Keyword::Implements)]
    #[token("in", |_| Keyword::In)]
    #[token("inline", |_| Keyword::Inline)]
    #[token("let", |_| Keyword::Let)]
    #[token("macro", |_| Keyword::Macro)]
    #[token("match", |_| Keyword::Match)]
    #[token("mutable", |_| Keyword::Mutable)]
    #[token("null", |_| Keyword::NullLiteral)]
    #[token("of", |_| Keyword::Of)]
    #[token("partial", |_| Keyword::Partial)]
    #[token("promise", |_| Keyword::Promise)]
    #[token("reference", |_| Keyword::Reference)]
    #[token("relocatable", |_| Keyword::Relocatable)]
    #[token("sealed", |_| Keyword::Sealed)]
    #[token("sizeof", |_| Keyword::Sizeof)]
    #[token("static", |_| Keyword::Static)]
    #[token("supports", |_| Keyword::Supports)]
    #[token("switch", |_| Keyword::Switch)]
    #[token("typedef", |_| Keyword::Typedef)]
    #[token("typeof", |_| Keyword::TypeOf)]
    #[token("var", |_| Keyword::Var)]
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
    #[token("using", |_| Keyword::Using)]
    #[token("view", |_| Keyword::View)]
    #[token("virtual", |_| Keyword::Virtual)]
    #[token("while", |_| Keyword::While)]
    Keyword(Keyword),

    #[token("wei", |_| SubDenomination::Wei)]
    #[token("gwei", |_| SubDenomination::Gwei)]
    #[token("ether", |_| SubDenomination::Ether)]
    #[token("seconds", |_| SubDenomination::Second)]
    #[token("minutes", |_| SubDenomination::Minute)]
    #[token("hours", |_| SubDenomination::Hour)]
    #[token("days", |_| SubDenomination::Day)]
    #[token("weeks", |_| SubDenomination::Week)]
    #[token("years", |_| SubDenomination::Year)]
    SubDenomination(SubDenomination),

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
    #[regex(r#"[0-9]+e-?([1-9][0-9]*)?"#, |_| Literal::Number)] // scientific notation
    #[regex(r#"[0-9]*\.[0-9]+e-?([1-9][0-9]*)?"#, |_| Literal::Number)] // scientific notation
    #[regex(r#"[0-9]*\.[0-9_]+"#, |_| Literal::Number)]
    #[regex(r#"[0-9][0-9_]*[a-zA-Z]*"#, validate_number)] // decimal, octal or hex int
    #[regex(r#"0x[0-9_]+"#, validate_number)] // hex
    #[regex(r#"(unicode)?""#, parse_string_literal)]
    #[regex(r#"(unicode)?'"#, parse_string_literal)]
    #[regex(r#"hex""#, parse_hex_string_literal)]
    #[regex(r#"hex'"#, parse_hex_string_literal)]
    Literal(Literal),

    #[token("unicode", |_| Err(LexerError::IllegalToken))]
    #[token("hex", |_| Err(LexerError::IllegalToken))]
    UnknownToken,

    #[regex(r#"[\$a-zA-Z_][\$a-zA-Z0-9_]*"#)]
    Identifier,

    #[regex(r#"//"#, parse_singleline_comment)]
    #[token("/*", parse_multiline_comment)]
    Comment,

    /// End of file
    Eof,
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

impl From<Literal> for Token {
    fn from(literal: Literal) -> Self {
        Token::Literal(literal)
    }
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

impl From<CompareOperator> for Token {
    fn from(operator: CompareOperator) -> Self {
        Token::CompareOperator(operator)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Comma,
    Or,
    And,
    BitOr,
    BitXor,
    BitAnd,
    Shl,
    SAR,
    Shr,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
}

impl From<BinaryOperator> for Token {
    fn from(operator: BinaryOperator) -> Self {
        Token::BinaryOperator(operator)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Not,
    BitNot,
    Inc,
    Dec,
    Delete,
}

impl From<UnaryOperator> for Token {
    fn from(operator: UnaryOperator) -> Self {
        Token::UnaryOperator(operator)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum InlineAssemblyOperator {
    Assign,
}

impl From<InlineAssemblyOperator> for Token {
    fn from(operator: InlineAssemblyOperator) -> Self {
        Token::InlineAssemblyOperator(operator)
    }
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

impl From<Assign> for Token {
    fn from(assign: Assign) -> Self {
        Token::Assign(assign)
    }
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

impl From<Punctuator> for Token {
    fn from(punctuator: Punctuator) -> Self {
        Token::Punctuator(punctuator)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SubDenomination {
    Wei,
    Gwei,
    Ether,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Year,
}

impl From<SubDenomination> for Token {
    fn from(sub_denomination: SubDenomination) -> Self {
        Token::SubDenomination(sub_denomination)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeKeyword {
    UInt,
    Int,
    Bytes,
    BytesM(usize),
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

impl From<TypeKeyword> for Token {
    fn from(type_keyword: TypeKeyword) -> Self {
        Token::Type(type_keyword)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
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
    Using,
    View,
    Virtual,
    While,
}

impl From<Keyword> for Token {
    fn from(keyword: Keyword) -> Self {
        Token::Keyword(keyword)
    }
}
