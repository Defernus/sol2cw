use logos::Lexer;

use crate::{
    utils::{
        consume_char::{consume_char, next_char},
        new_line::is_new_line,
    },
    LexerError, LexerResult, Token,
};

pub fn parse_multiline_comment(lex: &mut Lexer<Token>) -> LexerResult<()> {
    while let Some(ch) = consume_char(lex) {
        if ch == '*' {
            if let Some('/') = next_char(lex) {
                consume_char(lex);
                return Ok(());
            }
        }
    }

    Err(LexerError::OpenMultilineComment)
}

pub fn parse_singleline_comment(lex: &mut Lexer<Token>) -> LexerResult<()> {
    while let Some(ch) = next_char(lex) {
        if is_new_line(ch) {
            return Ok(());
        }
        consume_char(lex);
    }

    Ok(())
}
