use rslint_errors::Diagnostic;
use rslint_lexer::{Lexer, SyntaxKind, Token};
use rslint_rowan::{TextRange, TextSize};
use swc_common::{BytePos, Span, SyntaxContext};

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
    UnexpectedParserState(Span, String),
    MissingSemicolon(Span),
}

type ParseResult<T> = Result<T, ParseErr>;

impl<'a> Parser<'a> {
    fn advance(&mut self) {
        let mut next = None;
        for (token, error) in self.lexer.by_ref() {
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
        if self.current.is_none() {
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
            return current;
        }

        None
    }

    fn raw(&mut self) -> &'a str {
        &self.source[TextRange::new(
            TextSize::from(self.last_position as u32),
            TextSize::from(self.position as u32),
        )]
    }

    fn raw_from_token(&self, start: BytePos, token: Token) -> &'a str {
        &self.source[TextRange::new(
            TextSize::from(start.0),
            TextSize::from(start.0 + token.len as u32),
        )]
    }

    fn raw_from_range(&self, start: u32, end: u32) -> &'a str {
        &self.source[TextRange::new(TextSize::from(start), TextSize::from(end))]
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
            return current_token.kind == token;
        }

        false
    }

    fn eof(&mut self) -> bool {
        self.is_kind(SyntaxKind::EOF) || self.current.is_none()
    }

    fn span(&self) -> Span {
        Span::new(
            BytePos(self.last_position as u32),
            BytePos(self.position as u32),
            SyntaxContext::empty(),
        )
    }

    fn span_start(&self) -> BytePos {
        BytePos(self.last_position as u32)
    }

    fn finish_span(&self, span_start: BytePos) -> Span {
        Span::new(
            span_start,
            BytePos(self.position as u32),
            SyntaxContext::empty(),
        )
    }

    fn semicolon(&mut self) -> ParseResult<()> {
        self.eat(SyntaxKind::SEMICOLON)
            .map(|_| ())
            .ok_or(ParseErr::MissingSemicolon(
                self.finish_span(self.span_start()),
            ))
    }

    pub fn new(source: &'a str) -> Self {
        let lexer = Lexer::from_str(source, 0);

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

    fn parse_module_items(&mut self) -> ParseResult<Vec<ModuleItem>> {
        let mut items = vec![];
        while !self.eof() {
            items.push(self.parse_statement_list_item(true)?);
            self.semicolon()?;
        }

        Ok(items)
    }

    fn parse_statement_list_item(&mut self, is_top: bool) -> ParseResult<ModuleItem> {
        if is_top && (self.is("import") || self.is("export")) {
            return Ok(ModuleItem::ModuleDecl(self.parse_import_export()?));
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

        Err(ParseErr::UnexpectedParserState(
            self.finish_span(start),
            "Expected import/export keywords".into(),
        ))
    }

    fn parse_import_declaration(&mut self) -> ParseResult<ImportDecl> {
        let start = self.span_start();
        let type_only = self.eat_raw("type").is_some();

        let specifiers: Vec<ImportSpecifier> = if self.eat_raw("*").is_some() {
            self.eat_raw("as").ok_or(ParseErr::UnexpectedParserState(
                self.finish_span(start),
                "expected as keyword".into(),
            ))?;
            let import_binding = ImportStarAsSpecifier {
                span: self.finish_span(start),
                local: self.parse_ident()?,
            };

            vec![import_binding.into()]
        } else {
            self.parse_import_list()?
                .into_iter()
                .map(|x| x.into())
                .collect()
        };

        let from_start = self.span_start();

        self.eat_raw("from").ok_or(ParseErr::UnexpectedParserState(
            self.finish_span(from_start),
            "expected 'from' keyword".into(),
        ))?;

        let src = self.parse_str()?;

        Ok(ImportDecl {
            span: self.finish_span(start),
            specifiers,
            src,
            type_only,
            asserts: None,
        })
    }

    fn parse_import_list(&mut self) -> ParseResult<Vec<ImportNamedSpecifier>> {
        self.eat(SyntaxKind::L_CURLY)
            .ok_or(ParseErr::UnexpectedParserState(
                self.span(),
                "expected '{'".into(),
            ))?;
        let mut specifiers = vec![];

        while !self.is_kind(SyntaxKind::R_CURLY) {
            let start = self.span_start();
            let is_type_only = self.eat_raw("type").is_some();
            let module_export_or_import_binding = self.parse_module_export_name()?;
            let import_binding = if self.eat_raw("as").is_some() {
                Some(self.parse_ident()?)
            } else {
                None
            };

            let import_specifier = match import_binding {
                Some(ident) => ImportNamedSpecifier {
                    span: self.finish_span(start),
                    local: ident,
                    imported: Some(module_export_or_import_binding),
                    is_type_only,
                },
                None => ImportNamedSpecifier {
                    span: self.finish_span(start),
                    local: match module_export_or_import_binding {
                        ModuleExportName::Ident(ident) => ident,
                        _ => {
                            return Err(ParseErr::UnexpectedParserState(
                                self.finish_span(start),
                                "expected identifier, but got string".into(),
                            ))
                        }
                    },
                    imported: None,
                    is_type_only,
                },
            };

            // if the next two things aren't a comma AND the closing of the argument list, we're in an invalid state
            if self.eat(SyntaxKind::COMMA).is_none() && !self.is_kind(SyntaxKind::R_CURLY) {
                return Err(ParseErr::UnexpectedParserState(
                    self.finish_span(start),
                    "expected '}' or ','".into(),
                ));
            }

            specifiers.push(import_specifier);
        }

        self.eat(SyntaxKind::R_CURLY)
            .ok_or(ParseErr::UnexpectedParserState(
                self.span(),
                "expected to find '}'".into(),
            ))?;

        Ok(specifiers)
    }

    fn parse_export_declaration(&mut self) -> ParseResult<ExportDecl> {
        todo!()
    }

    fn parse_stmt_list_item(&mut self) -> ParseResult<Stmt> {
        todo!()
    }

    fn parse_module_export_name(&mut self) -> ParseResult<ModuleExportName> {
        if self.is_kind(SyntaxKind::STRING) {
            return Ok(ModuleExportName::Str(self.parse_str()?));
        }

        Ok(ModuleExportName::Ident(self.parse_ident()?))
    }

    pub(crate) fn parse_ident(&mut self) -> ParseResult<Ident> {
        let start = self.span_start();
        let ident_name = self
            .eat(SyntaxKind::IDENT)
            .map(|tok| self.raw_from_token(start, tok))
            .ok_or(ParseErr::CatchAll)?;

        Ok(Ident {
            span: self.finish_span(start),
            sym: ident_name.into(),
            optional: false,
        })
    }

    pub(crate) fn parse_str(&mut self) -> ParseResult<Str> {
        let start = self.span_start();
        let token = self
            .eat(SyntaxKind::STRING)
            .ok_or(ParseErr::UnexpectedParserState(
                self.finish_span(start),
                "expected to find string, this is a parser problem".into(),
            ))?;

        Ok(Str {
            span: self.finish_span(start),
            value: self
                .raw_from_range(start.0 + 1, start.0 + (token.len as u32) - 1)
                .into(),
            raw: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use swc_common::DUMMY_SP;
    use swc_petal_ast::{
        Ident, ImportDecl, ImportNamedSpecifier, ImportSpecifier, ImportStarAsSpecifier, Module,
        ModuleExportName, Program,
    };

    use crate::*;
    use swc_petal_ast::ImportSpecifier::Namespace;
    use swc_petal_ast::ModuleDecl::Import;
    use swc_petal_ast::ModuleItem::ModuleDecl;
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
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![ModuleDecl(Import(ImportDecl {
                span: DUMMY_SP,
                specifiers: vec![Namespace(ImportStarAsSpecifier {
                    span: DUMMY_SP,
                    local: Ident::new("Module".into(), DUMMY_SP),
                })],
                src: "bazinga".into(),
                type_only: false,
                asserts: None,
            }))],
            shebang: None,
        });

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_namespace_import_type() {
        let source = "import type * as Module from 'bazinga';";
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![ModuleDecl(Import(ImportDecl {
                span: DUMMY_SP,
                specifiers: vec![Namespace(ImportStarAsSpecifier {
                    span: DUMMY_SP,
                    local: Ident::new("Module".into(), DUMMY_SP),
                })],
                src: "bazinga".into(),
                type_only: true,
                asserts: None,
            }))],
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
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![
                ModuleDecl(Import(ImportDecl {
                    span: DUMMY_SP,
                    specifiers: vec![Namespace(ImportStarAsSpecifier {
                        span: DUMMY_SP,
                        local: Ident::new("Module".into(), DUMMY_SP),
                    })],
                    src: "bazinga".into(),
                    type_only: false,
                    asserts: None,
                })),
                ModuleDecl(Import(ImportDecl {
                    span: DUMMY_SP,
                    specifiers: vec![Namespace(ImportStarAsSpecifier {
                        span: DUMMY_SP,
                        local: Ident::new("Another".into(), DUMMY_SP),
                    })],
                    src: "std".into(),
                    type_only: false,
                    asserts: None,
                })),
            ],
            shebang: None,
        });

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_import_specifier_list_single() {
        let source = r#"import {a} from "b";"#;
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![ModuleDecl(Import(ImportDecl {
                span: DUMMY_SP,
                specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
                    span: DUMMY_SP,
                    local: Ident::new("a".into(), DUMMY_SP),
                    imported: None,
                    is_type_only: false,
                })],
                src: "b".into(),
                type_only: false,
                asserts: None,
            }))],
            shebang: None,
        });

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_import_specifier_list_single_trailing_comma() {
        let source = r#"import {a,} from "b";"#;
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![ModuleDecl(Import(ImportDecl {
                span: DUMMY_SP,
                specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
                    span: DUMMY_SP,
                    local: Ident::new("a".into(), DUMMY_SP),
                    imported: None,
                    is_type_only: false,
                })],
                src: "b".into(),
                type_only: false,
                asserts: None,
            }))],
            shebang: None,
        });

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_import_specifier_list_type() {
        let source = r#"import {type a} from "b";"#;
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![ModuleDecl(Import(ImportDecl {
                span: DUMMY_SP,
                specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
                    span: DUMMY_SP,
                    local: Ident::new("a".into(), DUMMY_SP),
                    imported: None,
                    is_type_only: true,
                })],
                src: "b".into(),
                type_only: false,
                asserts: None,
            }))],
            shebang: None,
        });

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_import_specifier_list_multiple() {
        let source = r#"import {foo, bazzar, grewt} from "b";"#;
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![ModuleDecl(Import(ImportDecl {
                span: DUMMY_SP,
                specifiers: vec![
                    ImportSpecifier::Named(ImportNamedSpecifier {
                        span: DUMMY_SP,
                        local: Ident::new("foo".into(), DUMMY_SP),
                        imported: None,
                        is_type_only: false,
                    }),
                    ImportSpecifier::Named(ImportNamedSpecifier {
                        span: DUMMY_SP,
                        local: Ident::new("bazzar".into(), DUMMY_SP),
                        imported: None,
                        is_type_only: false,
                    }),
                    ImportSpecifier::Named(ImportNamedSpecifier {
                        span: DUMMY_SP,
                        local: Ident::new("grewt".into(), DUMMY_SP),
                        imported: None,
                        is_type_only: false,
                    }),
                ],
                src: "b".into(),
                type_only: false,
                asserts: None,
            }))],
            shebang: None,
        });

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_import_specifier_list_alias() {
        let source = r#"import {foo as bar} from "copacabana";"#;
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![ModuleDecl(Import(ImportDecl {
                span: DUMMY_SP,
                specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
                    span: DUMMY_SP,
                    local: Ident::new("bar".into(), DUMMY_SP),
                    imported: Some(ModuleExportName::Ident(Ident::new("foo".into(), DUMMY_SP))),
                    is_type_only: false,
                })],
                src: "copacabana".into(),
                type_only: false,
                asserts: None,
            }))],
            shebang: None,
        });

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_imports_mixed() {
        let source = r#"
            import {foo, type t_foo} from "a";
            import * as module from "b";
            import type * as TypeNamespace from 'c';
            import type {type x, y} from "wacky_module";
        "#;
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![
                ModuleDecl(Import(ImportDecl {
                    span: DUMMY_SP,
                    specifiers: vec![
                        ImportSpecifier::Named(ImportNamedSpecifier {
                            span: DUMMY_SP,
                            local: Ident::new("foo".into(), DUMMY_SP),
                            imported: None,
                            is_type_only: false,
                        }),
                        ImportSpecifier::Named(ImportNamedSpecifier {
                            span: DUMMY_SP,
                            local: Ident::new("t_foo".into(), DUMMY_SP),
                            imported: None,
                            is_type_only: true,
                        }),
                    ],
                    src: "a".into(),
                    type_only: false,
                    asserts: None,
                })),
                ModuleDecl(Import(ImportDecl {
                    span: DUMMY_SP,
                    specifiers: vec![Namespace(ImportStarAsSpecifier {
                        span: DUMMY_SP,
                        local: Ident::new("module".into(), DUMMY_SP),
                    })],
                    src: "b".into(),
                    type_only: false,
                    asserts: None,
                })),
                ModuleDecl(Import(ImportDecl {
                    span: DUMMY_SP,
                    specifiers: vec![Namespace(ImportStarAsSpecifier {
                        span: DUMMY_SP,
                        local: Ident::new("TypeNamespace".into(), DUMMY_SP),
                    })],
                    src: "c".into(),
                    type_only: true,
                    asserts: None,
                })),
                ModuleDecl(Import(ImportDecl {
                    span: DUMMY_SP,
                    specifiers: vec![
                        ImportSpecifier::Named(ImportNamedSpecifier {
                            span: DUMMY_SP,
                            local: Ident::new("x".into(), DUMMY_SP),
                            imported: None,
                            is_type_only: true,
                        }),
                        ImportSpecifier::Named(ImportNamedSpecifier {
                            span: DUMMY_SP,
                            local: Ident::new("y".into(), DUMMY_SP),
                            imported: None,
                            is_type_only: false,
                        }),
                    ],
                    src: "wacky_module".into(),
                    type_only: true,
                    asserts: None,
                })),
            ],
            shebang: None,
        });

        assert_eq_ignore_span!(expectation, result);
    }
}
