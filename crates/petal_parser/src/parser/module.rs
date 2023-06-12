use rslint_lexer::SyntaxKind;
use crate::parser::{ParseErr, Parser, ParseResult};
use swc_petal_ast::*;

impl<'a> Parser<'a> {
    pub(super) fn parse_module(&mut self) -> Result<Module, ParseErr> {
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
            items.push(self.parse_module_list_item(true)?);
            self.semicolon()?;
        }

        Ok(items)
    }

    fn parse_module_list_item(&mut self, is_top: bool) -> ParseResult<ModuleItem> {
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
            return self.parse_export();
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

            self.finish_comma_curly()?;
            specifiers.push(import_specifier);
        }

        self.eat(SyntaxKind::R_CURLY)
            .ok_or(ParseErr::UnexpectedParserState(
                self.span(),
                "expected to find '}'".into(),
            ))?;

        Ok(specifiers)
    }

    fn parse_export(&mut self) -> ParseResult<ModuleDecl> {

        if self.is_kind(SyntaxKind::CONST_KW) || self.is("let") || self.is("type") {
            return Ok(self.parse_export_decl()?.into())
        }

        return self.parse_named_or_from_export();
    }

    fn parse_export_decl(&mut self) -> ParseResult<ExportDecl> {
        todo!()
    }

    fn parse_named_or_from_export(&mut self) -> ParseResult<ModuleDecl> {
        // we could either be entering an ExportFromClause or a NamedExport production

        if self.eat(SyntaxKind::STAR).is_some(){
            // we're officially in an export from clause.
            // for ast reasons, we trat this case like an export list
            if self.is("as") {
                return Ok(self.parse_exports_list(false, true)?.into())
            }

            let start = self.span_start();
            self.expect_raw("from")?;

            let src = self.parse_str()?;
            return Ok(ModuleDecl::ExportAll(ExportAll {
                span: self.finish_span(start),
                src,
                asserts: None,
            }))
        }

        // otherwise, we still don't know, but we've settled the ast ambiguity
        return Ok(self.parse_exports_list(true, false)?.into());
    }

    fn parse_exports_list(&mut self, needs_r_brace: bool, needs_from: bool) -> ParseResult<NamedExport> {
        let start = self.span_start();
        let mut specifiers = vec![];

        // we've already parsed a star, so let's see if its a namespace
        if self.eat_raw("as").is_some() {
            specifiers.push(ExportSpecifier::Namespace(ExportNamespaceSpecifier {
                span: self.finish_span(start),
                name: self.parse_module_export_name()?,
            }))
        } else {
            // otherwise, we are parsing named exports
            self.expect(SyntaxKind::L_CURLY)?;
            while !self.is_kind(SyntaxKind::R_CURLY) {
                let start = self.span_start();

                let orig = self.parse_module_export_name()?;
                let exported = if let Some(_) = self.eat_raw("as") {
                    Some(self.parse_module_export_name()?)
                } else {
                    None
                };

                specifiers.push(
                    ExportSpecifier::Named(ExportNamedSpecifier{
                        span: self.finish_span(start),
                        orig,
                        exported,
                        is_type_only: false,
                    })
                );

                self.finish_comma_curly()?;
            }
        }


        if needs_r_brace {
            self.expect(SyntaxKind::R_CURLY)?;
        }

        let from_result = self.eat_raw("from");
        let src = if let Some(_) = from_result {
            Some(self.parse_str()?)
        } else {
            None
        };

        if src.is_none() && needs_from {
            return Err(self.expected_state("from"));
        }

        Ok(NamedExport {
            span: self.finish_span(start),
            specifiers,
            src,
            type_only: false,
            asserts: None,
        })
    }

    fn parse_export_from(&mut self) -> ParseResult<ExportAll> {
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
}

#[cfg(test)]
mod tests {
    use swc_common::DUMMY_SP;
    use swc_petal_ast::*;

    use crate::*;
    use swc_petal_ast::ImportSpecifier::Namespace;
    use swc_petal_ast::ModuleDecl::{ExportAll as ModExportAll, ExportNamed, Import};
    use swc_petal_ast::ModuleItem::{ModuleDecl, };
    use swc_petal_ecma_visit::assert_eq_ignore_span;

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

    #[test]
    fn parse_export_all(){
        let source = r#"
            export * from "somewhere";
        "#;
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![ModuleDecl(ModExportAll(ExportAll {
                span: DUMMY_SP,
                src: "somewhere".into(),
                asserts: None,
            }))],
            shebang: None,
        });

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_export_from_namespace(){
        let source = r#"
            export * as Module from "somewhere";
        "#;
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![ModuleDecl(ExportNamed(NamedExport {
                span: DUMMY_SP,
                specifiers: vec![
                    ExportSpecifier::Namespace(ExportNamespaceSpecifier {
                        span: DUMMY_SP,
                        name: ModuleExportName::Ident(Ident::new("Module".into(), DUMMY_SP)),
                    })
                ],
                src: Some("somewhere".into()),
                type_only: false,
                asserts: None,
            }))],
            shebang: None,
        });

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_export_from_named(){
        let source = r#"
            export {Alpha, Beta as Gamma} from "some/slashed/module.ts";
        "#;
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![ModuleDecl(ExportNamed(NamedExport {
                span: DUMMY_SP,
                specifiers: vec![
                    ExportSpecifier::Named(ExportNamedSpecifier {
                        span: DUMMY_SP,
                        orig: ModuleExportName::Ident(Ident::new("Alpha".into(), DUMMY_SP)),
                        exported: None,
                        is_type_only: false,
                    }),
                    ExportSpecifier::Named(ExportNamedSpecifier {
                        span: DUMMY_SP,
                        orig: ModuleExportName::Ident(Ident::new("Beta".into(), DUMMY_SP)),
                        exported: Some(ModuleExportName::Ident(Ident::new("Gamma".into(), DUMMY_SP))),
                        is_type_only: false,
                    })
                ],
                src: Some("some/slashed/module.ts".into()),
                type_only: false,
                asserts: None,
            }))],
            shebang: None,
        });

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_export_from_empty(){
        let source = r#"
            export {} from "nowhere";
        "#;
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![ModuleDecl(ExportNamed(NamedExport {
                span: DUMMY_SP,
                specifiers: vec![],
                src: Some("nowhere".into()),
                type_only: false,
                asserts: None,
            }))],
            shebang: None,
        });

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_export_named(){
        let source = r#"
            export {Arpeggio, Dorian, Staccato as Articulation};
        "#;
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![ModuleDecl(ExportNamed(NamedExport {
                span: DUMMY_SP,
                specifiers: vec![
                    ExportSpecifier::Named(ExportNamedSpecifier {
                        span: DUMMY_SP,
                        orig: ModuleExportName::Ident(Ident::new("Arpeggio".into(), DUMMY_SP)),
                        exported: None,
                        is_type_only: false,
                    }),
                    ExportSpecifier::Named(ExportNamedSpecifier {
                        span: DUMMY_SP,
                        orig: ModuleExportName::Ident(Ident::new("Dorian".into(), DUMMY_SP)),
                        exported: None,
                        is_type_only: false,
                    }),
                    ExportSpecifier::Named(ExportNamedSpecifier {
                        span: DUMMY_SP,
                        orig: ModuleExportName::Ident(Ident::new("Staccato".into(), DUMMY_SP)),
                        exported: Some(ModuleExportName::Ident(Ident::new("Articulation".into(), DUMMY_SP))),
                        is_type_only: false,
                    })
                ],
                src: None,
                type_only: false,
                asserts: None,
            }))],
            shebang: None,
        });

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_export_named_empty(){
        let source = r#"
            export {};
        "#;
        let result = Parser::parse(source).expect("Failed to parse module");
        let expectation = Program::Module(Module {
            span: DUMMY_SP,
            body: vec![ModuleDecl(ExportNamed(NamedExport {
                span: DUMMY_SP,
                specifiers: vec![],
                src: None,
                type_only: false,
                asserts: None,
            }))],
            shebang: None,
        });

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_export_declaration(){
        let source = r#"
            export const X = "hello";
            export let SomeFn = () => {};
        "#;
        let result = Parser::parse(source).expect("Failed to parse module");
    }
}