use Petal::parser::lexer::{Lexer, LexingError};
use Petal::parser::tokens::Token;

#[test]
fn lex_file_expected_output() {
    let source = std::fs::read_to_string("./tests/lexer-fixture.js").unwrap();
    let mut lexer = Lexer::new(source);
    let mut tokens: Vec<Token> = vec![];

    let expectation = vec![
        Token::Keyword(String::from("import")),
        Token::Punctuator(String::from("{")),
        Token::IdentifierName(String::from("x")),
        Token::Punctuator(String::from("}")),
        Token::ConditionalKeyword(String::from("from")),
        Token::StringLiteral(String::from("y")),
        Token::Punctuator(String::from(";")),
        Token::Keyword(String::from("import")),
        Token::Punctuator(String::from("*")),
        Token::ConditionalKeyword(String::from("as")),
        Token::IdentifierName(String::from("myThing")),
        Token::ConditionalKeyword(String::from("from")),
        Token::StringLiteral(String::from("z")),
        Token::Punctuator(String::from(";")),
        Token::Keyword(String::from("function")),
        Token::IdentifierName(String::from("x")),
        Token::Punctuator(String::from("(")),
        Token::Punctuator(String::from(")")),
        Token::Punctuator(String::from("{")),
        Token::Keyword(String::from("return")),
        Token::NumericLiteral(1.0),
        Token::Punctuator(String::from("+")),
        Token::NumericLiteral(2.0),
        Token::Punctuator(String::from("+")),
        Token::NumericLiteral(3.0),
        Token::Punctuator(String::from("}")),
        Token::Keyword(String::from("const")),
        Token::IdentifierName(String::from("y")),
        Token::Punctuator(String::from("=")),
        Token::Keyword(String::from("function")),
        Token::Punctuator(String::from("(")),
        Token::Punctuator(String::from(")")),
        Token::Punctuator(String::from("{")),
        Token::Punctuator(String::from("}")),
        Token::Keyword(String::from("const")),
        Token::IdentifierName(String::from("z")),
        Token::Punctuator(String::from("=")),
        Token::Punctuator(String::from("(")),
        Token::Punctuator(String::from(")")),
        Token::Punctuator(String::from("=>")),
        Token::Punctuator(String::from("{")),
        Token::Punctuator(String::from("}")),
        Token::Keyword(String::from("function")),
        Token::IdentifierName(String::from("inline")),
        Token::Punctuator(String::from("(")),
        Token::IdentifierName(String::from("x")),
        Token::Punctuator(String::from(",")),
        Token::IdentifierName(String::from("y")),
        Token::Punctuator(String::from(")")),
        Token::Punctuator(String::from("{")),
        Token::Punctuator(String::from("}")),
        Token::Keyword(String::from("const")),
        Token::IdentifierName(String::from("template")),
        Token::Punctuator(String::from("=")),
        Token::NoSubstitutionTemplate(String::from("\n\n\nHere\'s a\n\nmultiline\n\ntl")),
        Token::Punctuator(String::from(";")),
        Token::Keyword(String::from("const")),
        Token::IdentifierName(String::from("templateParts")),
        Token::Punctuator(String::from("=")),
        Token::TemplateHead(String::from("")),
        Token::RegexLiteral(String::from("x"), String::from("y")),
        Token::TemplateMiddle(String::from("z")),
        Token::StringLiteral(String::from("_")),
        Token::TemplateTail(String::from("")),
        Token::Keyword(String::from("const")),
        Token::IdentifierName(String::from("regex")),
        Token::Punctuator(String::from("=")),
        Token::RegexLiteral(String::from("^[xds]$"), String::from("")),
        Token::Punctuator(String::from(";")),
    ];

    while let Ok(token) = lexer.next() {
        if token == Token::EOF {
            break;
        }
        tokens.push(token);
    }

    assert_eq!(tokens, expectation)
}