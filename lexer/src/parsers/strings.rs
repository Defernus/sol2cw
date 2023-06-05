use logos::Lexer;

use crate::{token::Literal, LexerError, LexerResult, Token};

pub fn parse_hex_string_literal(lex: &mut Lexer<Token>) -> LexerResult<Literal> {
    let slice = lex.slice();

    let slice = &slice[4..slice.len() - 1];

    if slice.len() % 2 != 0 {
        return Err(LexerError::IllegalHexString);
    }

    for ch in slice.chars() {
        if !ch.is_ascii_hexdigit() {
            return Err(LexerError::IllegalHexString);
        }
    }

    let value = slice.to_string();

    Ok(Literal::HexString(value))
}

pub fn parse_string_literal(lex: &mut Lexer<Token>) -> LexerResult<Literal> {
    let mut slice = lex.slice();

    let mut is_unicode = false;
    if slice.starts_with("unicode") {
        is_unicode = true;
        slice = &slice[7..];
    }

    let quote = slice.chars().next().unwrap();

    // trim the quotes
    let value = &slice[1..slice.len()];

    let value = validate_end_of_string(value, quote, is_unicode)?;

    let value = validate_escape_sequence(value, is_unicode)?;

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
fn validate_escape_sequence(value: &str, is_unicode: bool) -> LexerResult<String> {
    let mut result = String::new();
    let mut chars = value.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('t') => result.push('\t'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('u') => {
                    let mut hex = String::new();
                    for _ in 0..4 {
                        match chars.next() {
                            Some(c) => hex.push(c),
                            None => {
                                return Err(LexerError::IllegalEscapeSequence(format!("\\u{hex}")))
                            }
                        }
                    }

                    let code_point = u32::from_str_radix(&hex, 16)
                        .map_err(|_| LexerError::IllegalEscapeSequence(format!("\\u{}", hex)))?;

                    result.push(
                        std::char::from_u32(code_point).ok_or_else(|| {
                            LexerError::IllegalEscapeSequence(format!("\\u{}", hex))
                        })?,
                    );
                }
                Some('x') => {
                    let mut hex = String::new();
                    for _ in 0..2 {
                        match chars.next() {
                            Some(c) => hex.push(c),
                            None => {
                                return Err(LexerError::IllegalEscapeSequence(format!("\\x{hex}")))
                            }
                        }
                    }

                    let byte = u8::from_str_radix(&hex, 16)
                        .map_err(|_| LexerError::IllegalEscapeSequence(format!("\\x{}", hex)))?;

                    result.push(byte as char);
                }
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                Some(c) => return Err(LexerError::IllegalEscapeSequence(format!("\\{}", c))),
                None => return Err(LexerError::IncompleteEscapeSequence),
            }
        } else {
            validate_char(ch, is_unicode)?;
            result.push(ch);
        }
    }

    Ok(result)
}

fn validate_end_of_string(value: &str, quote: char, is_unicode: bool) -> LexerResult<&str> {
    let end = value
        .chars()
        .last()
        .ok_or(LexerError::IllegalStringEndQuote)?;

    validate_char(end, is_unicode)?;

    if end != quote {
        return Err(LexerError::IllegalStringEndQuote);
    }

    Ok(&value[0..value.len() - 1])
}

fn validate_char(c: char, is_unicode: bool) -> LexerResult<()> {
    if c == '\n' || c == '\r' || c == '\x0B' || c == '\x0C' {
        return Err(LexerError::IllegalStringEndQuote);
    }

    if !is_unicode && (c <= 0x1f as char || c >= 0x7f as char) {
        return Err(LexerError::UnicodeCharacterInNonUnicodeString(c));
    }

    Ok(())
}
