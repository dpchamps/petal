use super::*;
use crate::token::BinOpToken;

#[derive(Eq, PartialEq)]
pub enum TokenBodyType {
    Paren,
    Square,
    Curly,
    Angle,
    Template,
}

impl<I: Tokens> Parser<I> {
    fn is_bracket_body_end_token(&mut self, t: &TokenBodyType) -> bool {
        match t {
            TokenBodyType::Paren => is!(self, ')'),
            TokenBodyType::Square => is!(self, ']'),
            TokenBodyType::Curly => is!(self, '}'),
            TokenBodyType::Angle => is!(self, '>'),
            TokenBodyType::Template => is!(self, '}'),
        }
    }

    fn expect_bracket_start_token(&mut self, t: &TokenBodyType) -> PResult<()> {
        match t {
            TokenBodyType::Paren => {
                expect!(self, '(');
                Ok(())
            }
            TokenBodyType::Square => {
                expect!(self, '[');
                Ok(())
            }
            TokenBodyType::Curly => {
                expect!(self, '{');
                Ok(())
            }
            TokenBodyType::Angle => {
                expect!(self, '<');
                Ok(())
            }
            TokenBodyType::Template => {
                expect!(self, '`');
                Ok(())
            }
        }
    }

    fn expect_bracket_end_token(&mut self, t: &TokenBodyType) -> PResult<()> {
        match t {
            TokenBodyType::Paren => {
                expect!(self, ')');
                Ok(())
            }
            TokenBodyType::Square => {
                expect!(self, ']');
                Ok(())
            }
            TokenBodyType::Curly => {
                expect!(self, '}');
                Ok(())
            }
            TokenBodyType::Angle => {
                expect!(self, '>');
                Ok(())
            }
            TokenBodyType::Template => {
                expect!(self, '`');
                Ok(())
            }
        }
    }

    fn get_token_body_type(&mut self) -> PResult<TokenBodyType> {
        match cur!(self, false) {
            Ok(Token::LParen) => Ok(TokenBodyType::Paren),
            Ok(Token::LBrace) => Ok(TokenBodyType::Curly),
            Ok(Token::LBracket) => Ok(TokenBodyType::Square),
            Ok(Token::BinOp(BinOpToken::Lt)) => Ok(TokenBodyType::Angle),
            Ok(Token::BackQuote) => Ok(TokenBodyType::Template),
            _ => unreachable!(""),
        }
    }

    pub fn is_non_bracketed_token(&mut self) -> PResult<bool> {
        let is_bracket = is_one_of!(self, '(', ')', '[', ']', '{', '}', '<', '>');
        let is_tpl_el = self.ts_look_ahead(|p| Ok(p.is_tpl_el()))?;
        let is_bracketed_token = is_bracket || is_tpl_el;

        Ok(!is_bracketed_token)
    }

    pub fn is_tpl_el(&mut self) -> bool {
        match self.parse_tpl_element(false) {
            Ok(_) => true,
            _ => false,
        }
    }

    pub fn is_bracket_body_start(&mut self) -> bool {
        debug_assert!(self.input.syntax().typescript());

        is_one_of!(self, '(', '[', '{', '<', '`')
    }

    pub fn parse_es_token_body_el(&mut self) -> PResult<TokenOrBracketedTokens> {
        let result = match self.is_bracket_body_start() {
            true => TokenOrBracketedTokens::BracketBody(self.parse_es_bracketed_type()?),
            false => {
                let pos = cur_pos!(self);
                let t = cur!(self, false)?.clone();
                bump!(self);
                TokenOrBracketedTokens::Token(EsToken {
                    span: span!(self, pos),
                    value: format!("{:?}", t),
                })
            }
        };

        Ok(result)
    }

    pub fn parse_es_token_body(
        &mut self,
        t: &TokenBodyType,
    ) -> PResult<Vec<TokenOrBracketedTokens>> {
        let mut buffer = vec![];

        loop {
            trace_cur!(self, parse_es_token_body__element);

            if self.is_bracket_body_end_token(t) {
                break;
            }

            let next = self.parse_es_token_body_el()?;
            buffer.push(next);
        }

        Ok(buffer)
    }

    pub fn parse_es_template_bracket_body(
        &mut self,
    ) -> PResult<(Vec<TokenOrBracketedTokens>, Vec<TplElement>)> {
        let mut exprs = vec![];
        let first_el = self.parse_tpl_element(false)?;
        let mut is_tail = first_el.tail;
        let mut token_body = vec![first_el];

        while !is_tail {
            expect!(self, "${");
            let template_middle = self.parse_es_token_body(&TokenBodyType::Template)?;
            exprs.extend(template_middle);
            expect!(self, '}');
            let next_el = self.parse_tpl_element(false)?;
            is_tail = next_el.tail;
            token_body.push(next_el);
        }

        Ok((exprs, token_body))
    }

    pub fn parse_es_bracket_body(&mut self, token_type: &TokenBodyType) -> PResult<EsBracketBody> {
        let start = cur_pos!(self);

        if token_type == &TokenBodyType::Template {
            let (exprs, token_body) = self.parse_es_template_bracket_body()?;

            return Ok(EsBracketBody::EsTemplateBracketBody(
                EsTemplateBracketBody {
                    span: span!(self, start),
                    exprs,
                    token_body,
                },
            ));
        }

        let token_body = self.parse_es_token_body(token_type)?;

        Ok(EsBracketBody::EsNormalBracketBody(EsNormalBracketBody {
            span: span!(self, start),
            token_body,
        }))
    }

    pub fn parse_es_bracketed_type(&mut self) -> PResult<EsType> {
        if !self.is_bracket_body_start() {
            unexpected!(self, "One of: (,{,[,<,`")
        }

        let token_type = self.get_token_body_type()?;
        let start = cur_pos!(self);
        self.expect_bracket_start_token(&token_type)?;
        let body = self.parse_es_bracket_body(&token_type)?;

        let result = match token_type {
            TokenBodyType::Paren => EsType::EsParenthesizedType(EsParenthesizedType {
                span: span!(self, start),
                body,
            }),
            TokenBodyType::Square => EsType::EsSquareBracketedType(EsSquareBracketedType {
                span: span!(self, start),
                body,
            }),
            TokenBodyType::Angle => EsType::EsAngleBracketedType(EsAngleBracketedType {
                span: span!(self, start),
                body,
            }),
            TokenBodyType::Curly => EsType::EsCurlyBracketedType(EsCurlyBracketedType {
                span: span!(self, start),
                body,
            }),
            TokenBodyType::Template => EsType::EsTemplateBracketedType(EsTemplateBracketedType {
                span: span!(self, start),
                body,
            }),
        };

        self.expect_bracket_end_token(&token_type)?;

        Ok(result)
    }

    pub fn try_parse_es_idx_sig(&mut self, start: BytePos, is_readonly: bool, is_static: bool) -> PResult<Option<EsIndexSignature>> {
        if let Some(ts_idx_sig) = self.try_parse_ts_index_signature(start, is_readonly, is_static)? {
            let binding_id = match ts_idx_sig.params.get(0) {
                Ok(TsFnParam::Ident(b)) => b,
                _ => unreachable!()
            };
            return Ok(Some(EsIndexSignature {
                span: ts_idx_sig.span,
                binding_id,
                type_ann: ts_idx_sig.type_ann.map(|t| t.into()),
                readonly: ts_idx_sig.readonly,
                is_static: ts_idx_sig.is_static
            }))
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use swc_common::DUMMY_SP;
    use swc_petal_ecma_visit::assert_eq_ignore_span;

    use crate::{test_parser, Syntax};
    use swc_petal_ast::{
        EsBracketBody, EsCurlyBracketedType, EsNormalBracketBody, EsParenthesizedType,
        EsSquareBracketedType, EsTemplateBracketBody, EsTemplateBracketedType, EsToken, EsType,
        TokenOrBracketedTokens, TplElement,
    };

    #[test]
    fn bracket_body_single() {
        let result = test_parser(
            "(anything at all)",
            Syntax::EsTypeAnnotations(Default::default()),
            |p| p.parse_es_bracketed_type(),
        );
        let expected = EsType::EsParenthesizedType(EsParenthesizedType {
            span: DUMMY_SP,
            body: EsBracketBody::EsNormalBracketBody(EsNormalBracketBody {
                span: DUMMY_SP,
                token_body: vec![
                    TokenOrBracketedTokens::Token(EsToken {
                        span: DUMMY_SP,
                        value: "anything".into(),
                    }),
                    TokenOrBracketedTokens::Token(EsToken {
                        span: DUMMY_SP,
                        value: "at".into(),
                    }),
                    TokenOrBracketedTokens::Token(EsToken {
                        span: DUMMY_SP,
                        value: "all".into(),
                    }),
                ],
            }),
        });

        assert_eq_ignore_span!(result, expected);
    }

    #[test]
    fn bracket_body_single_empty() {
        let result = test_parser("()", Syntax::EsTypeAnnotations(Default::default()), |p| {
            p.parse_es_bracketed_type()
        });
        let expected = EsType::EsParenthesizedType(EsParenthesizedType {
            span: DUMMY_SP,
            body: EsBracketBody::EsNormalBracketBody(EsNormalBracketBody {
                span: DUMMY_SP,
                token_body: vec![],
            }),
        });

        assert_eq_ignore_span!(result, expected);
    }

    #[test]
    fn bracket_body_nested() {
        let result = test_parser(
            "(anything[at{all}])",
            Syntax::EsTypeAnnotations(Default::default()),
            |p| p.parse_es_bracketed_type(),
        );
        let expected = EsType::EsParenthesizedType(EsParenthesizedType {
            span: DUMMY_SP,
            body: EsBracketBody::EsNormalBracketBody(EsNormalBracketBody {
                span: DUMMY_SP,
                token_body: vec![
                    TokenOrBracketedTokens::Token(EsToken {
                        span: DUMMY_SP,
                        value: "anything".into(),
                    }),
                    TokenOrBracketedTokens::BracketBody(EsType::EsSquareBracketedType(
                        EsSquareBracketedType {
                            span: DUMMY_SP,
                            body: EsBracketBody::EsNormalBracketBody(EsNormalBracketBody {
                                span: DUMMY_SP,
                                token_body: vec![
                                    TokenOrBracketedTokens::Token(EsToken {
                                        span: DUMMY_SP,
                                        value: "at".into(),
                                    }),
                                    TokenOrBracketedTokens::BracketBody(
                                        EsType::EsCurlyBracketedType(EsCurlyBracketedType {
                                            span: DUMMY_SP,
                                            body: EsBracketBody::EsNormalBracketBody(
                                                EsNormalBracketBody {
                                                    span: DUMMY_SP,
                                                    token_body: vec![
                                                        TokenOrBracketedTokens::Token(EsToken {
                                                            span: DUMMY_SP,
                                                            value: "all".into(),
                                                        }),
                                                    ],
                                                },
                                            ),
                                        }),
                                    ),
                                ],
                            }),
                        },
                    )),
                ],
            }),
        });

        assert_eq_ignore_span!(result, expected);
    }

    #[test]
    fn bracket_body_template() {
        let result = test_parser(
            "`anything${number}atall`",
            Syntax::EsTypeAnnotations(Default::default()),
            |p| p.parse_es_bracketed_type(),
        );

        let expected = EsType::EsTemplateBracketedType(EsTemplateBracketedType {
            span: DUMMY_SP,
            body: EsBracketBody::EsTemplateBracketBody(EsTemplateBracketBody {
                span: DUMMY_SP,
                exprs: vec![TokenOrBracketedTokens::Token(EsToken {
                    span: DUMMY_SP,
                    value: "number".into(),
                })],
                token_body: vec![
                    TplElement {
                        span: DUMMY_SP,
                        cooked: Some("anything".into()),
                        raw: "anything".into(),
                        tail: false,
                    },
                    TplElement {
                        span: DUMMY_SP,
                        cooked: Some("atall".into()),
                        raw: "atall".into(),
                        tail: true,
                    },
                ],
            }),
        });

        assert_eq_ignore_span!(result, expected);
    }

    #[test]
    fn bracket_body_template_nested() {
        let result = test_parser(
            "`(anything)${(hello)}atall`",
            Syntax::EsTypeAnnotations(Default::default()),
            |p| p.parse_es_bracketed_type(),
        );

        let expected = EsType::EsTemplateBracketedType(EsTemplateBracketedType {
            span: DUMMY_SP,
            body: EsBracketBody::EsTemplateBracketBody(EsTemplateBracketBody {
                span: DUMMY_SP,
                exprs: vec![TokenOrBracketedTokens::BracketBody(
                    EsType::EsParenthesizedType(EsParenthesizedType {
                        span: DUMMY_SP,
                        body: EsBracketBody::EsNormalBracketBody(EsNormalBracketBody {
                            span: DUMMY_SP,
                            token_body: vec![TokenOrBracketedTokens::Token(EsToken {
                                span: DUMMY_SP,
                                value: "hello".into(),
                            })],
                        }),
                    }),
                )],
                token_body: vec![
                    TplElement {
                        span: DUMMY_SP,
                        cooked: Some("(anything)".into()),
                        raw: "(anything)".into(),
                        tail: false,
                    },
                    TplElement {
                        span: DUMMY_SP,
                        cooked: Some("atall".into()),
                        raw: "atall".into(),
                        tail: true,
                    },
                ],
            }),
        });

        assert_eq_ignore_span!(result, expected);
    }
}
