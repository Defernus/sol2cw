#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum LexerError {
    #[default]
    UnexpectedToken,
    FailedToParseBitNumber,
    BitNumberTooBig(u32),
    BitNumberIsZero,
    BitNumberIsNotMultipleOf8(u32),
    FailedToParseBytesSize,
    BytesSizeIsZero,
    BytesSizeIsTooBig(usize),
    NotImplemented(String),
    IncompleteEscapeSequence,
    IllegalEscapeSequence(String),
    IllegalStringEndQuote,
    UnicodeCharacterInNonUnicodeString(char),
    InvalidHexNumber,
    IllegalHexString,
    OctalNotAllowed,
    InvalidNumberLiteral,
    OpenMultilineComment,
    IllegalToken,
}

pub type LexerResult<T> = Result<T, LexerError>;
