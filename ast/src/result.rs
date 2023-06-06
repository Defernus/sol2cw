use sol2cw_lexer::LexerError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AstError {
    LexerError(LexerError),
}

impl From<LexerError> for AstError {
    fn from(error: LexerError) -> Self {
        Self::LexerError(error)
    }
}

pub type AstResult<T> = Result<T, AstError>;
