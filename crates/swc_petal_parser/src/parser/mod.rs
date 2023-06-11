#![allow(clippy::let_unit_value)]
#![deny(non_snake_case)]

use std::ops::{Deref, DerefMut};

use swc_atoms::JsWord;
use swc_common::{collections::AHashMap, comments::Comments, input::Input, BytePos, Span};
use swc_petal_ast::*;

pub use self::input::{Capturing, Tokens, TokensInput};
use self::{input::Buffer, util::ParseObject};
use crate::{
    error::SyntaxError,
    lexer::Lexer,
    token::{Token, Word},
    Context, EsVersion, Syntax, TsConfig,
};
#[cfg(test)]
extern crate test;
#[cfg(test)]
use test::Bencher;

use crate::error::Error;

#[macro_use]
mod macros;
mod class_and_fn;
mod es_type_annotation;
mod expr;
mod ident;
pub mod input;
mod jsx;
mod object;
mod pat;
mod stmt;
#[cfg(test)]
mod tests;
mod typescript;
mod util;
/// When error occurs, error is emitted and parser returns Err(()).
pub type PResult<T> = Result<T, Error>;

/// EcmaScript parser.
#[derive(Clone)]
pub struct Parser<I: Tokens> {
    state: State,
    input: Buffer<I>,
}

#[derive(Clone, Default)]
struct State {
    labels: Vec<JsWord>,
    /// Start position of an assignment expression.
    potential_arrow_start: Option<BytePos>,

    found_module_item: bool,
    /// Start position of an AST node and the span of its trailing comma.
    trailing_commas: AHashMap<BytePos, Span>,
}

impl<'a, I: Input> Parser<Lexer<'a, I>> {
    pub fn new(syntax: Syntax, input: I, comments: Option<&'a dyn Comments>) -> Self {
        Self::new_from(Lexer::new(syntax, Default::default(), input, comments))
    }
}

impl<I: Tokens> Parser<I> {
    pub fn new_from(mut input: I) -> Self {
        let in_declare = matches!(
            input.syntax(),
            Syntax::Typescript(TsConfig { dts: true, .. })
        );
        let ctx = Context {
            in_declare,
            ..input.ctx()
        };
        input.set_ctx(ctx);

        Parser {
            state: Default::default(),
            input: Buffer::new(input),
        }
    }

    pub fn take_errors(&mut self) -> Vec<Error> {
        self.input().take_errors()
    }

    pub(crate) fn target(&self) -> EsVersion {
        self.input.target()
    }

    pub fn parse_module(&mut self) -> PResult<Module> {
        let ctx = Context {
            module: true,
            can_be_module: true,
            strict: true,
            ..self.ctx()
        };
        // Module code is always in strict mode
        self.set_ctx(ctx);

        let start = cur_pos!(self);
        let shebang = self.parse_shebang()?;

        self.parse_block_body(true, true, None).map(|body| Module {
            span: span!(self, start),
            body,
            shebang,
        })
    }

    fn parse_shebang(&mut self) -> PResult<Option<JsWord>> {
        match cur!(self, false) {
            Ok(&Token::Shebang(..)) => match bump!(self) {
                Token::Shebang(v) => Ok(Some(v)),
                _ => unreachable!(),
            },
            _ => Ok(None),
        }
    }

    fn ctx(&self) -> Context {
        self.input.get_ctx()
    }

    #[cold]
    fn emit_err(&self, span: Span, error: SyntaxError) {
        if self.ctx().ignore_error || !self.syntax().early_errors() {
            return;
        }

        self.emit_error(Error::new(span, error))
    }

    #[cold]
    fn emit_error(&self, error: Error) {
        if self.ctx().ignore_error || !self.syntax().early_errors() {
            return;
        }

        self.input_ref().add_error(error);
    }

    #[cold]
    fn emit_strict_mode_err(&self, span: Span, error: SyntaxError) {
        if self.ctx().ignore_error {
            return;
        }
        let error = Error::new(span, error);
        self.input_ref().add_module_mode_error(error);
    }
}

#[cfg(test)]
pub fn test_parser<F, Ret>(s: &'static str, syntax: Syntax, f: F) -> Ret
where
    F: FnOnce(&mut Parser<Lexer<crate::StringInput<'_>>>) -> Result<Ret, Error>,
{
    crate::with_test_sess(s, |handler, input| {
        let lexer = Lexer::new(syntax, EsVersion::Es2019, input, None);
        let mut p = Parser::new_from(lexer);
        let ret = f(&mut p);
        let mut error = false;

        for err in p.take_errors() {
            error = true;
            err.into_diagnostic(handler).emit();
        }

        let res = ret.map_err(|err| err.into_diagnostic(handler).emit())?;

        if error {
            return Err(());
        }

        Ok(res)
    })
    .unwrap_or_else(|output| panic!("test_parser(): failed to parse \n{}\n{}", s, output))
}

#[cfg(test)]
pub fn test_parser_comment<F, Ret>(c: &dyn Comments, s: &'static str, syntax: Syntax, f: F) -> Ret
where
    F: FnOnce(&mut Parser<Lexer<crate::StringInput<'_>>>) -> Result<Ret, Error>,
{
    crate::with_test_sess(s, |handler, input| {
        let lexer = Lexer::new(syntax, EsVersion::Es2019, input, Some(&c));
        let mut p = Parser::new_from(lexer);
        let ret = f(&mut p);

        for err in p.take_errors() {
            err.into_diagnostic(handler).emit();
        }

        ret.map_err(|err| err.into_diagnostic(handler).emit())
    })
    .unwrap_or_else(|output| panic!("test_parser(): failed to parse \n{}\n{}", s, output))
}

#[cfg(test)]
pub fn bench_parser<F>(b: &mut Bencher, s: &'static str, syntax: Syntax, mut f: F)
where
    F: for<'a> FnMut(&'a mut Parser<Lexer<'a, crate::StringInput<'_>>>) -> PResult<()>,
{
    b.bytes = s.len() as u64;

    let _ = crate::with_test_sess(s, |handler, input| {
        b.iter(|| {
            let lexer = Lexer::new(syntax, Default::default(), input.clone(), None);
            let _ =
                f(&mut Parser::new_from(lexer)).map_err(|err| err.into_diagnostic(handler).emit());
        });

        Ok(())
    });
}
