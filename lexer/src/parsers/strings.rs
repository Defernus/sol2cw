use logos::Lexer;

use crate::{token::StringLiteral, LexerResult, Token};

pub fn parse_string_token(lex: &mut Lexer<Token>) -> LexerResult<StringLiteral> {
    let slice = lex.slice();
    // trim the quotes
    let value = slice[1..slice.len() - 1].to_string();

    validate_escape_sequence(&value)?;
    let is_unicode = is_unicode(&value)?;

    Ok(StringLiteral { is_unicode, value })
}

fn validate_escape_sequence(_value: &String) -> LexerResult<()> {
    // TODO add escape sequences validation

    Ok(())
}

fn is_unicode(_value: &String) -> LexerResult<bool> {
    // TODO add escape sequences validation

    Ok(false)
}
