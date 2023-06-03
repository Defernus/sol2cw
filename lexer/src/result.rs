#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LexerError(pub String);

impl Default for LexerError {
    fn default() -> Self {
        Self("Unknown error".to_string())
    }
}

pub type LexerResult<T> = Result<T, LexerError>;
