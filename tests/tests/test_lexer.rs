use logos::Logos;
use sol2cw_lexer::{
    token::{
        Assign, BinaryOperator, CompareOperator, InlineAssemblyOperator, Keyword, Literal,
        Punctuator, ReservedKeyword, SubDenomination, TypeKeyword, UnaryOperator,
    },
    utils::new_line::is_new_line,
    LexerError, Token,
};

#[test]
fn test_empty() {
    let mut lex = Token::lexer("");

    assert_eq!(lex.next(), None);
}

#[test]
fn test_smoke_test() {
    let mut lex = Token::lexer("function break;765  \t  \"string1\",'string2'\nidentifier1");

    assert_eq!(lex.next(), Some(Ok(Token::Keyword(Keyword::Function))));
    assert_eq!(lex.next(), Some(Ok(Token::Keyword(Keyword::Break))));
    assert_eq!(lex.next(), Some(Ok(Punctuator::Semicolon.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "765");
    assert_eq!(
        lex.next(),
        Some(Ok(Literal::String("string1".into()).into()))
    );
    assert_eq!(lex.slice(), "\"string1\"");
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Comma.into())));
    assert_eq!(
        lex.next(),
        Some(Ok(Literal::String("string2".into()).into()))
    );
    assert_eq!(lex.slice(), "'string2'");
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.slice(), "identifier1");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_assembly_assign() {
    let mut lex = Token::lexer("let a := 1");

    assert_eq!(lex.next(), Some(Ok(ReservedKeyword::Let.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(InlineAssemblyOperator::Assign.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "1");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_assembly_multiple_assign() {
    let mut lex = Token::lexer("let a, b, c := 1");

    assert_eq!(lex.next(), Some(Ok(ReservedKeyword::Let.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Comma.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Comma.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(InlineAssemblyOperator::Assign.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "1");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_string_printable() {
    for v in 0x20..0x7e {
        let initial_str = format!("{}", v as char);
        // Escape \ and " (since we are quoting with ")
        let escaped_str = if v == b'\\' || v == b'"' {
            format!("\\{}", initial_str.clone())
        } else {
            initial_str.clone()
        };
        let inp = format!("  {{ \"{}\"", escaped_str);
        let mut lex = Token::lexer(inp.as_str());

        assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
        assert_eq!(lex.next(), Some(Ok(Literal::String(initial_str).into())));
        assert_eq!(lex.slice(), &format!("\"{}\"", escaped_str));
        assert_eq!(lex.next(), None);
    }

    let mut lex = Token::lexer("  { '\"'");

    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Ok(Literal::String("\"".to_string()).into()))
    );
    assert_eq!(lex.slice(), "'\"'");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_string_nonprintable() {
    for v in 0..0xff {
        // Skip the valid ones
        if (0x20..=0x7e).contains(&v) {
            continue;
        }
        let v = v as u8 as char;
        let initial_str = format!("{}", v);
        let inp = format!("  {{ \"{}\"", initial_str);
        let mut lex = Token::lexer(inp.as_str());

        assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));

        if v == '\n' || v == '\r' || v == '\x0B' || v == '\x0C' {
            assert_eq!(lex.next(), Some(Err(LexerError::IllegalStringEndQuote)));
            assert_eq!(lex.slice(), "\"");
        } else {
            assert_eq!(
                lex.next(),
                Some(Err(LexerError::UnicodeCharacterInNonUnicodeString(v)))
            );
            if is_new_line(v) {
                assert_eq!(lex.slice(), "\"");
            } else {
                assert_eq!(lex.slice(), &format!("\"{}\"", initial_str));
            }
        }
    }
}

#[test]
fn test_string_escapes() {
    let mut lex = Token::lexer("  { \"a\\x61\"");

    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Ok(Literal::String("aa".to_string()).into()))
    );
    assert_eq!(lex.slice(), "\"a\\x61\"");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_string_escapes_all() {
    let mut lex = Token::lexer("  { \"a\\x61\\n\\r\\t\"");

    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Ok(Literal::String("aa\n\r\t".to_string()).into()))
    );
    assert_eq!(lex.slice(), "\"a\\x61\\n\\r\\t\"");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_string_escapes_legal_before_080() {
    let mut lex = Token::lexer("  { \"a\\b\"");

    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Err(LexerError::IllegalEscapeSequence("\\b".into())))
    );
    assert_eq!(lex.slice(), "\"a\\b\"");
    assert_eq!(lex.next(), None);

    let mut lex = Token::lexer("  { \"a\\f\"");

    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Err(LexerError::IllegalEscapeSequence("\\f".into())))
    );
    assert_eq!(lex.slice(), "\"a\\f\"");
    assert_eq!(lex.next(), None);

    let mut lex = Token::lexer("  { \"a\\v\"");

    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Err(LexerError::IllegalEscapeSequence("\\v".into())))
    );
    assert_eq!(lex.slice(), "\"a\\v\"");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_string_escapes_with_zero() {
    let mut lex = Token::lexer("  { \"a\\x61\\x00abc\"");

    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Ok(Literal::String("aa\u{0}abc".to_string()).into()))
    );
    assert_eq!(lex.slice(), "\"a\\x61\\x00abc\"");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_string_escape_illegal() {
    let mut lex = Token::lexer(" bla \"\\x6rf\" (illegalescape)");

    assert_eq!(lex.next(), Some(Ok(Token::Identifier)),);
    assert_eq!(
        lex.next(),
        Some(Err(LexerError::IllegalEscapeSequence("\\x6r".into())))
    );
    assert_eq!(lex.slice(), "\"\\x6rf\"");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LParen.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Punctuator::RParen.into())));
}

#[test]
fn test_hex_numbers() {
    let mut lex = Token::lexer("var x = 0x765432536763762734623472346;");

    assert_eq!(lex.next(), Some(Ok(ReservedKeyword::Var.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Assign::Assign.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "0x765432536763762734623472346");
    assert_eq!(lex.next(), Some(Ok(Punctuator::Semicolon.into())));
    assert_eq!(lex.next(), None);

    let mut lex = Token::lexer("0x1234");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "0x1234");

    let mut lex = Token::lexer("0X1234");
    assert_eq!(lex.next(), Some(Err(LexerError::InvalidNumberLiteral)));
}

#[test]
fn test_octal_numbers() {
    let mut lex = Token::lexer("07");
    assert_eq!(lex.next(), Some(Err(LexerError::OctalNotAllowed)));

    let mut lex = Token::lexer("007");
    assert_eq!(lex.next(), Some(Err(LexerError::OctalNotAllowed)));

    let mut lex = Token::lexer("-07");
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Sub.into())));
    assert_eq!(lex.next(), Some(Err(LexerError::OctalNotAllowed)));

    let mut lex = Token::lexer("-.07");
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Sub.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));

    let mut lex = Token::lexer("0");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));

    let mut lex = Token::lexer("0.1");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
}

#[test]
fn test_scientific_notation() {
    let mut lex = Token::lexer("var x = 2e10;");

    assert_eq!(lex.next(), Some(Ok(ReservedKeyword::Var.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Assign::Assign.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "2e10");
    assert_eq!(lex.next(), Some(Ok(Punctuator::Semicolon.into())));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_leading_dot_in_identifier() {
    let mut lex = Token::lexer("function .a(");
    assert_eq!(lex.next(), Some(Ok(Keyword::Function.into())));
    assert_eq!(lex.next(), Some(Ok(Punctuator::Period.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Punctuator::LParen.into())));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_middle_dot_in_identifier() {
    let mut lex = Token::lexer("function a..a(");
    assert_eq!(lex.next(), Some(Ok(Keyword::Function.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Punctuator::Period.into())));
    assert_eq!(lex.next(), Some(Ok(Punctuator::Period.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Punctuator::LParen.into())));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_trailing_dot_in_identifier() {
    let mut lex = Token::lexer("function a.(");
    assert_eq!(lex.next(), Some(Ok(Keyword::Function.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Punctuator::Period.into())));
    assert_eq!(lex.next(), Some(Ok(Punctuator::LParen.into())));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_trailing_dot_in_numbers() {
    let mut lex = Token::lexer("2.5");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "2.5");
    assert_eq!(lex.next(), None);
    let mut lex = Token::lexer("2.5e10");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "2.5e10");
    assert_eq!(lex.next(), None);
    let mut lex = Token::lexer(".5");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.next(), None);
    let mut lex = Token::lexer(".5e10");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.next(), None);
    let mut lex = Token::lexer("2.");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.next(), Some(Ok(Punctuator::Period.into())));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_leading_underscore_decimal_is_identifier() {
    let mut lex = Token::lexer("_1.2");
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.slice(), "_1");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), ".2");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_leading_underscore_decimal_after_dot_illegal() {
    let mut lex = Token::lexer("1._2");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "1._2");
    assert_eq!(lex.next(), None);
    let mut lex = Token::lexer("1._");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "1._");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_leading_underscore_exp_are_identifier() {
    let mut lex = Token::lexer("_1e2");
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.slice(), "_1e2");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_leading_underscore_exp_after_e_illegal() {
    let mut lex = Token::lexer("1e_2");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "1e");
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.slice(), "_2");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_leading_underscore_hex_illegal() {
    let mut lex = Token::lexer("0x_abc");
    assert_eq!(lex.next(), Some(Err(LexerError::InvalidHexNumber)));
    assert_eq!(lex.slice(), "0x_");
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_fixed_number_invalid_underscore_front() {
    let mut lex = Token::lexer("12._1234_1234");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "12._1234_1234");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_number_literals_with_trailing_underscore_at_eos() {
    let mut lex = Token::lexer("0x123_");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "0x123_");
    assert_eq!(lex.next(), None);
    let mut lex = Token::lexer("123_");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "123_");
    assert_eq!(lex.next(), None);
    let mut lex = Token::lexer("12.34_");
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "12.34_");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_negative_numbers() {
    let mut lex = Token::lexer("var x = -.2 + -0x78 + -7.3 + 8.9 + 2e-2;");
    assert_eq!(lex.next(), Some(Ok(ReservedKeyword::Var.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Assign::Assign.into())));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Sub.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), ".2");
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Add.into())));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Sub.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "0x78");
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Add.into())));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Sub.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "7.3");
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Add.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "8.9");
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Add.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.slice(), "2e-2");
    assert_eq!(lex.next(), Some(Ok(Punctuator::Semicolon.into())));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_locations() {
    let mut lex = Token::lexer("function_identifier has ; -0x743/*comment*/\n ident //comment");
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.span(), 0..19);
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.span(), 20..23);
    assert_eq!(lex.next(), Some(Ok(Punctuator::Semicolon.into())));
    assert_eq!(lex.span(), 24..25);
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Sub.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.span(), 27..32);
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(lex.span(), 32..43);
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.span(), 45..50);
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(lex.span(), 51..60);
    assert_eq!(lex.next(), None);
}

#[test]
fn test_ambiguities() {
    let mut lex = Token::lexer("<= < + +=a++ => << >>  >>= >>> >>>=  >>>>>=><<=");
    assert_eq!(
        lex.next(),
        Some(Ok(CompareOperator::LessThanOrEqual.into()))
    );
    assert_eq!(lex.next(), Some(Ok(CompareOperator::LessThan.into())));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Add.into())));
    assert_eq!(lex.next(), Some(Ok(Assign::AssignAdd.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(UnaryOperator::Inc.into())));
    assert_eq!(lex.next(), Some(Ok(Punctuator::DoubleArrow.into())));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Shl.into())));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::SAR.into())));
    assert_eq!(lex.next(), Some(Ok(Assign::AssignSar.into())));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Shr.into())));
    assert_eq!(lex.next(), Some(Ok(Assign::AssignShr.into())));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Shr.into())));
    assert_eq!(lex.next(), Some(Ok(Assign::AssignSar.into())));
    assert_eq!(lex.next(), Some(Ok(CompareOperator::GreaterThan.into())));
    assert_eq!(lex.next(), Some(Ok(Assign::AssignShl.into())));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_documentation_comments_parsed_begin() {
    let mut lex = Token::lexer("/// Send $(value / 1000) chocolates to the user");
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(
        lex.slice(),
        "/// Send $(value / 1000) chocolates to the user"
    );
    assert_eq!(lex.next(), None);
}

#[test]
fn test_multiline_documentation_comments_parsed_begin() {
    let mut lex = Token::lexer("/** Send $(value / 1000) chocolates to the user*/");
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(
        lex.slice(),
        "/** Send $(value / 1000) chocolates to the user*/"
    );
    assert_eq!(lex.next(), None);
}

#[test]
fn test_documentation_comments_parsed() {
    let mut lex = Token::lexer("some other tokens /// Send $(value / 1000) chocolates to the user");
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(
        lex.slice(),
        "/// Send $(value / 1000) chocolates to the user"
    );
    assert_eq!(lex.next(), None);
}

#[test]
fn test_multiline_documentation_comments_parsed() {
    let mut lex =
        Token::lexer("some other tokens /**\n* Send $(value / 1000) chocolates to the user\n*/");
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(
        lex.slice(),
        "/**\n* Send $(value / 1000) chocolates to the user\n*/"
    );
    assert_eq!(lex.next(), None);
}

#[test]
fn test_multiline_documentation_no_stars() {
    let mut lex =
        Token::lexer("some other tokens /**\n Send $(value / 1000) chocolates to the user\n*/");
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(
        lex.slice(),
        "/**\n Send $(value / 1000) chocolates to the user\n*/"
    );
    assert_eq!(lex.next(), None);
}

#[test]
fn test_multiline_documentation_whitespace_hell() {
    let mut lex = Token::lexer(
        "some other tokens /** \t \r \n\t \r  * Send $(value / 1000) chocolates to the user\n*/",
    );
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(
        lex.slice(),
        "/** \t \r \n\t \r  * Send $(value / 1000) chocolates to the user\n*/"
    );
    assert_eq!(lex.next(), None);
}

#[test]
fn test_comment_before_eos() {
    let mut lex = Token::lexer("//");
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(lex.slice(), "//");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_documentation_comment_before_eos() {
    let mut lex = Token::lexer("///");
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(lex.slice(), "///");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_empty_multiline_comment() {
    let mut lex = Token::lexer("/**/");
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(lex.slice(), "/**/");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_empty_multiline_documentation_comment_before_eos() {
    let mut lex = Token::lexer("/***/");
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(lex.slice(), "/***/");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_comments_mixed_in_sequence() {
    let mut lex = Token::lexer("hello_world ///documentation comment \n//simple comment \n<<");
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Shl.into())));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_ether_subdenominations() {
    let mut lex = Token::lexer("wei gwei ether");
    assert_eq!(lex.next(), Some(Ok(SubDenomination::Wei.into())));
    assert_eq!(lex.next(), Some(Ok(SubDenomination::Gwei.into())));
    assert_eq!(lex.next(), Some(Ok(SubDenomination::Ether.into())));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_time_subdenominations() {
    let mut lex = Token::lexer("seconds minutes hours days weeks years");
    assert_eq!(lex.next(), Some(Ok(SubDenomination::Second.into())));
    assert_eq!(lex.next(), Some(Ok(SubDenomination::Minute.into())));
    assert_eq!(lex.next(), Some(Ok(SubDenomination::Hour.into())));
    assert_eq!(lex.next(), Some(Ok(SubDenomination::Day.into())));
    assert_eq!(lex.next(), Some(Ok(SubDenomination::Week.into())));
    assert_eq!(lex.next(), Some(Ok(SubDenomination::Year.into())));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_empty_comment() {
    let mut lex = Token::lexer("//\ncontract{}");
    assert_eq!(lex.next(), Some(Ok(Token::Comment)));
    assert_eq!(lex.next(), Some(Ok(Keyword::Contract.into())));
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(lex.next(), Some(Ok(Punctuator::RBrace.into())));
    assert_eq!(lex.next(), None);
}

// Unicode string escapes

#[test]
fn test_valid_unicode_string_escape() {
    let mut lex = Token::lexer("{ \"\\u00DAnicode\"");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Ok(Literal::String("\u{00DA}nicode".into()).into()))
    );
    assert_eq!(lex.slice(), "\"\\u00DAnicode\"");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_valid_unicode_string_escape_7f() {
    let mut lex = Token::lexer("{ \"\\u007Fnicode\"");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Ok(Literal::String("\u{007F}nicode".into()).into()))
    );
    assert_eq!(lex.slice(), "\"\\u007Fnicode\"");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_valid_unicode_string_escape_7ff() {
    let mut lex = Token::lexer("{ \"\\u07FFnicode\"");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Ok(Literal::String("\u{07FF}nicode".into()).into()))
    );
    assert_eq!(lex.slice(), "\"\\u07FFnicode\"");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_valid_unicode_string_escape_ffff() {
    let mut lex = Token::lexer("{ \"\\uFFFFnicode\"");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Ok(Literal::String("\u{FFFF}nicode".into()).into()))
    );
    assert_eq!(lex.slice(), "\"\\uFFFFnicode\"");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_invalid_short_unicode_string_escape() {
    let mut lex = Token::lexer("{ \"\\uFFnicode\"");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Err(LexerError::IllegalEscapeSequence("\\uFFni".into())))
    );
    assert_eq!(lex.next(), None);
}

// Unicode string literal

#[test]
fn test_unicode_prefix_only() {
    let mut lex = Token::lexer("{ unicode");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(lex.next(), Some(Err(LexerError::IllegalToken)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_unicode_invalid_space() {
    let mut lex = Token::lexer("{ unicode ");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(lex.next(), Some(Err(LexerError::IllegalToken)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_unicode_invalid_token() {
    let mut lex = Token::lexer("{ unicode test");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(lex.next(), Some(Err(LexerError::IllegalToken)));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_valid_unicode_literal() {
    let mut lex = Token::lexer("{ unicode\"Hello 😃\"");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Ok(Literal::UnicodeString("Hello 😃".into()).into()))
    );
    assert_eq!(lex.slice(), "unicode\"Hello 😃\"");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_valid_nonprintable_in_unicode_literal() {
    let mut lex = Token::lexer("{ unicode\"Hello \u{7}😃\"");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Ok(Literal::UnicodeString("Hello \u{7}😃".into()).into()))
    );
    assert_eq!(lex.slice(), "unicode\"Hello \u{7}😃\"");
    assert_eq!(lex.next(), None);
}

// Hex string literal

#[test]
fn test_hex_prefix_only() {
    let mut lex = Token::lexer("{ hex");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(lex.next(), Some(Err(LexerError::IllegalToken)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_hex_invalid_space() {
    let mut lex = Token::lexer("{ hex ");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(lex.next(), Some(Err(LexerError::IllegalToken)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_hex_invalid_token() {
    let mut lex = Token::lexer("{ hex test");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(lex.next(), Some(Err(LexerError::IllegalToken)));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_valid_hex_literal() {
    let mut lex = Token::lexer("{ hex\"00112233FF\"");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(
        lex.next(),
        Some(Ok(Literal::HexString("00112233FF".into()).into()))
    );
    assert_eq!(lex.slice(), "hex\"00112233FF\"");
    assert_eq!(lex.next(), None);
}

#[test]
fn test_invalid_short_hex_literal() {
    let mut lex = Token::lexer("{ hex\"00112233F\"");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(lex.next(), Some(Err(LexerError::IllegalHexString)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_invalid_hex_literal_with_space() {
    let mut lex = Token::lexer("{ hex\"00112233FF \"");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(lex.next(), Some(Err(LexerError::IllegalHexString)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_invalid_hex_literal_with_wrong_quotes() {
    let mut lex = Token::lexer("{ hex\"00112233FF'");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(lex.next(), Some(Err(LexerError::IllegalHexString)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_invalid_hex_literal_nonhex_string() {
    let mut lex = Token::lexer("{ hex\"hello\"");
    assert_eq!(lex.next(), Some(Ok(Punctuator::LBrace.into())));
    assert_eq!(lex.next(), Some(Err(LexerError::IllegalHexString)));
    assert_eq!(lex.next(), None);
}

// Comments

#[test]
fn test_invalid_multiline_comment_close() {
    let mut lex = Token::lexer("/** / x");
    assert_eq!(lex.next(), Some(Err(LexerError::OpenMultilineComment)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_multiline_doc_comment_at_eos() {
    let mut lex = Token::lexer("/**");
    assert_eq!(lex.next(), Some(Err(LexerError::OpenMultilineComment)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_multiline_comment_at_eos() {
    let mut lex = Token::lexer("/*");
    assert_eq!(lex.next(), Some(Err(LexerError::OpenMultilineComment)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_regular_line_break_in_single_line_comment() {
    for nl in ["\r", "\n", "\r\n"] {
        let inp = format!("// abc {} def ", nl);
        let mut lex = Token::lexer(&inp);
        assert_eq!(lex.next(), Some(Ok(Token::Comment)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), None);
    }
}

#[test]
fn test_irregular_line_breaks_in_single_line_comment() {
    for nl in ["\x0C", "\x0B", "\u{2028}", "\u{2029}"] {
        let inp = format!("// abc {} def ", nl);
        let mut lex = Token::lexer(&inp);
        assert_eq!(lex.next(), Some(Ok(Token::Comment)));
        assert_eq!(lex.next(), Some(Err(LexerError::UnexpectedToken)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), None);
    }
}

#[test]
fn test_regular_line_breaks_in_single_line_doc_comment() {
    for nl in ["\r", "\n", "\r\n"] {
        let inp = format!("/// abc {} def ", nl);
        let mut lex = Token::lexer(&inp);
        assert_eq!(lex.next(), Some(Ok(Token::Comment)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), None);
    }
}

#[test]
fn test_regular_line_breaks_in_multiline_doc_comment() {
    for nl in ["\r", "\n", "\r\n"] {
        let inp = format!("/// Hello{}/// World{}ident", nl, nl);
        let mut lex = Token::lexer(&inp);
        assert_eq!(lex.next(), Some(Ok(Token::Comment)));
        assert_eq!(lex.next(), Some(Ok(Token::Comment)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), None);
    }
}

#[test]
fn test_irregular_line_breaks_in_single_line_doc_comment() {
    for nl in ["\x0C", "\x0B", "\u{2028}", "\u{2029}"] {
        let inp = format!("/// abc {} def ", nl);
        let mut lex = Token::lexer(&inp);
        assert_eq!(lex.next(), Some(Ok(Token::Comment)));
        assert_eq!(lex.next(), Some(Err(LexerError::UnexpectedToken)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), None);
    }
}

#[test]
fn test_regular_line_breaks_in_strings() {
    for nl in ["\r", "\n", "\r\n"] {
        let inp = format!("\"abc {} def\"", nl);
        let mut lex = Token::lexer(&inp);
        assert_eq!(lex.next(), Some(Err(LexerError::IllegalStringEndQuote)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), Some(Err(LexerError::UnexpectedEndOfString)));
        assert_eq!(lex.next(), None);
    }
}

#[test]
fn test_irregular_line_breaks_in_strings() {
    for nl in ['\x0B', '\x0C', '\u{2028}', '\u{2029}'] {
        let inp = format!("\"abc {} def\"", nl);
        let mut lex = Token::lexer(&inp);
        if nl == '\u{2028}' || nl == '\u{2029}' {
            assert_eq!(
                lex.next(),
                Some(Err(LexerError::UnicodeCharacterInNonUnicodeString(nl)))
            );
        } else {
            assert_eq!(lex.next(), Some(Err(LexerError::IllegalStringEndQuote)));
        }
        assert_eq!(lex.next(), Some(Err(LexerError::UnexpectedToken)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), Some(Err(LexerError::UnexpectedEndOfString)));
        assert_eq!(lex.next(), None);
    }
}

#[test]
fn test_solidity_keywords() {
    let keywords = "return byte bool address var in true false leave switch case default";
    let mut lex = Token::lexer(keywords);
    assert_eq!(lex.next(), Some(Ok(Keyword::Return.into())));
    assert_eq!(lex.next(), Some(Ok(ReservedKeyword::Byte.into())));
    assert_eq!(lex.next(), Some(Ok(TypeKeyword::Bool.into())));
    assert_eq!(lex.next(), Some(Ok(TypeKeyword::Address.into())));
    assert_eq!(lex.next(), Some(Ok(ReservedKeyword::Var.into())));
    assert_eq!(lex.next(), Some(Ok(ReservedKeyword::In.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::True.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::False.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(ReservedKeyword::Switch.into())));
    assert_eq!(lex.next(), Some(Ok(ReservedKeyword::Case.into())));
    assert_eq!(lex.next(), Some(Ok(ReservedKeyword::Default.into())));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_yul_keyword_like() {
    let mut lex = Token::lexer("leave.function");
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Punctuator::Period.into())));
    assert_eq!(lex.next(), Some(Ok(Keyword::Function.into())));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_yul_identifier_with_dots() {
    let mut lex = Token::lexer("mystorage.slot := 1");
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Punctuator::Period.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(InlineAssemblyOperator::Assign.into())));
    assert_eq!(lex.next(), Some(Ok(Literal::Number.into())));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_yul_function() {
    let mut lex = Token::lexer("function f(a, b) -> x, y");
    assert_eq!(lex.next(), Some(Ok(Keyword::Function.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Punctuator::LParen.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Comma.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Punctuator::RParen.into())));
    assert_eq!(lex.next(), Some(Ok(Punctuator::RightArrow.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Comma.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_yul_function_with_whitespace() {
    let mut lex = Token::lexer("function f (a, b) - > x, y");
    assert_eq!(lex.next(), Some(Ok(Keyword::Function.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Punctuator::LParen.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Comma.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(Punctuator::RParen.into())));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Sub.into())));
    assert_eq!(lex.next(), Some(Ok(CompareOperator::GreaterThan.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), Some(Ok(BinaryOperator::Comma.into())));
    assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lex.next(), None);
}
