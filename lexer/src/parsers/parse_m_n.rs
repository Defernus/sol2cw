use logos::Lexer;

use crate::{LexerResult, Token};

pub fn parse_m_n(_lex: &mut Lexer<Token>, _offset: usize) -> LexerResult<(u32, u32)> {
    todo!("parse_m_n")
}

pub fn parse_ufixed_m_n(lex: &mut Lexer<Token>) -> LexerResult<(u32, u32)> {
    parse_m_n(lex, 6)
}

pub fn parse_fixed_m_n(lex: &mut Lexer<Token>) -> LexerResult<(u32, u32)> {
    parse_m_n(lex, 5)
}
