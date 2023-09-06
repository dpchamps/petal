use crate::parser::{ParseResult, Parser};
use rslint_lexer::SyntaxKind;
use swc_petal_ast::*;

impl<'a> Parser<'a> {
    pub(super) fn parse_type_decl(&mut self) -> ParseResult<EsTypeAliasDecl> {
        let start = self.span_start();
        self.expect_raw("type")?;
        let ident = self.parse_ident()?;
        let type_params = if self.is_kind(SyntaxKind::L_ANGLE) {
            Some(self.parse_type_params()?)
        } else {
            None
        };

        self.expect(SyntaxKind::EQ)?;

        let type_ann = Box::new(self.parse_type()?);

        Ok(EsTypeAliasDecl {
            span: self.finish_span(start),
            ident,
            type_params,
            type_ann,
        })
    }

    fn parse_type_angle_bracketed_list(&mut self) -> ParseResult<EsAngleBracketedType> {
        unimplemented!()
    }

    fn parse_type(&mut self) -> ParseResult<EsType> {
        Ok(self.parse_type_ref()?.into())
    }

    fn parse_conditional_type(&mut self) -> ParseResult<EsConditionalType> {
        todo!()
    }

    fn parse_non_conditional_type<T>(&mut self) -> ParseResult<T> {
        todo!()
    }

    fn parse_union_type(&mut self) -> ParseResult<EsUnionType> {
        todo!()
    }

    fn parse_intersection_type(&mut self) -> ParseResult<EsIntersectionType> {
        todo!()
    }

    fn parse_type_op_type(&mut self) -> ParseResult<EsTypeOperatorType> {
        todo!()
    }

    fn parse_primary_type(&mut self) -> ParseResult<Box<EsType>> {
        todo!()
    }

    fn parse_type_ref(&mut self) -> ParseResult<EsTypeRef> {
        let start = self.span_start();
        let type_name = self.parse_type_name()?;
        let type_arguments =  if self.is_kind(SyntaxKind::L_ANGLE) {
            Some(self.parse_type_arguments()?)
        } else {
            None
        };

        Ok(EsTypeRef {
            span: self.finish_span(start),
            type_name,
            type_arguments,
        })
    }

    fn parse_type_name(&mut self) -> ParseResult<EsEntityName> {
        let mut entity = EsEntityName::Ident(self.parse_ident()?);

        while self.eat(SyntaxKind::DOT).is_some() {
            let ident = self.parse_ident()?;
            entity = EsEntityName::EsQualifiedName(Box::new(EsQualifiedName {
                left: entity,
                right: ident
            }));
        }

       Ok(entity)
    }

    fn parse_array_type(&mut self) -> ParseResult<EsArrayType> {
        todo!()
    }

    fn parse_literal_type(&mut self) -> ParseResult<EsLiteralType> {
        todo!()
    }

    fn parse_template_literal_type(&mut self) -> ParseResult<EsTemplateBracketedType> {
        todo!()
    }

    fn parse_type_query(&mut self) -> ParseResult<EsTypeQuery> {
        todo!()
    }

    fn parse_type_query_expr(&mut self) -> ParseResult<EsTypeQueryExpr> {
        todo!()
    }

    fn parse_import_type(&mut self) -> ParseResult<EsImportType> {
        todo!()
    }

    fn parse_type_predicate(&mut self) -> ParseResult<EsTypePredicate> {
        todo!()
    }

    fn parse_function_type(&mut self) -> ParseResult<EsFunctionType> {
        let start = self.span_start();
        let type_params = if self.is_kind(SyntaxKind::L_ANGLE) {
            Some(self.parse_type_params()?)
        } else {
            None
        };

        let mut params = vec![];

        while !self.is_kind(SyntaxKind::R_PAREN) {
            params.push(Box::new(self.parse_type()?));

            self.finish_trailing_comma(SyntaxKind::R_PAREN)?;
        }

        self.expect(SyntaxKind::FAT_ARROW)?;

        let return_type = Box::new(self.parse_type()?);

        
        Ok(EsFunctionType {
            span: self.finish_span(start),
            type_params,
            params,
            return_type
        })
    }

    pub(crate) fn parse_type_annotation(&mut self) -> ParseResult<EsTypeAnn> {
        let start = self.span_start();
        self.expect(SyntaxKind::COLON)?;
        let type_ann_type = self.parse_type()?;
        Ok(EsTypeAnn {
            span: self.finish_span(start),
            type_ann: Box::new(type_ann_type),
        })
    }

    fn parse_index_signature(&mut self) -> ParseResult<EsIndexSignature> {
        let start = self.span_start();
        self.expect(SyntaxKind::L_BRACK)?;
        let binding_id = self.parse_ident()?;
        let type_ann = if self.is_kind(SyntaxKind::COLON) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        self.expect(SyntaxKind::R_BRACK)?;

        return Ok(EsIndexSignature {
            binding_id: EsBindingIdent {
                span: binding_id.span,
                id: binding_id,
                type_ann,
            },
            type_ann: None,
            readonly: false,
            is_static: false,
            span: self.finish_span(start),
        })
    }

    fn parse_type_arguments(&mut self) -> ParseResult<EsTypeArguments> {
        let start = self.span_start();
        self.expect(SyntaxKind::L_ANGLE)?;

        let mut params = vec![];

        while !self.is_kind(SyntaxKind::R_ANGLE) {
            params.push(Box::new(self.parse_type()?));

            self.finish_trailing_comma(SyntaxKind::R_ANGLE)?;
        }

        self.expect(SyntaxKind::R_ANGLE)?;

        Ok(EsTypeArguments {
            span: self.finish_span(start),
            params,
        })
    }

    fn parse_type_params(&mut self) -> ParseResult<EsTypeParameters> {
        let start = self.span_start();
        self.expect(SyntaxKind::L_ANGLE)?;
        let mut params = vec![];
        while !self.is_kind(SyntaxKind::R_ANGLE) {
            params.push(self.parse_type_param()?);

            self.finish_trailing_comma(SyntaxKind::R_ANGLE)?;
        }

        self.expect(SyntaxKind::R_ANGLE)?;

        Ok(EsTypeParameters {
            span: self.finish_span(start),
            params,
        })
    }

    fn parse_type_param(&mut self) -> ParseResult<EsTypeParamDecl> {
        let start = self.span_start();
        let base_type = self.parse_ident()?;
        if self.eat_raw("extends").is_some() {
            return Ok(EsTypeParamDecl::HeritageTypeConstraint(
                EsHeritageTypeConstraint {
                    span: self.finish_span(start),
                    base_type,
                    constraint: Box::new(self.parse_type()?),
                },
            ));
        }

        return Ok(EsTypeParamDecl::Ident(base_type));
    }

    /*
    Note to self:

    There are several productions omitted various *Token productions.
    These are inherited from the types as comments spec, and
    are more than likely not necessary.

    The refinement type syntax is still in flight, which will subsume
    the `CurlyBracketedType` production. The rest will stay in the grammar
    but remain unparsed until it's certain that they should be removed
    */
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use swc_common::DUMMY_SP;
    use swc_petal_ast::{EsEntityName, EsFunctionType, EsHeritageTypeConstraint, EsType, EsTypeParamDecl, EsTypeParameters, EsTypeRef, Ident};
    use swc_petal_ast::EsType::EsTypeReference;
    use swc_petal_ecma_visit::assert_eq_ignore_span;

    fn get_partial_parser(source: &str) -> Parser {
        let mut parser = Parser::new(source);
        parser.advance();

        parser
    }

    #[test]
    fn parse_type_empty_type_params() {
        let source = "<>";
        let mut parser = get_partial_parser(source);

        let expectation = EsTypeParameters {
            span: DUMMY_SP,
            params: vec![],
        };
        let result = parser
            .parse_type_params()
            .expect("Failed to parse type params");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_single_type_params() {
        let source = "<T>";
        let mut parser = get_partial_parser(source);

        let expectation = EsTypeParameters {
            span: DUMMY_SP,
            params: vec![EsTypeParamDecl::Ident(Ident {
                span: DUMMY_SP,
                sym: "T".into(),
                optional: false,
            })],
        };
        let result = parser
            .parse_type_params()
            .expect("Failed to parse type params");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_multiple_type_params() {
        let source = "<T, U>";
        let mut parser = get_partial_parser(source);

        let expectation = EsTypeParameters {
            span: DUMMY_SP,
            params: vec![
                EsTypeParamDecl::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "T".into(),
                    optional: false,
                }),
                EsTypeParamDecl::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "U".into(),
                    optional: false,
                }),
            ],
        };
        let result = parser
            .parse_type_params()
            .expect("Failed to parse type params");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_type_params_trailing_comma() {
        let source = "<T, U,>";
        let mut parser = get_partial_parser(source);

        let expectation = EsTypeParameters {
            span: DUMMY_SP,
            params: vec![
                EsTypeParamDecl::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "T".into(),
                    optional: false,
                }),
                EsTypeParamDecl::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "U".into(),
                    optional: false,
                }),
            ],
        };
        let result = parser
            .parse_type_params()
            .expect("Failed to parse type params");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_type_params_heritage() {
        let source = "<T extends U>";
        let mut parser = get_partial_parser(source);

        let expectation = EsTypeParameters {
            span: DUMMY_SP,
            params: vec![
                EsTypeParamDecl::HeritageTypeConstraint(EsHeritageTypeConstraint {
                    span: DUMMY_SP,
                    base_type: Ident {
                        span: DUMMY_SP,
                        sym: "T".into(),
                        optional: false,
                    },
                    constraint: Box::new(EsTypeReference(EsTypeRef {
                        span: DUMMY_SP,
                        type_name: EsEntityName::Ident(Ident {
                            span: DUMMY_SP,
                            sym: "U".into(),
                            optional: false
                        }),
                        type_arguments: None,
                    })),
                })
            ],
        };
        let result = parser
            .parse_type_params()
            .expect("Failed to parse type params");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_function_type(){
        let source = "() => X";

        let mut parser = get_partial_parser(source);

        let expectation = EsFunctionType {
            span: DUMMY_SP,
            type_params: None,
            params: vec![],
            return_type: Box::new(EsType::EsTypeReference(EsTypeRef {
                span: DUMMY_SP,
                type_name: EsEntityName::Ident(Ident{
                    span: DUMMY_SP,
                    sym: "X".into(),
                    optional: false,
                }),
                type_arguments: None,
            })),
        };
    }
}
