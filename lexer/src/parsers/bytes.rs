use logos::Lexer;

use crate::{token::TypeKeyword, LexerError, LexerResult, Token};

pub fn parse_bytes(lex: &mut Lexer<Token>) -> LexerResult<TypeKeyword> {
    let slice = lex.slice();
    let slice = &slice[5..];

    let len: u32 = slice
        .parse()
        .map_err(|err| LexerError(format!("Failed to parse bytes len \"{}\": {}", slice, err)))?;

    if len > 32 {
        return Err(LexerError(format!(
            "Bytes len \"{}\" is too large, must be <= 32",
            len
        )));
    }

    if len == 0 {
        return Err(LexerError(format!(
            "Bytes len \"{}\" is too small, must be >= 1",
            len
        )));
    }

    Ok(TypeKeyword::BytesM(len))
}
