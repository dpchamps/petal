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
    UnexpectedParserState(Span)
}

type ParseResult<T> = Result<T, ParseErr>;

impl<'a> Parser<'a> {
    fn advance(&mut self) {
        let mut next = None;
        while let Some((token, error)) = self.lexer.next() {
            self.last_position = self.position;
            self.position += token.len;

            if token.kind != SyntaxKind::WHITESPACE {
                next = Some((token, error));
                break;
            }

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

        match self.current {
            Some(next) if next.kind == kind => {
                self.advance();
                Some(())
            }
            _ => None,
        }
    }

    fn eat_raw(&mut self, input: &str) -> bool {
        if self.is(input) {
            self.advance();
            return true
        }

        false
    }

    fn raw(&mut self) -> &'a str {
        &self.source[TextRange::new(
            TextSize::from(self.last_position as u32),
            TextSize::from((self.position as u32)),
        )]
    }

    fn is(&mut self, input: &str) -> bool {
        if self.current.is_none() {
            self.advance()
        }

        self.raw() == input
    }

    fn span(&self) -> Span {
        Span::new(BytePos(self.last_position as u32), BytePos(self.position as u32), SyntaxContext::empty())
    }

    fn span_start(&self) -> BytePos {
        BytePos(self.last_position as u32)
    }

    fn finish_span(&self, span_start: BytePos) -> Span {
        Span::new(span_start, BytePos(self.position as u32), SyntaxContext::empty())
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
        parser.advance();

        Ok(Program::Module(parser.parse_module()?))
    }

    fn parse_module(&mut self) -> Result<Module, ParseErr> {
        let start = self.span_start();
        let module_item_list = self.parse_module_items()?;

        Ok(Module {
            span: self.finish_span(start),
            body: module_item_list,
            shebang: None,
        })
    }

    fn parse_module_items(&mut self) -> ParseResult<Vec<ModuleItem>>
    {
        let mut items = vec![];
        while self.current.is_some() {
            items.push(self.parse_module_item()?)
        }

        Ok(items)
    }

    fn parse_module_item(&mut self) -> ParseResult<ModuleItem> {
        if self.is("import") || self.is("export") {
            return Ok(ModuleItem::ModuleDecl(self.parse_import_export()?))
        }

        Ok(ModuleItem::Stmt(self.parse_stmt_list_item()?))
    }

    fn parse_import_export(&mut self) -> ParseResult<ModuleDecl> {
        let start = self.span_start();

        if self.eat_raw("import") {
            return Ok(self.parse_import_declaration()?.into());
        }

        if self.eat_raw("export") {
            return Ok(self.parse_export_declaration()?.into());
        }

        Err(ParseErr::UnexpectedParserState(self.finish_span(start)))
    }

    fn parse_import_declaration(&mut self) -> ParseResult<ImportDecl> {
        todo!()
    }

    fn parse_export_declaration(&mut self) -> ParseResult<ExportDecl> {
        todo!()
    }

    fn parse_stmt_list_item(&mut self) -> ParseResult<Stmt> {
        todo!()
    }

    pub(crate) fn parse_ident(&mut self) -> ParseResult<Ident> {
        self.eat(SyntaxKind::IDENT).ok_or(ParseErr::CatchAll)?;
        let ident_name = self.raw();

        Ok(Ident {
            span: self.span(),
            sym: ident_name.into(),
            optional: false
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn is() {
        let source = "abra kadabra alakazam";
        let mut parser = Parser::new(source);

        assert!(parser.is("abra"));
        parser.advance();
        assert!(parser.is("kadabra"));
        parser.advance();
        assert!(parser.is("alakazam"));
    }
}