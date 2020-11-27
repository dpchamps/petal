use crate::parser::ast::ProgramNode;
use crate::parser::lexer::Lexer;
use crate::parser::tokens::Token;

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(source: &String) -> Self {
        Parser {
            lexer: Lexer::new(source.clone()),
            current_token: None,
        }
    }

    pub fn parse(&mut self) -> ProgramNode {
        unimplemented!()
    }
}
