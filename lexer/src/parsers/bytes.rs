use logos::Lexer;

use crate::{token::TypeKeyword, LexerError, LexerResult, Token};

pub fn parse_bytes(lex: &mut Lexer<Token>) -> LexerResult<TypeKeyword> {
    let slice = lex.slice();
    let slice = &slice[5..];

    let len: usize = slice
        .parse()
        .map_err(|_| LexerError::FailedToParseBytesSize)?;

    if len > 32 {
        return Err(LexerError::BytesSizeIsTooBig(len));
    }

    if len == 0 {
        return Err(LexerError::BytesSizeIsZero);
    }

    Ok(TypeKeyword::BytesM(len))
}
