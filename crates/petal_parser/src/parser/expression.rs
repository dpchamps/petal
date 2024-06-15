use crate::parser::{ParseResult, Parser};

use swc_petal_ast::*;

impl<'a> Parser<'a> {
    pub(super) fn parse_expression(&mut self) -> ParseResult<Expr> {
        todo!()
    }

    fn parse_ident_reference(&mut self) -> ParseResult<Ident> {
        let ident = self.parse_ident()?;

        // todo: static semantics

        Ok(ident)
    }

    fn parse_binding_ident(&mut self) -> ParseResult<EsBindingIdent> {
        let start = self.span_start();
        let ident = self.parse_ident()?;

        // todo: static semantics

        Ok(EsBindingIdent {
            span: self.finish_span(start),
            id: ident,
            type_ann: None,
        })
    }

    fn parse_label_identifier(&mut self) -> ParseResult<Ident> {
        let ident = self.parse_ident()?;

        // todo: static semantics

        Ok(ident)
    }

    fn parse_primary_expression(&mut self) -> ParseResult<Expr> {
        todo!()
    }

    fn parse_cover_parenthesized_expression_and_array_parameter_list() -> ParseResult<Function>{
        unimplemented!()
    }
}
