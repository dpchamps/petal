use crate::parser::{ParseErr, ParseResult, Parser};
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

    fn parse_type(&mut self) -> ParseResult<EsType> {
        self.parse_conditional_type()
    }

    fn parse_conditional_type(&mut self) -> ParseResult<EsType> {
        let start = self.span_start();

        let nc_type = self.parse_non_conditional_type()?;

        if self.eat(SyntaxKind::EXTENDS_KW).is_some() {
            let extends_type = self.parse_non_conditional_type()?;
            self.expect(SyntaxKind::QUESTION)?;
            let true_type = self.parse_type()?;
            self.expect(SyntaxKind::COLON)?;
            let false_type = self.parse_type()?;

            return Ok(EsType::EsConditionalType(EsConditionalType {
                span: self.finish_span(start),
                check_type: Box::new(nc_type),
                extends_type: Box::new(extends_type),
                true_type: Box::new(true_type),
                false_type: Box::new(false_type),
            }))
        }

        Ok(nc_type)
    }

    fn parse_non_conditional_type(&mut self) -> ParseResult<EsType> {
        if self.is_kind(SyntaxKind::L_PAREN) || self.is_kind(SyntaxKind::L_ANGLE) {
            return Ok(EsType::EsFunctionType(self.parse_function_type()?));
        }

        self.parse_union_type()
    }

    fn parse_union_type(&mut self) -> ParseResult<EsType> {
        let mut start = self.span_start();

        self.eat(SyntaxKind::PIPE);

        let t = self.parse_intersection_type()?;

        if !self.is_kind(SyntaxKind::PIPE) {
            return Ok(t)
        }

        let mut union_type = EsUnionType {
            span: self.finish_span(start),
            types: vec![Box::new(t)],
        };

        while self.eat(SyntaxKind::PIPE).is_some() {
            union_type.types.push(Box::new(self.parse_intersection_type()?))
        }

        Ok(union_type.into())
    }

    fn parse_intersection_type(&mut self) -> ParseResult<EsType> {
        let mut start = self.span_start();

        self.eat(SyntaxKind::AMP);

        let t = self.parse_type_op_type()?;

        if !self.is_kind(SyntaxKind::AMP) {
            return Ok(t)
        }

        let mut intersection_type = EsIntersectionType {
            span: self.finish_span(start),
            types: vec![Box::new(t)],
        };

        while self.eat(SyntaxKind::AMP).is_some() {
            intersection_type.types.push(Box::new(self.parse_type_op_type()?))
        }

        Ok(intersection_type.into())
    }

    fn parse_type_op_type(&mut self) -> ParseResult<EsType> {
        let start = self.span_start();

        if self.eat_raw("readonly").is_some() {
            Ok(EsTypeOperatorType {
                span: self.finish_span(start),
                op: EsTypeOperatorOp::ReadOnly,
                es_type: Box::new(self.parse_type_op_type()?),
            }
            .into())
        } else if self.eat_raw("keyof").is_some() {
            Ok(EsTypeOperatorType {
                span: self.finish_span(start),
                op: EsTypeOperatorOp::ReadOnly,
                es_type: Box::new(self.parse_type_op_type()?),
            }
            .into())
        } else if self.eat_raw("unique").is_some() {
            Ok(EsTypeOperatorType {
                span: self.finish_span(start),
                op: EsTypeOperatorOp::Unique,
                es_type: Box::new(self.parse_type_op_type()?),
            }
            .into())
        } else if self.eat_raw("infer").is_some() {
            Ok(EsTypeOperatorType {
                span: self.finish_span(start),
                op: EsTypeOperatorOp::Infer,
                es_type: Box::new(self.parse_type_op_type()?),
            }
            .into())
        } else if self.eat_raw("not").is_some() {
            Ok(EsTypeOperatorType {
                span: self.finish_span(start),
                op: EsTypeOperatorOp::Not,
                es_type: Box::new(self.parse_type_op_type()?),
            }
            .into())
        } else {
            self.parse_array_type()
        }
    }

    /// Note: this somewhat diverges from how the grammar is written
    /// keep an eye on the es types as comments repo. when they release the
    /// next iteration of the grammar, check to see if this is different.
    ///
    /// Currently, the productions are as follows:
    ///
    /// PrimaryType:
    ///     ArrayType
    ///
    /// ArrayType:
    ///     PrimaryType []
    ///
    /// But it should be something closer to
    ///
    /// ArrayType:
    ///     PrimaryType
    ///     PrimaryType IndexedArrayType
    ///
    /// IndexedArrayType:
    ///     [Type <sub>opt</sub>]
    ///     [Type <sub>opt</sub>] IndexedArrayType

    fn parse_array_type(&mut self) -> ParseResult<EsType> {
        let start = self.span_start();
        let mut primary_type = self.parse_primary_type()?;

        while self.eat(SyntaxKind::L_BRACK).is_some() {
            primary_type = EsType::EsArrayType(EsArrayType {
                span: self.finish_span(start),
                elem_type: Box::new(primary_type),
            });
            self.expect(SyntaxKind::R_BRACK)?;
        }

        Ok(primary_type)
    }

    fn parse_primary_type(&mut self) -> ParseResult<EsType> {
        if self.is_kind(SyntaxKind::L_BRACK) {
            return Ok(EsType::EsTupleType(self.parse_tuple_type()?));
        } else if self.is_kind(SyntaxKind::L_CURLY) {
            todo!("Parse refinement type or object")
        } else if self.is_kind(SyntaxKind::TYPEOF_KW) {
            return Ok(EsType::EsTypeQuery(self.parse_type_query()?));
        } else if self.is_kind(SyntaxKind::IMPORT_KW) {
            return Ok(EsType::EsImportType(self.parse_import_type()?));
        } else if self.is_kind(SyntaxKind::VOID_KW) {
            let start = self.span_start();
            self.expect(SyntaxKind::VOID_KW)?;
            return Ok(EsType::EsVoidType(EsVoidType {
                span: self.finish_span(start),
            }));
        }

        self.resolve_ambiguous_primary_type()

        // // Account for array type
        // if self.eat(SyntaxKind::L_BRACK).is_some(){
        //     self.expect(SyntaxKind::R_BRACK)?;
        //     return Ok(EsType::EsArrayType(EsArrayType{
        //         span: self.finish_span(start),
        //         elem_type: Box::new(p_type),
        //     }))
        // }
        //
        // return Ok(p_type)
    }

    fn resolve_ambiguous_primary_type(&mut self) -> ParseResult<EsType> {
        // Resolving some ambiguous productions without any lookahead
        // if we haven't found a primary type, then it's either
        // this type, type ref type, type predicate type, or literal type

        // if asserts is next, we know it's a type predicate
        if self.is("asserts") {
            // type predicate
            Ok(EsType::EsTypePredicate(self.parse_type_predicate()?))
        } else if self.is_kind(SyntaxKind::THIS_KW) || self.is_kind(SyntaxKind::IDENT) {
            self.resolve_ambiguous_type_predicate_type_ref_this_type()
        } else {
            // if the above isn't true, then we have a literal type
            // literal type
            Ok(EsType::EsLiteralType(self.parse_literal_type()?))
        }
    }

    fn resolve_ambiguous_type_predicate_type_ref_this_type(&mut self) -> ParseResult<EsType> {
        let start = self.span_start();
        let this_or_ident_type = self.parse_ident_or_this()?;

        // we still need to determine whether or not we're in a
        // type predicate. If `is` is present after a this or ident, then
        // we know this is a type predicate
        if self.eat_raw("is").is_some() {
            // type predicate
            Ok(EsType::EsTypePredicate(EsTypePredicate {
                span: self.finish_span(start),
                asserts: false,
                param_name: this_or_ident_type,
                type_ann: Some(Box::new(self.parse_type()?)),
            }))
        } else {
            // Otherwise, we need to now unwrap the previous ast node into the corresponding type
            // todo: could also implement this as an into, but seems like overkill for a one off
            match this_or_ident_type {
                // this
                EsThisTypeOrIdent::EsThisType(this_type) => Ok(EsType::EsThisType(this_type)),
                // typeref
                EsThisTypeOrIdent::Ident(id) => {
                    Ok(EsType::EsTypeReference(self.parse_type_ref_from_ident(id)?))
                }
            }
        }
    }

    fn parse_type_ref(&mut self) -> ParseResult<EsTypeRef> {
        let start = self.span_start();
        let type_name = self.parse_type_name()?;
        let type_arguments = if self.is_kind(SyntaxKind::L_ANGLE) {
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
                right: ident,
            }));
        }

        Ok(entity)
    }

    fn parse_type_ref_from_ident(&mut self, ident: Ident) -> ParseResult<EsTypeRef> {
        let start_span = ident.span;
        let type_name = self.parse_type_name_from_ident(ident)?;
        let type_arguments = if self.is_kind(SyntaxKind::L_ANGLE) {
            Some(self.parse_type_arguments()?)
        } else {
            None
        };

        Ok(EsTypeRef {
            span: start_span,
            type_name,
            type_arguments,
        })
    }

    fn parse_type_name_from_ident(&mut self, ident: Ident) -> ParseResult<EsEntityName> {
        let mut entity = EsEntityName::Ident(ident);

        while self.eat(SyntaxKind::DOT).is_some() {
            let ident = self.parse_ident()?;
            entity = EsEntityName::EsQualifiedName(Box::new(EsQualifiedName {
                left: entity,
                right: ident,
            }));
        }

        Ok(entity)
    }

    fn parse_tuple_type(&mut self) -> ParseResult<EsTupleType> {
        let start = self.span_start();

        self.expect(SyntaxKind::L_BRACK)?;

        let mut elem_types = vec![];

        while !self.is_kind(SyntaxKind::R_BRACK) {
            if self.is_kind(SyntaxKind::DOT2) {
                elem_types.push(Box::new(EsType::EsRestType(self.parse_type_rest_type()?)));
            } else {
                elem_types.push(Box::new(self.parse_type()?))
            }
            self.finish_trailing_comma(SyntaxKind::R_BRACK)?;
        }

        self.expect(SyntaxKind::R_BRACK)?;

        Ok(EsTupleType {
            span: self.finish_span(start),
            elem_types,
        })
    }

    fn parse_literal_type(&mut self) -> ParseResult<EsLiteralType> {
        if self.is_kind(SyntaxKind::NUMBER) {
            return Ok(EsLiteralType::Number(self.parse_number()?));
        } else if self.is_kind(SyntaxKind::STRING) {
            return Ok(EsLiteralType::Str(self.parse_str()?));
        } else if self.is_kind(SyntaxKind::BACKTICK) {
            return Ok(EsLiteralType::Template(self.parse_template_literal_type()?));
        } else if self.is_kind(SyntaxKind::TRUE_KW) || self.is_kind(SyntaxKind::FALSE_KW) {
            return Ok(EsLiteralType::Bool(self.parse_bool()?));
        }

        Err(ParseErr::UnexpectedParserState(
            self.span(),
            "Failed to parse literal type, received unexpected value".into(),
        ))
    }

    fn parse_template_literal_type(&mut self) -> ParseResult<EsTemplateBracketedType> {
        let start = self.span_start();

        self.expect(SyntaxKind::BACKTICK)?;

        let mut types = vec![];
        let mut quasis = vec![];

        let quasi_span = self.span_start();
        let mut raw_tmpl = String::new();

        while !self.is_kind(SyntaxKind::BACKTICK) {
            if self.eat(SyntaxKind::DOLLARCURLY).is_some() {
                // push current el
                quasis.push(TplElement {
                    span: self.finish_span(quasi_span),
                    tail: false,
                    cooked: None,
                    raw: raw_tmpl.clone().into(),
                });

                // parse tmpl type
                types.push(Box::new(self.parse_type()?));
                // finish type
                self.expect(SyntaxKind::R_CURLY)?;
                raw_tmpl.clear();
            } else {
                let raw_start = self.span_start();
                let template_chunk = self.expect(SyntaxKind::TEMPLATE_CHUNK)?;
                let raw_chunk = self.raw_from_token(raw_start, template_chunk);

                raw_tmpl.push_str(raw_chunk);
            }
        }

        quasis.push(TplElement {
            span: self.finish_span(quasi_span),
            tail: !quasis.is_empty(),
            cooked: None,
            raw: raw_tmpl.clone().into(),
        });

        self.expect(SyntaxKind::BACKTICK)?;

        Ok(EsTemplateBracketedType {
            span: self.finish_span(start),
            types,
            quasis,
        })
    }

    fn parse_type_query(&mut self) -> ParseResult<EsTypeQuery> {
        let start = self.span_start();

        self.expect(SyntaxKind::TYPEOF_KW)?;

        let expr_name = self.parse_type_query_expr()?;

        let type_args = if self.is_kind(SyntaxKind::L_ANGLE) {
            Some(self.parse_type_arguments()?)
        } else {
            None
        };

        Ok(EsTypeQuery {
            span: self.finish_span(start),
            expr_name,
            type_args,
        })
    }

    fn parse_type_query_expr(&mut self) -> ParseResult<EsTypeQueryExpr> {
        if self.is_kind(SyntaxKind::IMPORT_KW) {
            return Ok(EsTypeQueryExpr::Import(self.parse_import_type()?));
        }

        Ok(EsTypeQueryExpr::EsEntityName(self.parse_type_name()?))
    }

    fn parse_import_type(&mut self) -> ParseResult<EsImportType> {
        let start = self.span_start();

        self.expect(SyntaxKind::IMPORT_KW)?;
        self.expect(SyntaxKind::L_PAREN)?;
        let module_specifier = self.parse_str()?;
        self.expect(SyntaxKind::R_PAREN)?;
        let qualifier = if self.eat(SyntaxKind::DOT).is_some() {
            Some(self.parse_type_name()?)
        } else {
            None
        };

        Ok(EsImportType {
            span: self.finish_span(start),
            arg: module_specifier,
            qualifier,
            type_args: None,
        })
    }

    fn parse_type_predicate(&mut self) -> ParseResult<EsTypePredicate> {
        let start = self.span_start();
        let asserts = self.eat_raw("asserts").is_some();
        let param_name = self.parse_ident_or_this()?;

        // Type annotation is required when the predicate is not an assertion
        let type_ann = if !asserts || self.is("is") {
            self.expect_raw("is")?;
            Some(Box::new(self.parse_type()?))
        } else {
            None
        };

        Ok(EsTypePredicate {
            span: self.finish_span(start),
            asserts,
            param_name,
            type_ann,
        })
    }

    fn parse_ident_or_this(&mut self) -> ParseResult<EsThisTypeOrIdent> {
        let start = self.span_start();

        if self.eat(SyntaxKind::THIS_KW).is_some() {
            return Ok(EsThisTypeOrIdent::EsThisType(EsThisType {
                span: self.finish_span(start),
            }));
        }

        Ok(EsThisTypeOrIdent::Ident(self.parse_ident()?))
    }

    fn parse_function_type(&mut self) -> ParseResult<EsFunctionType> {
        let start = self.span_start();
        let type_params = if self.is_kind(SyntaxKind::L_ANGLE) {
            Some(self.parse_type_params()?)
        } else {
            None
        };

        let mut params = vec![];
        self.expect(SyntaxKind::L_PAREN)?;

        while !self.is_kind(SyntaxKind::R_PAREN) {
            params.push(Box::new(self.parse_type()?));

            self.finish_trailing_comma(SyntaxKind::R_PAREN)?;
        }

        self.expect(SyntaxKind::R_PAREN)?;
        self.expect(SyntaxKind::FAT_ARROW)?;

        let return_type = Box::new(self.parse_type()?);

        Ok(EsFunctionType {
            span: self.finish_span(start),
            type_params,
            params,
            return_type,
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

        Ok(EsIndexSignature {
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

        Ok(EsTypeParamDecl::Ident(base_type))
    }

    fn parse_type_rest_type(&mut self) -> ParseResult<EsRestType> {
        let start = self.span_start();
        self.expect(SyntaxKind::DOT2)?;
        let type_ann = Box::new(self.parse_type()?);

        Ok(EsRestType {
            span: self.finish_span(start),
            type_ann,
        })
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
    use swc_petal_ast::EsType::EsTypeReference;
    use swc_petal_ast::{Bool, EsArrayType, EsConditionalType, EsEntityName, EsFunctionType, EsHeritageTypeConstraint, EsImportType, EsIntersectionType, EsLiteralType, EsQualifiedName, EsRestType, EsTemplateBracketedType, EsThisType, EsThisTypeOrIdent, EsTupleType, EsType, EsTypeArguments, EsTypeOperatorOp, EsTypeOperatorType, EsTypeParamDecl, EsTypeParameters, EsTypePredicate, EsTypeQuery, EsTypeQueryExpr, EsTypeRef, EsUnionType, Ident, Number, Str, TplElement};
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
            params: vec![EsTypeParamDecl::HeritageTypeConstraint(
                EsHeritageTypeConstraint {
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
                            optional: false,
                        }),
                        type_arguments: None,
                    })),
                },
            )],
        };
        let result = parser
            .parse_type_params()
            .expect("Failed to parse type params");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_function_type() {
        let source = "() => X";

        let mut parser = get_partial_parser(source);

        let expectation = EsFunctionType {
            span: DUMMY_SP,
            type_params: None,
            params: vec![],
            return_type: Box::new(EsType::EsTypeReference(EsTypeRef {
                span: DUMMY_SP,
                type_name: EsEntityName::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "X".into(),
                    optional: false,
                }),
                type_arguments: None,
            })),
        };

        let result = parser
            .parse_function_type()
            .expect("Failed to parse function type");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_type_predicate_assert_no_type_ann() {
        let input = "asserts a";
        let mut parser = get_partial_parser(input);

        let expectation = EsTypePredicate {
            span: DUMMY_SP,
            asserts: true,
            param_name: EsThisTypeOrIdent::Ident(Ident {
                span: DUMMY_SP,
                sym: "a".into(),
                optional: false,
            }),
            type_ann: None,
        };

        let result = parser
            .parse_type_predicate()
            .expect("Failed to parse type_predicate");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_import_type() {
        let input = r#"import("bazinga")"#;
        let mut parser = get_partial_parser(input);

        let expectation = EsImportType {
            span: DUMMY_SP,
            arg: Str {
                span: DUMMY_SP,
                value: "bazinga".into(),
                raw: None,
            },
            qualifier: None,
            type_args: None,
        };

        let result = parser
            .parse_import_type()
            .expect("Failed to parse import type");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_import_type_qualifier() {
        let input = r#"import("bazinga").X"#;
        let mut parser = get_partial_parser(input);

        let expectation = EsImportType {
            span: DUMMY_SP,
            arg: Str {
                span: DUMMY_SP,
                value: "bazinga".into(),
                raw: None,
            },
            qualifier: Some(EsEntityName::Ident(Ident {
                span: DUMMY_SP,
                sym: "X".into(),
                optional: false,
            })),
            type_args: None,
        };

        let result = parser
            .parse_import_type()
            .expect("Failed to parse import type");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_type_query() {
        let input = "typeof X";
        let mut parser = get_partial_parser(input);

        let expectation = EsTypeQuery {
            span: DUMMY_SP,
            expr_name: EsTypeQueryExpr::EsEntityName(EsEntityName::Ident(Ident {
                span: DUMMY_SP,
                sym: "X".into(),
                optional: false,
            })),
            type_args: None,
        };

        let result = parser
            .parse_type_query()
            .expect("Failed to parse type query");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_type_query_import() {
        let input = "typeof import('module')";
        let mut parser = get_partial_parser(input);

        let expectation = EsTypeQuery {
            span: DUMMY_SP,
            expr_name: EsTypeQueryExpr::Import(EsImportType {
                span: DUMMY_SP,
                arg: Str {
                    span: DUMMY_SP,
                    value: "module".into(),
                    raw: None,
                },
                qualifier: None,
                type_args: None,
            }),
            type_args: None,
        };

        let result = parser
            .parse_type_query()
            .expect("Failed to parse type query");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_type_query_type_args() {
        let input = "typeof X<Y>";
        let mut parser = get_partial_parser(input);

        let expectation = EsTypeQuery {
            span: DUMMY_SP,
            expr_name: EsTypeQueryExpr::EsEntityName(EsEntityName::Ident(Ident {
                span: DUMMY_SP,
                sym: "X".into(),
                optional: false,
            })),
            type_args: Some(EsTypeArguments {
                span: DUMMY_SP,
                params: vec![Box::new(EsTypeReference(EsTypeRef {
                    span: DUMMY_SP,
                    type_name: EsEntityName::Ident(Ident {
                        span: DUMMY_SP,
                        sym: "Y".into(),
                        optional: false,
                    }),
                    type_arguments: None,
                }))],
            }),
        };

        let result = parser
            .parse_type_query()
            .expect("Failed to parse type query");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_template_literal_type() {
        let input = "`this is a template`";
        let mut parser = get_partial_parser(input);

        let expectation = EsTemplateBracketedType {
            span: DUMMY_SP,
            types: vec![],
            quasis: vec![TplElement {
                span: DUMMY_SP,
                tail: false,
                cooked: None,
                raw: "this is a template".into(),
            }],
        };

        let result = parser
            .parse_template_literal_type()
            .expect("failed to parse template literal type");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_template_literal_type_single_type_with_quasis() {
        let input = "`this is ${X} a template`";
        let mut parser = get_partial_parser(input);

        let expectation = EsTemplateBracketedType {
            span: DUMMY_SP,
            types: vec![Box::new(EsType::EsTypeReference(EsTypeRef {
                span: DUMMY_SP,
                type_name: EsEntityName::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "X".into(),
                    optional: false,
                }),
                type_arguments: None,
            }))],
            quasis: vec![
                TplElement {
                    span: DUMMY_SP,
                    tail: false,
                    cooked: None,
                    raw: "this is ".into(),
                },
                TplElement {
                    span: DUMMY_SP,
                    tail: true,
                    cooked: None,
                    raw: " a template".into(),
                },
            ],
        };

        let result = parser
            .parse_template_literal_type()
            .expect("failed to parse template literal type");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_template_literal_type_single_type() {
        let input = "`${X}`";
        let mut parser = get_partial_parser(input);

        let expectation = EsTemplateBracketedType {
            span: DUMMY_SP,
            types: vec![Box::new(EsType::EsTypeReference(EsTypeRef {
                span: DUMMY_SP,
                type_name: EsEntityName::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "X".into(),
                    optional: false,
                }),
                type_arguments: None,
            }))],
            quasis: vec![
                TplElement {
                    span: DUMMY_SP,
                    tail: false,
                    cooked: None,
                    raw: "".into(),
                },
                TplElement {
                    span: DUMMY_SP,
                    tail: true,
                    cooked: None,
                    raw: "".into(),
                },
            ],
        };

        let result = parser
            .parse_template_literal_type()
            .expect("failed to parse template literal type");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_tuple_type() {
        let input = "[T]";
        let mut parser = get_partial_parser(input);

        let expectation = EsTupleType {
            span: DUMMY_SP,
            elem_types: vec![Box::new(EsType::EsTypeReference(EsTypeRef {
                span: DUMMY_SP,
                type_name: EsEntityName::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "T".into(),
                    optional: false,
                }),
                type_arguments: None,
            }))],
        };

        let result = parser
            .parse_tuple_type()
            .expect("failed to parse tuple type");

        assert_eq_ignore_span!(expectation, result);
    }
    #[test]
    fn parse_type_tuple_type_rest_param() {
        let input = "[...T]";
        let mut parser = get_partial_parser(input);

        let expectation = EsTupleType {
            span: DUMMY_SP,
            elem_types: vec![Box::new(EsType::EsRestType(EsRestType {
                span: DUMMY_SP,
                type_ann: Box::new(EsType::EsTypeReference(EsTypeRef {
                    span: DUMMY_SP,
                    type_name: EsEntityName::Ident(Ident {
                        span: DUMMY_SP,
                        sym: "T".into(),
                        optional: false,
                    }),
                    type_arguments: None,
                })),
            }))],
        };

        let result = parser
            .parse_tuple_type()
            .expect("failed to parse tuple type");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_primary_type_ambiguity_asserts_type_predicate() {
        let input = "asserts T is T";
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsTypePredicate(EsTypePredicate {
            span: DUMMY_SP,
            asserts: true,
            param_name: EsThisTypeOrIdent::Ident(Ident {
                span: DUMMY_SP,
                sym: "T".into(),
                optional: false,
            }),
            type_ann: Some(Box::new(EsType::EsTypeReference(EsTypeRef {
                span: DUMMY_SP,
                type_name: EsEntityName::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "T".into(),
                    optional: false,
                }),
                type_arguments: None,
            }))),
        });

        let result = parser
            .parse_primary_type()
            .expect("Failed to parse asserts type predicate from primary type");

        assert_eq_ignore_span!(expectation, result)
    }

    #[test]
    fn parse_type_primary_type_ambiguity_is_type_predicate() {
        let input = "T is T";
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsTypePredicate(EsTypePredicate {
            span: DUMMY_SP,
            asserts: false,
            param_name: EsThisTypeOrIdent::Ident(Ident {
                span: DUMMY_SP,
                sym: "T".into(),
                optional: false,
            }),
            type_ann: Some(Box::new(EsType::EsTypeReference(EsTypeRef {
                span: DUMMY_SP,
                type_name: EsEntityName::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "T".into(),
                    optional: false,
                }),
                type_arguments: None,
            }))),
        });

        let result = parser
            .parse_primary_type()
            .expect("Failed to parse asserts type predicate from primary type");

        assert_eq_ignore_span!(expectation, result)
    }

    #[test]
    fn parse_type_primary_type_ambiguity_this_is_type_predicate() {
        let input = "this is T";
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsTypePredicate(EsTypePredicate {
            span: DUMMY_SP,
            asserts: false,
            param_name: EsThisTypeOrIdent::EsThisType(EsThisType { span: DUMMY_SP }),
            type_ann: Some(Box::new(EsType::EsTypeReference(EsTypeRef {
                span: DUMMY_SP,
                type_name: EsEntityName::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "T".into(),
                    optional: false,
                }),
                type_arguments: None,
            }))),
        });

        let result = parser
            .parse_primary_type()
            .expect("Failed to parse asserts type predicate from primary type");

        assert_eq_ignore_span!(expectation, result)
    }

    #[test]
    fn parse_type_primary_type_ambiguity_this() {
        let input = "this";
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsThisType(EsThisType { span: DUMMY_SP });

        let result = parser
            .parse_primary_type()
            .expect("Failed to parse asserts type predicate from primary type");

        assert_eq_ignore_span!(expectation, result)
    }

    #[test]
    fn parse_type_primary_type_ambiguity_type_ref() {
        let input = "T.U<X, Y>";
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsTypeReference(EsTypeRef {
            span: DUMMY_SP,
            type_name: EsEntityName::EsQualifiedName(Box::new(EsQualifiedName {
                left: EsEntityName::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "T".into(),
                    optional: false,
                }),
                right: Ident {
                    span: DUMMY_SP,
                    sym: "U".into(),
                    optional: false,
                },
            })),
            type_arguments: Some(EsTypeArguments {
                span: DUMMY_SP,
                params: vec![
                    Box::new(EsType::EsTypeReference(EsTypeRef {
                        span: DUMMY_SP,
                        type_name: EsEntityName::Ident(Ident {
                            span: DUMMY_SP,
                            sym: "X".into(),
                            optional: false,
                        }),
                        type_arguments: None,
                    })),
                    Box::new(EsType::EsTypeReference(EsTypeRef {
                        span: DUMMY_SP,
                        type_name: EsEntityName::Ident(Ident {
                            span: DUMMY_SP,
                            sym: "Y".into(),
                            optional: false,
                        }),
                        type_arguments: None,
                    })),
                ],
            }),
        });

        let result = parser
            .parse_primary_type()
            .expect("Failed to parse asserts type predicate from primary type");

        assert_eq_ignore_span!(expectation, result)
    }

    #[test]
    fn parse_type_primary_type_ambiguity_literal_type_string() {
        let input = r#""string""#;
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsLiteralType(EsLiteralType::Str(Str {
            span: DUMMY_SP,
            value: "string".into(),
            raw: None,
        }));

        let result = parser
            .parse_primary_type()
            .expect("Failed to parse asserts type predicate from primary type");

        assert_eq_ignore_span!(expectation, result)
    }

    #[test]
    fn parse_type_primary_type_ambiguity_literal_type_number() {
        let input = r#"42.0"#;
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsLiteralType(EsLiteralType::Number(Number {
            span: DUMMY_SP,
            value: 42.0,
            raw: Some("42.0".into()),
        }));

        let result = parser
            .parse_primary_type()
            .expect("Failed to parse asserts type predicate from primary type");

        assert_eq_ignore_span!(expectation, result)
    }

    #[test]
    fn parse_type_primary_type_ambiguity_literal_type_bool_true() {
        let input = r#"true"#;
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsLiteralType(EsLiteralType::Bool(Bool {
            span: DUMMY_SP,
            value: true,
        }));

        let result = parser
            .parse_primary_type()
            .expect("Failed to parse asserts type predicate from primary type");

        assert_eq_ignore_span!(expectation, result)
    }

    #[test]
    fn parse_type_primary_type_ambiguity_literal_type_bool_false() {
        let input = r#"false"#;
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsLiteralType(EsLiteralType::Bool(Bool {
            span: DUMMY_SP,
            value: false,
        }));

        let result = parser
            .parse_primary_type()
            .expect("Failed to parse asserts type predicate from primary type");

        assert_eq_ignore_span!(expectation, result)
    }

    #[test]
    fn parse_type_array_type() {
        let input = "T[]";
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsArrayType(EsArrayType {
            span: DUMMY_SP,
            elem_type: Box::new(EsType::EsTypeReference(EsTypeRef {
                span: DUMMY_SP,
                type_name: EsEntityName::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "T".into(),
                    optional: false,
                }),
                type_arguments: None,
            })),
        });

        let result = parser
            .parse_array_type()
            .expect("Failed to parse array type");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_array_type_multiple() {
        let input = "T[][]";
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsArrayType(EsArrayType {
            span: DUMMY_SP,
            elem_type: Box::new(EsType::EsArrayType(EsArrayType {
                span: DUMMY_SP,
                elem_type: Box::new(EsType::EsTypeReference(EsTypeRef {
                    span: DUMMY_SP,
                    type_name: EsEntityName::Ident(Ident {
                        span: DUMMY_SP,
                        sym: "T".into(),
                        optional: false,
                    }),
                    type_arguments: None,
                })),
            })),
        });

        let result = parser
            .parse_array_type()
            .expect("Failed to parse array type");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_type_operator_type() {
        let input = "infer U";
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsTypeOperatorType(EsTypeOperatorType {
            span: DUMMY_SP,
            op: EsTypeOperatorOp::Infer,
            es_type: Box::new(EsType::EsTypeReference(EsTypeRef {
                span: DUMMY_SP,
                type_name: EsEntityName::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "U".into(),
                    optional: false,
                }),
                type_arguments: None,
            })),
        });

        let result = parser
            .parse_type_op_type()
            .expect("Failed to parse type operator type");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_type_operator_type_chained() {
        let input = "infer not U";
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsTypeOperatorType(EsTypeOperatorType {
            span: DUMMY_SP,
            op: EsTypeOperatorOp::Infer,
            es_type: Box::new(EsType::EsTypeOperatorType(EsTypeOperatorType {
                span: DUMMY_SP,
                op: EsTypeOperatorOp::Not,
                es_type: Box::new(EsType::EsTypeReference(EsTypeRef {
                    span: DUMMY_SP,
                    type_name: EsEntityName::Ident(Ident {
                        span: DUMMY_SP,
                        sym: "U".into(),
                        optional: false,
                    }),
                    type_arguments: None,
                })),
            })),
        });

        let result = parser
            .parse_type_op_type()
            .expect("Failed to parse type operator type");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_union_intersection() {
        let input = "T | U & V";
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsUnionType(EsUnionType {
            span: DUMMY_SP,
            types: vec![
                Box::new(EsType::EsTypeReference(EsTypeRef {
                    span: DUMMY_SP,
                    type_name:  EsEntityName::Ident(Ident {
                        span: DUMMY_SP,
                        sym: "T".into(),
                        optional: false,
                    }),
                    type_arguments: None,
                })),
                Box::new(EsType::EsIntersectionType(EsIntersectionType {
                    span: DUMMY_SP,
                    types: vec![
                        Box::new(EsType::EsTypeReference(EsTypeRef{
                            span: DUMMY_SP,
                            type_name: EsEntityName::Ident(Ident {
                                span: DUMMY_SP,
                                sym: "U".into(),
                                optional: false,
                            }),
                            type_arguments: None,
                        })),
                        Box::new(EsType::EsTypeReference(EsTypeRef{
                            span: DUMMY_SP,
                            type_name: EsEntityName::Ident(Ident {
                                span: DUMMY_SP,
                                sym: "V".into(),
                                optional: false,
                            }),
                            type_arguments: None,
                        }))
                    ],
                }))
            ],
        });

        let result = parser
            .parse_union_type()
            .expect("Failed to parse type union and intersection");

        assert_eq_ignore_span!(expectation, result);
    }

    #[test]
    fn parse_type_conditional_type() {
        let input = "true extends false ? Record<string, unknown> : unknown";
        let mut parser = get_partial_parser(input);

        let expectation = EsType::EsConditionalType(EsConditionalType{
            span: DUMMY_SP,
            check_type: Box::new(EsType::EsLiteralType(EsLiteralType::Bool(Bool{
                span: DUMMY_SP,
                value: true,
            }))),
            extends_type: Box::new(EsType::EsLiteralType(EsLiteralType::Bool(Bool{
                span: DUMMY_SP,
                value: false,
            }))),
            true_type: Box::new(EsType::EsTypeReference(EsTypeRef{
                span: DUMMY_SP,
                type_name: EsEntityName::Ident(Ident{
                    span: DUMMY_SP,
                    sym: "Record".into(),
                    optional: false,
                }),
                type_arguments: Some(EsTypeArguments{
                    span: DUMMY_SP,
                    params: vec![
                        Box::new(EsType::EsTypeReference(EsTypeRef{
                            span: DUMMY_SP,
                            type_name: EsEntityName::Ident(Ident{
                                span: DUMMY_SP,
                                sym: "string".into(),
                                optional: false,
                            }),
                            type_arguments: None,
                        })),
                        Box::new(EsType::EsTypeReference(EsTypeRef{
                            span: DUMMY_SP,
                            type_name: EsEntityName::Ident(Ident{
                                span: DUMMY_SP,
                                sym: "unknown".into(),
                                optional: false,
                            }),
                            type_arguments: None,
                        }))
                    ],
                }),
            })),
            false_type: Box::new(EsType::EsTypeReference(EsTypeRef{
                span: DUMMY_SP,
                type_name: EsEntityName::Ident(Ident{
                    span: DUMMY_SP,
                    sym: "unknown".into(),
                    optional: false,
                }),
                type_arguments: None,
            })),
        });

        let result = parser.parse_conditional_type().expect("failed to parse conditional type");

        assert_eq_ignore_span!(expectation, result);
    }
}
