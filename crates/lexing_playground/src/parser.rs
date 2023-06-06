use rslint_errors::Diagnostic;
use rslint_lexer::{Lexer, LexerReturn, SyntaxKind, Token};
use rslint_rowan::{TextRange, TextSize};
use swc_common::{BytePos, Span, SyntaxContext};
use swc_common::util::take::Take;
use swc_petal_ast::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    source: &'a str,
    current: Option<Token>,
    error_state: Vec<Diagnostic>,
    last_position: usize,
    position: usize,
}

#[derive(Debug)]
pub enum ParseErr {
    CatchAll,
}

impl<'a> Parser<'a> {
    fn advance(&mut self) {
        let mut next = None;
        while let Some((token, error)) = self.lexer.next() {
            if token.kind != SyntaxKind::WHITESPACE {
                next = Some((token, error));
                break;
            }
            self.last_position = self.position;
            self.position += token.len;
        }

        if let Some((token, diagnostic)) = next {
            self.current = Some(token);

            if let Some(diagnostic) = diagnostic {
                self.error_state.push(diagnostic)
            }
        }
    }

    fn eat(&mut self, kind: SyntaxKind) -> Option<()> {
        if let None = self.current {
            self.advance();
        }

        println!("\t Current: {:?}", self.current);

        match self.current {
            Some(next) if next.kind == kind => {
                self.current = None;
                self.last_position = self.position;
                self.position += next.len;
                Some(())
            }
            _ => None,
        }
    }

    fn raw(&mut self) -> &'a str {
        println!("last: {}, current: {}", self.last_position, self.position);
        &self.source[TextRange::new(
            TextSize::from(self.last_position as u32),
            TextSize::from((self.position as u32)),
        )]
    }

    fn span(&self) -> Span {
        Span::new(BytePos(self.last_position as u32), BytePos(self.position as u32), SyntaxContext::empty())
    }

    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::from_str(source, 0);

        Self {
            lexer,
            source,
            current: None,
            error_state: vec![],
            position: 0,
            last_position: 0,
        }
    }

    pub fn parse(input: &'a str) -> Result<Program, ParseErr> {
        let mut parser = Parser::new(input);

        Ok(Program::Module(parser.parse_module()?))
    }

    fn parse_module(&self) -> Result<Module, ParseErr> {
        Ok(Module::dummy())
    }

    fn parse_body_block<Type>(&self) -> Vec<Type> {
        vec![]
    }

    fn parse_module_item(&self) -> ModuleItem {
        unimplemented!()
    }

    pub(crate) fn parse_ident(&mut self) -> Result<Ident, ParseErr> {
        self.eat(SyntaxKind::IDENT).ok_or(ParseErr::CatchAll)?;
        let ident_name = self.raw();

        Ok(Ident {
            span: self.span(),
            sym: ident_name.into(),
            optional: false
        })
    }
}
