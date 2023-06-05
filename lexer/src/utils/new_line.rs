use crate::{LexerError, LexerResult};

pub fn is_new_line_unicode(ch: char, is_unicode: bool) -> LexerResult<bool> {
    if 0x0a as char <= ch && ch <= 0x0d as char {
        return Ok(true);
    }

    if ch == '\u{0085}' || ch == '\u{2028}' || ch == '\u{2029}' {
        if !is_unicode {
            return Err(LexerError::UnicodeCharacterInNonUnicodeString(ch));
        }
        return Ok(true);
    }
    return Ok(false);
}

pub fn is_new_line(ch: char) -> bool {
    if 0x0a as char <= ch && ch <= 0x0d as char {
        return true;
    }

    if ch == '\u{0085}' || ch == '\u{2028}' || ch == '\u{2029}' {
        return true;
    }
    return false;
}
