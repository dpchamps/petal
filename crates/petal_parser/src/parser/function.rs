use crate::parser::{ParseErr, ParseResult, Parser};
use rslint_lexer::SyntaxKind;

use swc_petal_ast::*;

impl<'a> Parser<'a> {
    pub(super) fn parse_function(&mut self) -> ParseResult<Function> {
        todo!()
    }
}