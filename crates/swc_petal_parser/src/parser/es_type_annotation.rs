// use std::fmt::Write;

// use either::Either;
// use swc_atoms::js_word;
// use swc_common::{Spanned, SyntaxContext};

use super::*;
// use crate::{lexer::TokenContexts, parser::class_and_fn::IsSimpleParameterList, token::Keyword};

enum _TokenBodyType {
    Paren,
    Square,
    Curly,
    Angle
}

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

    pub fn is_bracket_body_start(&mut self) -> PResult<bool> {
        debug_assert!(self.input.syntax().typescript());

        Ok(is_one_of!(self, '(', '[', '{', '<'))
    }

    pub fn is_bracket_body_terminator(&mut self) -> PResult<bool> {
        debug_assert!(self.input.syntax().typescript());

        Ok(is_one_of!(self, ')', ']', '}', '>'))
    }

    pub fn parse_es_token_body_el(&mut self) -> PResult<TokenOrBracketedTokens> {

        let result = match self.is_bracket_body_start()? {
            true => TokenOrBracketedTokens::BracketBody(self.parse_es_bracket_body()?),
            false => {
                let t = bump!(self);
                TokenOrBracketedTokens::Token(EsToken{
                    span: span!(self, cur_pos!(self)),
                    value: format!("{:?}", t)
                })
            }
        };

        Ok(result)
    }

    pub fn parse_es_token_body(&mut self) -> PResult<Vec<TokenOrBracketedTokens>> {
        let mut buffer = vec![];

        loop {
            trace_cur!(self, parse_es_token_body__element);

            if(self.is_bracket_body_terminator())? {
                break;
            }

            let next = self.parse_es_token_body_el()?;
            buffer.push(next);

        }

        Ok(buffer)
    }

    pub fn parse_es_bracket_body(&mut self) -> PResult<EsBracketBody> {
        let start = cur_pos!(self);

        let token_body = self.parse_es_token_body()?;

        Ok(EsBracketBody {
            span: span!(self, start),
            token_body
        })
    }
}