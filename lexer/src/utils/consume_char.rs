use logos::Lexer;

use crate::Token;

/// Consumes a single character from the lexer and extends the span (token size) by it.
///
/// **NOTE:** If the character is a UTF-8 multi-byte character, it will consume the entire character.
pub fn consume_char(lex: &mut Lexer<Token>) -> Option<char> {
    let mut bump = 1;
    let remainder = lex.remainder();

    let char = next_char(lex)?;

    while !remainder.is_char_boundary(bump) && bump < remainder.len() {
        bump += 1;
    }
    lex.bump(bump);

    Some(char)
}

/// Returns the character after the current token (or None if there is no characters left).
pub fn next_char(lex: &Lexer<Token>) -> Option<char> {
    lex.remainder().chars().next()
}
