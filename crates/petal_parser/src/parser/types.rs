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
}