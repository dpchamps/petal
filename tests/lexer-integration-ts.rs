use Petal::parser::lexer::{Lexer, LexingError};
use Petal::parser::tokens::Token;

#[test]
fn lex_ts_expected_output() {
    let source = std::fs::read_to_string("./tests/lexer-ts-fixture.ts").unwrap();
    let mut lexer = Lexer::new(source);
    let tokens: Vec<Token> = lexer.into_iter().collect();

    let expectation = vec![
        Token::IdentifierName(String::from("type")),
        Token::IdentifierName(String::from("A")),
        Token::Punctuator(String::from("=")),
        Token::StringLiteral(String::from("x")),
        Token::Punctuator(String::from("|")),
        Token::StringLiteral(String::from("y")),
        Token::Punctuator(String::from("|")),
        Token::StringLiteral(String::from("z")),
        Token::IdentifierName(String::from("type")),
        Token::IdentifierName(String::from("B")),
        Token::Punctuator(String::from("=")),
        Token::Punctuator(String::from("{")),
        Token::IdentifierName(String::from("b")),
        Token::Punctuator(String::from(":")),
        Token::StringLiteral(String::from("b")),
        Token::Punctuator(String::from("}")),
        Token::IdentifierName(String::from("type")),
        Token::IdentifierName(String::from("C")),
        Token::Punctuator(String::from("=")),
        Token::Punctuator(String::from("{")),
        Token::IdentifierName(String::from("c")),
        Token::Punctuator(String::from(":")),
        Token::StringLiteral(String::from("c")),
        Token::Punctuator(String::from("}")),
        Token::IdentifierName(String::from("type")),
        Token::IdentifierName(String::from("X")),
        Token::Punctuator(String::from("<")),
        Token::IdentifierName(String::from("T")),
        Token::Punctuator(String::from(">")),
        Token::Punctuator(String::from("=")),
        Token::IdentifierName(String::from("T")),
        Token::Keyword(String::from("extends")),
        Token::IdentifierName(String::from("A")),
        Token::Punctuator(String::from("?")),
        Token::StringLiteral(String::from("Good")),
        Token::Punctuator(String::from(":")),
        Token::StringLiteral(String::from("Bad")),
        Token::Punctuator(String::from(";")),
        Token::Keyword(String::from("const")),
        Token::IdentifierName(String::from("returnsGood")),
        Token::Punctuator(String::from("=")),
        Token::Punctuator(String::from("(")),
        Token::Punctuator(String::from(")")),
        Token::Punctuator(String::from(":")),
        Token::IdentifierName(String::from("X")),
        Token::Punctuator(String::from("<")),
        Token::StringLiteral(String::from("x")),
        Token::Punctuator(String::from(">")),
        Token::Punctuator(String::from("=>")),
        Token::StringLiteral(String::from("Good")),
        Token::Punctuator(String::from(";")),
        Token::Keyword(String::from("const")),
        Token::IdentifierName(String::from("returnsBad")),
        Token::Punctuator(String::from("=")),
        Token::Punctuator(String::from("(")),
        Token::Punctuator(String::from(")")),
        Token::Punctuator(String::from(":")),
        Token::IdentifierName(String::from("X")),
        Token::Punctuator(String::from("<")),
        Token::StringLiteral(String::from("j")),
        Token::Punctuator(String::from(">")),
        Token::Punctuator(String::from("=>")),
        Token::StringLiteral(String::from("Bad")),
        Token::Punctuator(String::from(";")),
        Token::IdentifierName(String::from("type")),
        Token::IdentifierName(String::from("fn")),
        Token::Punctuator(String::from("=")),
        Token::Punctuator(String::from("(")),
        Token::Punctuator(String::from(")")),
        Token::Punctuator(String::from("=>")),
        Token::IdentifierName(String::from("Promise")),
        Token::Punctuator(String::from("<")),
        Token::IdentifierName(String::from("unknown")),
        Token::Punctuator(String::from(">")),
        Token::Punctuator(String::from(";")),
        Token::Keyword(String::from("const")),
        Token::IdentifierName(String::from("asynFn")),
        Token::Punctuator(String::from(":")),
        Token::IdentifierName(String::from("fn")),
        Token::Punctuator(String::from("=")),
        Token::Punctuator(String::from("(")),
        Token::Punctuator(String::from(")")),
        Token::Punctuator(String::from("=>")),
        Token::Keyword(String::from("new")),
        Token::IdentifierName(String::from("Promise")),
        Token::Punctuator(String::from("<")),
        Token::IdentifierName(String::from("number")),
        Token::Punctuator(String::from(">")),
        Token::Punctuator(String::from("(")),
        Token::Punctuator(String::from("(")),
        Token::Punctuator(String::from(")")),
        Token::Punctuator(String::from("=>")),
        Token::Punctuator(String::from("{")),
        Token::Punctuator(String::from("}")),
        Token::Punctuator(String::from(")")),
        Token::Punctuator(String::from(";")),
        Token::Keyword(String::from("const")),
        Token::IdentifierName(String::from("intersection")),
        Token::Punctuator(String::from("=")),
        Token::Punctuator(String::from("(")),
        Token::Punctuator(String::from(")")),
        Token::Punctuator(String::from(":")),
        Token::IdentifierName(String::from("B")),
        Token::Punctuator(String::from("&")),
        Token::IdentifierName(String::from("C")),
        Token::Punctuator(String::from("=>")),
        Token::Punctuator(String::from("(")),
        Token::Punctuator(String::from("{")),
        Token::IdentifierName(String::from("b")),
        Token::Punctuator(String::from(":")),
        Token::StringLiteral(String::from("b")),
        Token::Punctuator(String::from(",")),
        Token::IdentifierName(String::from("c")),
        Token::Punctuator(String::from(":")),
        Token::StringLiteral(String::from("c")),
        Token::Punctuator(String::from(",")),
        Token::Punctuator(String::from("}")),
        Token::Punctuator(String::from(")")),
        Token::Punctuator(String::from(";")),
    ];

    assert_eq!(tokens, expectation);
}
