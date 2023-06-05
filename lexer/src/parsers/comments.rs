use logos::Lexer;

use crate::{LexerError, LexerResult, Token};

pub fn parse_multiline_comment(lex: &mut Lexer<Token>) -> LexerResult<()> {
    while let Some(ch) = lex.remainder().chars().next() {
        if ch == '*' {
            lex.bump(1);
            if let Some('/') = lex.remainder().chars().next() {
                lex.bump(1);
                return Ok(());
            }
        } else {
            lex.bump(1);
        }
    }

    Err(LexerError::OpenMultilineComment)
}
