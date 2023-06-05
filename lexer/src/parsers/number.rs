use logos::Lexer;

use crate::{token::Literal, LexerError, LexerResult, Token};

pub fn validate_number(lex: &mut Lexer<Token>) -> LexerResult<Literal> {
    let slice = lex.slice();

    if validate_hex_number(slice)? {
        return Ok(Literal::Number);
    }

    if validate_octal_number(slice)? {
        return Err(LexerError::OctalNotAllowed);
    }

    validate_scientific_notation(slice)?;

    Ok(Literal::Number)
}

pub fn validate_hex_number(slice: &str) -> LexerResult<bool> {
    if slice.len() < 3 {
        return Ok(false);
    }

    if slice.starts_with("0x") {
        let mut is_valid = false;

        for ch in slice[2..].chars() {
            if ch == '_' {
                continue;
            }
            if !ch.is_ascii_hexdigit() {
                return Err(LexerError::InvalidHexNumber);
            }

            is_valid = true;
        }

        if !is_valid {
            return Err(LexerError::InvalidHexNumber);
        }

        return Ok(true);
    }

    Ok(false)
}

pub fn validate_octal_number(slice: &str) -> LexerResult<bool> {
    if slice.len() < 2 {
        return Ok(false);
    }

    if slice.starts_with("0") {
        for ch in slice[1..].chars() {
            if !ch.is_digit(8) {
                return Ok(false);
            }
        }
        return Ok(true);
    }

    Ok(false)
}

pub fn validate_scientific_notation(slice: &str) -> LexerResult<()> {
    let mut chars = slice.chars();

    // consume the string until we find 'e' or end of string
    while let Some(ch) = chars.next() {
        if ch == 'e' {
            chars.next();
            break;
        }

        if !ch.is_ascii_digit() && ch != '_' {
            return Err(LexerError::InvalidNumberLiteral);
        }
    }

    // consume rest of the string
    while let Some(ch) = chars.next() {
        if !ch.is_ascii_digit() {
            return Err(LexerError::InvalidNumberLiteral);
        }
    }

    Ok(())
}
