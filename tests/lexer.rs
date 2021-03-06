use Petal::parser::lexer::{Lexer, LexingError};
use Petal::parser::tokens::Token;
#[test]
fn whitespace() {
    let mut lexer = Lexer::new(String::from("      \n\n     "));

    assert_eq!(Ok(Token::EOF), lexer.next());
}

#[test]
fn single_line_comment() {
    let mut lexer = Lexer::new(String::from("// This is a comment"));

    assert_eq!(Ok(Token::EOF), lexer.next());
}

#[test]
fn single_line_comment_newline() {
    let mut lexer = Lexer::new(String::from("// This is a comment \n 10"));

    assert_eq!(lexer.next(), Ok(Token::NumericLiteral(10.0)));
}

#[test]
fn multiline_comment() {
    let input = "/** \
                 This is a multi line doc block **\
                 ** Still going\
                 */";
    let mut lexer = Lexer::new(String::from(input));

    assert_eq!(Ok(Token::EOF), lexer.next());
}

#[test]
fn multiline_comment_continue() {
    let input = "/** \
                 This is a multi line doc block **\
                 ** Still going\
                 */\
                 500";
    let mut lexer = Lexer::new(String::from(input));

    assert_eq!(Ok(Token::NumericLiteral(500.0)), lexer.next());
    assert_eq!(Ok(Token::EOF), lexer.next());
}

#[test]
fn lexer_digit() {
    let mut lexer = Lexer::new(String::from("18"));

    assert_eq!(Token::NumericLiteral(18.0), lexer.next().unwrap());
}

#[test]
fn lexer_decimal_digit() {
    let mut lexer = Lexer::new(String::from("18.123"));

    assert_eq!(Token::NumericLiteral(18.123), lexer.next().unwrap());
}

#[test]
fn lexer_decimal_digit_exponent() {
    let mut lexer = Lexer::new(String::from("18e10"));

    assert_eq!(Token::NumericLiteral(180000000000.0), lexer.next().unwrap());
}

#[test]
fn lexer_digit_signed_exponent() {
    let mut lexer = Lexer::new(String::from("18.12E-5"));

    assert_eq!(Token::NumericLiteral(0.0001812), lexer.next().unwrap());
}

#[test]
fn lexer_digit_big_int() {
    let mut lexer = Lexer::new(String::from("100234n"));

    assert_eq!(
        Token::BigIntLiteral(String::from("100234"), 10),
        lexer.next().unwrap()
    );
}

#[test]
fn lexer_bigint_zero() {
    let mut lexer = Lexer::new(String::from("0n"));

    assert_eq!(
        Ok(Token::BigIntLiteral(String::from("0"), 10)),
        lexer.next()
    )
}

#[test]
fn lexer_digit_binary() {
    let mut lexer = Lexer::new(String::from("0b10"));
    let mut lexer_bigint = Lexer::new(String::from("0b10n"));

    assert_eq!(Token::NumericLiteral(2.0), lexer.next().unwrap());
    assert_eq!(
        Token::BigIntLiteral(String::from("10"), 2),
        lexer_bigint.next().unwrap()
    );
}

#[test]
fn lexer_digit_octal() {
    let mut lexer = Lexer::new(String::from("0o55"));
    let mut lexer_bigint = Lexer::new(String::from("0o55n"));

    assert_eq!(Token::NumericLiteral(45.0), lexer.next().unwrap());
    assert_eq!(
        Token::BigIntLiteral(String::from("55"), 8),
        lexer_bigint.next().unwrap()
    );
}

#[test]
fn lexer_digit_hex() {
    let mut lexer = Lexer::new(String::from("0xFFFF"));
    let mut lexer_bigint = Lexer::new(String::from("0xFFFFn"));

    assert_eq!(Token::NumericLiteral(65535.0), lexer.next().unwrap());
    assert_eq!(
        Token::BigIntLiteral(String::from("FFFF"), 16),
        lexer_bigint.next().unwrap()
    );
}

#[test]
fn lexer_number_invalid_token() {
    let mut lexer_char = Lexer::new(String::from("10i"));
    let mut lexer_digit = Lexer::new(String::from("20n100"));

    assert_eq!(Err(LexingError::InvalidToken), lexer_char.next());
    assert_eq!(Err(LexingError::InvalidToken), lexer_digit.next());
}

#[test]
fn null_literal() {
    let mut lexer = Lexer::new(String::from("null"));

    assert_eq!(Ok(Token::NullLiteral), lexer.next());
}

#[test]
fn boolean_literal() {
    let mut lexer = Lexer::new(String::from("true false"));

    assert_eq!(Ok(Token::BooleanLiteral(true)), lexer.next());
    assert_eq!(Ok(Token::BooleanLiteral(false)), lexer.next());
}

#[test]
fn punctuator() {
    let mut lexer = Lexer::new(String::from("= >>> !=="));

    assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("="))));
    assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from(">>>"))));
    assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("!=="))));
}

#[test]
fn punctuator_greedy_backtrack() {
    let mut lexer = Lexer::new(String::from("<<< -->"));

    assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("<<"))));
    assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("<"))));

    assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("--"))));

    assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from(">"))));
}

#[test]
fn ident_keyword() {
    let mut lexer = Lexer::new(String::from("break do in super this"));

    assert_eq!(lexer.next(), Ok(Token::Keyword(String::from("break"))));
    assert_eq!(lexer.next(), Ok(Token::Keyword(String::from("do"))));
    assert_eq!(lexer.next(), Ok(Token::Keyword(String::from("in"))));
    assert_eq!(lexer.next(), Ok(Token::Keyword(String::from("super"))));
    assert_eq!(lexer.next(), Ok(Token::Keyword(String::from("this"))));
}

#[test]
fn ident_conditional_keyword() {
    let mut lexer = Lexer::new(String::from("as async package yield"));

    assert_eq!(
        lexer.next(),
        Ok(Token::ConditionalKeyword(String::from("as")))
    );
    assert_eq!(
        lexer.next(),
        Ok(Token::ConditionalKeyword(String::from("async")))
    );
    assert_eq!(
        lexer.next(),
        Ok(Token::ConditionalKeyword(String::from("package")))
    );
    assert_eq!(
        lexer.next(),
        Ok(Token::ConditionalKeyword(String::from("yield")))
    );
}

#[test]
fn ident_reserved_keyword() {
    let mut lexer = Lexer::new(String::from("enum"));

    assert_eq!(
        lexer.next(),
        Ok(Token::ReservedKeyword(String::from("enum")))
    );
}

#[test]
fn ident_name() {
    let mut lexer = Lexer::new(String::from(r"x $ _ \uAAFF $x$_\uA38D"));

    assert_eq!(lexer.next(), Ok(Token::IdentifierName(String::from("x"))));
    assert_eq!(lexer.next(), Ok(Token::IdentifierName(String::from("$"))));
    assert_eq!(lexer.next(), Ok(Token::IdentifierName(String::from("_"))));
    assert_eq!(
        lexer.next(),
        Ok(Token::IdentifierName(String::from("\u{aaff}")))
    );
    assert_eq!(
        lexer.next(),
        Ok(Token::IdentifierName(String::from("$x$_ꎍ")))
    );
}

#[test]
fn ident_early_exit() {
    let mut lexer = Lexer::new(String::from(r"\u{A0000}"));

    println!("{:?}", lexer.next());
}

#[test]
fn string_literal_single_quote() {
    let mut lexer = Lexer::new(String::from(r"'abra\tkadabra \n\n hullo\0 \ufa45 \xFA'"));
    let expectation = String::from("abra	kadabra \n\n hullo  海 ú");

    assert_eq!(Ok(Token::StringLiteral(expectation)), lexer.next());
}

#[test]
fn string_literal_double_quote() {
    let mut lexer = Lexer::new(String::from(r#""abra\tkadabra \n\n hullo\0 \ufa45 \xFA""#));
    let expectation = String::from("abra	kadabra \n\n hullo  海 ú");

    assert_eq!(Ok(Token::StringLiteral(expectation)), lexer.next());
}

#[test]
fn no_substitution_template() {
    let mut lexer = Lexer::new(String::from(r"`abra\tkadabra \n\n hullo\0 \ufa45 \xFA`"));
    let expectation = String::from("abra	kadabra \n\n hullo  海 ú");

    assert_eq!(Ok(Token::NoSubstitutionTemplate(expectation)), lexer.next());
}

#[test]
fn template_parts() {
    let mut lexer = Lexer::new(String::from("`a${b}c${d}e`"));

    assert_eq!(lexer.next(), Ok(Token::TemplateHead(String::from("a"))));
    assert_eq!(lexer.next(), Ok(Token::IdentifierName(String::from("b"))));
    assert_eq!(lexer.next(), Ok(Token::TemplateMiddle(String::from("c"))));
    assert_eq!(lexer.next(), Ok(Token::IdentifierName(String::from("d"))));
    assert_eq!(lexer.next(), Ok(Token::TemplateTail(String::from("e"))));
}

#[test]
fn template_edge_cases() {
    let mut lexer = Lexer::new(String::from(r"`Price is $${price.replace(/,/g,'.')} \$\{`"));

    assert_eq!(
        lexer.next(),
        Ok(Token::TemplateHead(String::from("Price is $")))
    );
    assert_eq!(
        lexer.next(),
        Ok(Token::IdentifierName(String::from("price")))
    );
    assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("."))));
    assert_eq!(
        lexer.next(),
        Ok(Token::IdentifierName(String::from("replace")))
    );
    assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("("))));
    assert_eq!(
        lexer.next(),
        Ok(Token::RegexLiteral(String::from(","), String::from("g")))
    );
    assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from(","))));
    assert_eq!(lexer.next(), Ok(Token::StringLiteral(String::from("."))));
    assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from(")"))));
    assert_eq!(lexer.next(), Ok(Token::TemplateTail(String::from(" ${"))));
}

#[test]
fn regex() {
    let mut lexer = Lexer::new(String::from("/abx/gi"));

    assert_eq!(
        lexer.next(),
        Ok(Token::RegexLiteral(String::from("abx"), String::from("gi")))
    );
}

#[test]
fn regex_with_class() {
    let mut lexer = Lexer::new(String::from("/^[xds]$/"));
    assert_eq!(
        lexer.next(),
        Ok(Token::RegexLiteral(
            String::from("^[xds]$"),
            String::from("")
        ))
    );
}

#[test]
fn regex_ambiguity() {
    let mut lexer = Lexer::new(String::from("('a')/a/g {}/a/g +{}/a/g"));

    lexer.next();
    lexer.next();
    lexer.next();

    assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("/"))));
    assert_eq!(lexer.next(), Ok(Token::IdentifierName(String::from("a"))));
    assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("/"))));
    assert_eq!(lexer.next(), Ok(Token::IdentifierName(String::from("g"))));

    lexer.next();
    lexer.next();

    assert_eq!(
        lexer.next(),
        Ok(Token::RegexLiteral(String::from("a"), String::from("g")))
    );
}

#[test]
fn automatic_semicolon_insertion() {
    let source = String::from("const x = y\nconst z = a");
    let mut lexer = Lexer::new(source);

    assert_eq!(lexer.next().unwrap(), Token::Keyword(String::from("const")));
    assert_eq!(
        lexer.next().unwrap(),
        Token::IdentifierName(String::from("x"))
    );
    assert_eq!(lexer.next().unwrap(), Token::Punctuator(String::from("=")));
    assert_eq!(
        lexer.next().unwrap(),
        Token::IdentifierName(String::from("y"))
    );
    assert_eq!(lexer.next().unwrap(), Token::Keyword(String::from("const")));

    let token_1 = lexer.automatic_semicolon_insertion();
    let token_2 = lexer.next().unwrap();

    assert_eq!(token_1, Token::Punctuator(String::from(";")));
    assert_eq!(token_2, Token::Keyword(String::from("const")));
}
