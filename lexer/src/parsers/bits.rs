use logos::Lexer;

use crate::{LexerError, LexerResult, Token};

/// Parses bits for types like u16, i256, etc.
pub fn parse_bits(lex: &mut Lexer<Token>, offset: usize) -> LexerResult<u32> {
    let slice = lex.slice();
    let slice = &slice[offset..];

    let len: u32 = slice
        .parse()
        .map_err(|err| LexerError(format!("Failed to parse bits len \"{}\": {}", slice, err)))?;

    if len > 256 {
        return Err(LexerError(format!(
            "Bits len \"{}\" is too large, must be <= 256",
            len
        )));
    }

    if len == 0 {
        return Err(LexerError(format!(
            "Bits len \"{}\" is too small, must be >= 1",
            len
        )));
    }

    if len % 8 != 0 {
        return Err(LexerError(format!(
            "Bits len \"{}\" is not a multiple of 8",
            len
        )));
    }

    Ok(len)
}

pub fn parse_int_bits(lex: &mut Lexer<Token>) -> LexerResult<u32> {
    parse_bits(lex, 3)
}

pub fn parse_uint_bits(lex: &mut Lexer<Token>) -> LexerResult<u32> {
    parse_bits(lex, 4)
}
