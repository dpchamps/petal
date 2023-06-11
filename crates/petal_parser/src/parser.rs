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
    UnexpectedParserState(Span),
    Semicolon(Span)
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

    fn eat(&mut self, kind: SyntaxKind) -> Option<Token> {
        if let None = self.current {
            self.advance();
        }

        match self.current {
            Some(next) if next.kind == kind => {
                self.advance();
                Some(next)
            }
            _ => None,
        }
    }

    fn eat_raw(&mut self, input: &str) -> Option<Token> {
        if self.is(input) {
            let current = self.current;
            self.advance();
            return current
        }

        None
    }

    fn raw(&mut self) -> &'a str {
        &self.source[TextRange::new(
            TextSize::from(self.last_position as u32),
            TextSize::from((self.position as u32)),
        )]
    }

    fn raw_from_token(&self, start: BytePos, token: Token) -> &'a str {
        &self.source[TextRange::new(
            TextSize::from(start.0),
            TextSize::from(start.0 + token.len as u32)
        )]
    }

    fn raw_from_range(&self, start: u32, end: u32) -> &'a str {
        &self.source[TextRange::new(
            TextSize::from(start),
            TextSize::from(end)
        )]
    }

    fn is(&mut self, input: &str) -> bool {
        if self.current.is_none() {
            self.advance()
        }

        self.raw() == input
    }

    fn is_kind(&mut self, token: SyntaxKind) -> bool {
        if self.current.is_none() {
            self.advance()
        }

        if let Some(current_token) = self.current {
            return current_token.kind == token
        }

        false
    }

    fn eof(&mut self) -> bool {
        self.is_kind(SyntaxKind::EOF) || self.current.is_none()
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

    fn semicolon(&mut self) -> ParseResult<()> {
        self.eat(SyntaxKind::SEMICOLON).map(|_| ()).ok_or(ParseErr::Semicolon(self.finish_span(self.span_start())))
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
        while !self.eof() {
            items.push(self.parse_statement_list_item(true)?);
            self.semicolon()?;
        }

        Ok(items)
    }

    fn parse_statement_list_item(&mut self, is_top: bool) -> ParseResult<ModuleItem> {
        if is_top && (self.is("import") || self.is("export")) {
            return Ok(ModuleItem::ModuleDecl(self.parse_import_export()?))
        }

        Ok(ModuleItem::Stmt(self.parse_stmt_list_item()?))
    }

    fn parse_import_export(&mut self) -> ParseResult<ModuleDecl> {
        let start = self.span_start();

        if self.eat_raw("import").is_some() {
            return Ok(self.parse_import_declaration()?.into());
        }

        if self.eat_raw("export").is_some() {
            return Ok(self.parse_export_declaration()?.into());
        }

        Err(ParseErr::UnexpectedParserState(self.finish_span(start)))
    }

    fn parse_import_declaration(&mut self) -> ParseResult<ImportDecl> {
        let start = self.span_start();
        let type_only = self.eat_raw("type").is_some();

        let specifiers: Vec<ImportSpecifier> = if self.eat_raw("*").is_some() {
            self.eat_raw("as").ok_or(ParseErr::UnexpectedParserState(self.finish_span(start)))?;
            let import_binding = ImportStarAsSpecifier {
                span: self.finish_span(start),
                local: self.parse_ident()?,
            };

            vec![import_binding.into()]
        } else if self.eat_raw("{").is_some() {
            vec![]
        } else {
            vec![]
        };

        let from_start = self.span_start();

        self.eat_raw("from").ok_or(ParseErr::UnexpectedParserState(self.finish_span(from_start)))?;

        let src = self.parse_str()?;

        Ok(ImportDecl {
            span: self.finish_span(start),
            specifiers,
            src,
            type_only,
            asserts: None,
        })
    }

    fn parse_export_declaration(&mut self) -> ParseResult<ExportDecl> {
        todo!()
    }

    fn parse_stmt_list_item(&mut self) -> ParseResult<Stmt> {
        todo!()
    }

    pub(crate) fn parse_ident(&mut self) -> ParseResult<Ident> {
        let start = self.span_start();
        let ident_name = self.eat(SyntaxKind::IDENT).map(|tok| self.raw_from_token(start, tok)).ok_or(ParseErr::CatchAll)?;

        Ok(Ident {
            span: self.finish_span(start),
            sym: ident_name.into(),
            optional: false
        })
    }

    pub(crate) fn parse_str(&mut self) -> ParseResult<Str> {
        let start = self.span_start();
        let token = self.eat(SyntaxKind::STRING).ok_or(ParseErr::UnexpectedParserState(self.finish_span(start)))?;

        Ok(Str {
            span: self.finish_span(start),
            value: self.raw_from_range(start.0+1, start.0+(token.len as u32)-1).into(),
            raw: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use swc_common::DUMMY_SP;
    use swc_petal_ast::{Ident, ImportDecl, ImportStarAsSpecifier, Module, Program};
    use swc_petal_ast::ImportSpecifier::Namespace;
    use swc_petal_ast::ModuleDecl::Import;
    use swc_petal_ast::ModuleItem::ModuleDecl;
    use crate::*;
    use swc_petal_ecma_visit::assert_eq_ignore_span;
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

    #[test]
    fn parse_namespace_import() {
        let source = "import * as Module from 'bazinga';";
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module{
            span: DUMMY_SP,
            body: vec![
                ModuleDecl(Import(ImportDecl {
                    span: DUMMY_SP,
                    specifiers: vec![
                        Namespace(ImportStarAsSpecifier {
                            span: DUMMY_SP,
                            local: Ident::new("Module".into(), DUMMY_SP),
                        })
                    ],
                    src: "bazinga".into(),
                    type_only: false,
                    asserts: None,
                }))
            ],
            shebang: None,
        });


        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_namespace_import_multi_line() {
        let source = r#"
            import * as Module from 'bazinga';
            import * as Another from 'std';
        "#;
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module{
            span: DUMMY_SP,
            body: vec![
                ModuleDecl(Import(ImportDecl {
                    span: DUMMY_SP,
                    specifiers: vec![
                        Namespace(ImportStarAsSpecifier {
                            span: DUMMY_SP,
                            local: Ident::new("Module".into(), DUMMY_SP),
                        })
                    ],
                    src: "bazinga".into(),
                    type_only: false,
                    asserts: None,
                })),
                ModuleDecl(Import(ImportDecl {
                    span: DUMMY_SP,
                    specifiers: vec![
                        Namespace(ImportStarAsSpecifier {
                            span: DUMMY_SP,
                            local: Ident::new("Another".into(), DUMMY_SP),
                        })
                    ],
                    src: "std".into(),
                    type_only: false,
                    asserts: None,
                }))
            ],
            shebang: None,
        });


        assert_eq_ignore_span!(expectation, result);
    }
}