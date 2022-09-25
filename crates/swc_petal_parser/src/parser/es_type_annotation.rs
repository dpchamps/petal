// use std::fmt::Write;

// use either::Either;
// use swc_atoms::js_word;
// use swc_common::{Spanned, SyntaxContext};

use super::*;
use crate::token::BinOpToken;
// use crate::{lexer::TokenContexts, parser::class_and_fn::IsSimpleParameterList, token::Keyword};

pub enum TokenBodyType {
    Paren,
    Square,
    Curly,
    Angle,
    // TODO: template bracketed tokens
}

impl<I: Tokens> Parser<I> {
    fn is_bracket_body_end_token(&mut self, t: &TokenBodyType) -> bool {
        match t {
            TokenBodyType::Paren => is!(self, ')'),
            TokenBodyType::Square => is!(self, ']'),
            TokenBodyType::Curly => is!(self, '}'),
            TokenBodyType::Angle => is!(self, '>'),
        }
    }

    fn get_token_body_type(&mut self) -> PResult<TokenBodyType> {
        match cur!(self, false) {
            Ok(Token::LParen) => Ok(TokenBodyType::Paren),
            Ok(Token::LBrace) => Ok(TokenBodyType::Curly),
            Ok(Token::LBracket) => Ok(TokenBodyType::Square),
            Ok(Token::BinOp(BinOpToken::Lt)) => Ok(TokenBodyType::Angle),
            _ => unreachable!(""),
        }
    }

    pub fn is_non_bracked_token(&mut self) -> PResult<bool> {
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

        is_one_of!(self, '(', '[', '{', '<')
    }

    pub fn is_bracket_body_terminator(&mut self) -> PResult<bool> {
        debug_assert!(self.input.syntax().typescript());

        Ok(is_one_of!(self, ')', ']', '}', '>'))
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
                bump!(self);
                break;
            }

            let next = self.parse_es_token_body_el()?;
            buffer.push(next);
        }

        Ok(buffer)
    }

    pub fn parse_es_bracket_body(&mut self, token_type: &TokenBodyType) -> PResult<EsBracketBody> {
        let start = cur_pos!(self);

        let token_body = self.parse_es_token_body(&token_type)?;

        Ok(EsBracketBody {
            span: span!(self, start),
            token_body,
        })
    }

    pub fn parse_es_bracketed_type(&mut self) -> PResult<EsType> {
        if !self.is_bracket_body_start() {
            unexpected!(self, "One of: (,{,[,<")
        }

        let token_type = self.get_token_body_type()?;
        let start = cur_pos!(self);
        bump!(self);

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
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    // use swc_common::DUMMY_SP;
    // use swc_ecma_visit::assert_eq_ignore_span;
    // use swc_petal_ast::*;

    use swc_common::DUMMY_SP;
    use swc_petal_ecma_visit::assert_eq_ignore_span;

    use crate::{test_parser, Syntax};
    use swc_petal_ast::{
        EsBracketBody, EsCurlyBracketedType, EsParenthesizedType, EsSquareBracketedType, EsToken,
        EsType, TokenOrBracketedTokens,
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
            body: EsBracketBody {
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
            },
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
            body: EsBracketBody {
                span: DUMMY_SP,
                token_body: vec![],
            },
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
            body: EsBracketBody {
                span: DUMMY_SP,
                token_body: vec![
                    TokenOrBracketedTokens::Token(EsToken {
                        span: DUMMY_SP,
                        value: "anything".into(),
                    }),
                    TokenOrBracketedTokens::BracketBody(EsType::EsSquareBracketedType(
                        EsSquareBracketedType {
                            span: DUMMY_SP,
                            body: EsBracketBody {
                                span: DUMMY_SP,
                                token_body: vec![
                                    TokenOrBracketedTokens::Token(EsToken {
                                        span: DUMMY_SP,
                                        value: "at".into(),
                                    }),
                                    TokenOrBracketedTokens::BracketBody(
                                        EsType::EsCurlyBracketedType(EsCurlyBracketedType {
                                            span: DUMMY_SP,
                                            body: EsBracketBody {
                                                span: DUMMY_SP,
                                                token_body: vec![TokenOrBracketedTokens::Token(
                                                    EsToken {
                                                        span: DUMMY_SP,
                                                        value: "all".into(),
                                                    },
                                                )],
                                            },
                                        }),
                                    ),
                                ],
                            },
                        },
                    )),
                ],
            },
        });

        assert_eq_ignore_span!(result, expected);
    }
}
