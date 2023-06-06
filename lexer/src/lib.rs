pub mod parsers;
pub mod result;
pub mod token;
pub mod utils;

use std::ops::Range;

use logos::Logos;
pub use result::{LexerError, LexerResult};
pub use token::{
    Assign, BinaryOperator, CompareOperator, InlineAssemblyOperator, Keyword, Literal, Punctuator,
    SubDenomination, Token, TypeKeyword, UnaryOperator,
};

pub struct Lexer<'a> {
    lex: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(sources: &'a str) -> Self {
        Self {
            lex: Token::lexer(sources),
        }
    }

    /// Consume next token
    pub fn next_token(&mut self) -> LexerResult<Token> {
        match self.lex.next() {
            Some(token) => token,
            None => Ok(Token::Eof),
        }
    }

    /// Get a str slice of the current token.
    pub fn slice(&self) -> &str {
        self.lex.slice()
    }

    /// Get the range for the current token in source.
    pub fn span(&self) -> Range<usize> {
        self.lex.span()
    }

    /// Parse the whole input.
    pub fn to_parsed(self) -> Vec<LexerResult<Token>> {
        self.lex.collect::<Vec<_>>()
    }
}
