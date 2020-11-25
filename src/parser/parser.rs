use crate::parser::lexer::Lexer;
use crate::parser::tokens::Token;

pub struct Parser {
    lexer: Lexer,
    currentToken: Option<Token>
}

impl Parser {
    pub fn new(source: &String) -> Self {
        Parser {
            lexer: Lexer::new(source.clone()),
            currentToken: None,
        }
    }
}
