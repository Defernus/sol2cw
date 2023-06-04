use logos::Lexer;

use crate::{token::Literal, LexerError, LexerResult, Token};

pub fn parse_hex_string_literal(lex: &mut Lexer<Token>) -> LexerResult<Literal> {
    let slice = lex.slice();

    let value = slice[4..slice.len() - 1].to_string();

    Ok(Literal::HexString(value))
}

pub fn parse_string_literal(lex: &mut Lexer<Token>) -> LexerResult<Literal> {
    let slice = lex.slice();
    // trim the quotes
    let value = slice[1..slice.len() - 1].to_string();

    let (value, is_unicode) = validate_escape_sequence(value)?;

    if is_unicode {
        Ok(Literal::UnicodeString(value))
    } else {
        Ok(Literal::String(value))
    }
}

/// Validates escape sequences in string literals.
///
/// Returns an error if the string contains invalid escape sequences,
/// returns parsed string otherwise.
// TODO rm this after unicode escape sequences are supported
#[allow(unused_assignments)]
fn validate_escape_sequence(value: String) -> LexerResult<(String, bool)> {
    let mut result = String::new();
    let mut chars = value.chars();
    let mut is_unicode = false;

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('t') => result.push('\t'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                // TODO support unicode escape sequences
                Some('u') => {
                    is_unicode = true;
                    return Err(LexerError(format!(
                        "Unicode escape sequences are not supported yet"
                    )));
                }
                // TODO support hex escape sequences
                Some('x') => {
                    return Err(LexerError(format!(
                        "Hex escape sequences are not supported yet"
                    )))
                }
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                Some(c) => return Err(LexerError(format!("Invalid escape sequence: \\{}", c))),
                None => return Err(LexerError("Incomplete escape sequence".to_string())),
            }
        } else {
            result.push(ch);
        }
    }

    Ok((result, is_unicode))
}
