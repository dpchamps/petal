// use std::fmt::Write;

// use either::Either;
// use swc_atoms::js_word;
// use swc_common::{Spanned, SyntaxContext};

use super::*;
// use crate::{lexer::TokenContexts, parser::class_and_fn::IsSimpleParameterList, token::Keyword};

impl<I: Tokens> Parser<I> {

    pub fn parse_token_body(&mut self) -> PResult<bool> {

        todo!()
    }

    pub fn is_non_bracked_token(&mut self) -> PResult<bool> {
        let is_bracket = is_one_of!(self, '(', ')', '[', ']', '{', '}', '<', '>');
        let is_tpl_el = self.ts_look_ahead(|p| Ok(p.is_tpl_el()))?;
        let is_bracketed_token = is_bracket || is_tpl_el;

        return Ok(!is_bracketed_token);
    }

    pub fn is_tpl_el(&mut self) -> bool {
        match self.parse_tpl_element(false) {
            Ok(_) => true,
            _ => false
        }
    }
}