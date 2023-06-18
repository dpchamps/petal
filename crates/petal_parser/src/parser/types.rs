use crate::parser::{Parser, ParseResult};
use swc_petal_ast::*;


impl<'a> Parser<'a> {
    pub(super) fn parse_type_decl(&mut self) -> ParseResult<EsTypeAliasDecl> {
        let start = self.span_start();
        self.expect_raw("type")?;
        let ident = self.parse_ident()?;
        let type_params = self.parse_type_angle_bracketed_list().ok();
        let type_ann = self.parse_type()?;

        Ok(EsTypeAliasDecl {
            span: self.finish_span(start),
            ident,
            type_params,
            type_ann
        })
    }

    fn parse_type_angle_bracketed_list(&mut self) -> ParseResult<EsAngleBracketedType>{
        unimplemented!()
    }

    fn parse_type(&mut self) -> ParseResult<Box<EsType>> {
        unimplemented!()
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
        todo!()
    }

    fn parse_type_name(&mut self) -> ParseResult<EsEntityName> {
        todo!()
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

    fn parse_import_type(&mut self) -> ParseResult<EsImportType>{
        todo!()
    }

    fn parse_type_predicate(&mut self) -> ParseResult<EsTypePredicate> {
        todo!()
    }

    fn parse_function_type(&mut self) -> ParseResult<EsFunctionType> {
        todo!()
    }

    pub(crate) fn parse_type_annotation(&mut self) -> ParseResult<EsTypeAnn> {
        todo!()
    }

    fn parse_index_signature(&mut self) -> ParseResult<EsIndexSignature> {
        todo!()
    }

    fn parse_type_arguments(&mut self) -> ParseResult<EsTypeArguments> {
        todo!()
    }

    fn parse_type_params(&mut self) -> ParseResult<EsTypeArguments> {
        todo!()
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