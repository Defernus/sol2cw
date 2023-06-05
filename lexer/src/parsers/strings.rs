use logos::Lexer;

use crate::{
    token::Literal,
    utils::{
        consume_char::{consume_char, next_char},
        new_line::is_new_line_unicode,
    },
    LexerError, LexerResult, Token,
};

pub fn parse_hex_string_literal(lex: &mut Lexer<Token>) -> LexerResult<Literal> {
    let slice = lex.slice();

    let quote = slice.chars().last().ok_or(LexerError::IllegalToken)?;

    if slice.len() % 2 != 0 {
        return Err(LexerError::IllegalHexString);
    }

    let mut result = String::new();
    let mut err: Option<LexerError> = None;

    while let Some(ch) = consume_char(lex) {
        if ch == quote {
            if let Some(err) = err {
                return Err(err);
            }

            if result.len() % 2 != 0 {
                return Err(LexerError::IllegalHexString);
            }

            return Ok(Literal::HexString(result));
        }

        if !ch.is_ascii_hexdigit() {
            err = err.or(Some(LexerError::IllegalHexString));
        }

        result.push(ch);
    }

    if let Some(err) = err {
        return Err(err);
    }

    Err(LexerError::UnexpectedEndOfString)
}

pub fn parse_string_literal(lex: &mut Lexer<Token>) -> LexerResult<Literal> {
    let slice = lex.slice();

    let mut is_unicode = false;
    if slice.starts_with("unicode") {
        is_unicode = true;
    }

    let quote = slice.chars().last().ok_or(LexerError::IllegalToken)?;

    let mut result = String::new();

    let mut err: Option<LexerError> = None;

    while let Some(ch) = next_char(lex) {
        if ch == quote {
            consume_char(lex);
            if let Some(err) = err {
                return Err(err);
            }

            return if is_unicode {
                Ok(Literal::UnicodeString(result))
            } else {
                Ok(Literal::String(result))
            };
        }

        match is_new_line_unicode(ch, is_unicode) {
            Ok(true) => {
                if let Some(err) = err {
                    return Err(err);
                }

                return Err(LexerError::IllegalStringEndQuote);
            }
            Err(e) => {
                if let Some(err) = err {
                    return Err(err);
                }

                return Err(e);
            }
            Ok(false) => {}
        }

        if ch == '\\' {
            consume_char(lex);
            match next_char(lex) {
                Some('t') => result.push('\t'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('u') => {
                    let mut hex = String::new();
                    for _ in 0..4 {
                        consume_char(lex);
                        match next_char(lex) {
                            Some(c) => hex.push(c),
                            None => {
                                err = Some(LexerError::IllegalEscapeSequence(format!("\\u{hex}")));
                                continue;
                            }
                        }
                    }

                    let code_point = u32::from_str_radix(&hex, 16)
                        .map_err(|_| LexerError::IllegalEscapeSequence(format!("\\u{}", hex)));

                    match code_point {
                        Err(e) => {
                            err = err.or(Some(e));
                        }
                        Ok(code_point) => {
                            result.push(std::char::from_u32(code_point).ok_or_else(|| {
                                LexerError::IllegalEscapeSequence(format!("\\u{}", hex))
                            })?);
                        }
                    }
                }
                Some('x') => {
                    let mut hex = String::new();
                    for _ in 0..2 {
                        consume_char(lex);
                        match next_char(lex) {
                            Some(c) => hex.push(c),
                            None => {
                                err = Some(LexerError::IllegalEscapeSequence(format!("\\x{hex}")));
                                continue;
                            }
                        }
                    }

                    let byte = u8::from_str_radix(&hex, 16)
                        .map_err(|_| LexerError::IllegalEscapeSequence(format!("\\x{}", hex)));

                    match byte {
                        Err(e) => {
                            err = err.or(Some(e));
                        }
                        Ok(byte) => {
                            result.push(byte as char);
                        }
                    }
                }
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                Some(c) => {
                    err = Some(LexerError::IllegalEscapeSequence(format!("\\{}", c)));
                    continue;
                }
                None => {
                    err = Some(LexerError::IncompleteEscapeSequence);
                    continue;
                }
            }
        } else {
            err = err.or(validate_char(ch, is_unicode).err());
            result.push(ch);
        }

        consume_char(lex);
    }

    Err(err.unwrap_or(LexerError::UnexpectedEndOfString))
}

fn validate_char(c: char, is_unicode: bool) -> LexerResult<()> {
    if !is_unicode && (c <= 0x1f as char || c >= 0x7f as char) {
        return Err(LexerError::UnicodeCharacterInNonUnicodeString(c));
    }

    Ok(())
}
