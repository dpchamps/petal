use rslint_errors::Diagnostic;
use rslint_lexer::{Lexer, SyntaxKind, Token};
use rslint_rowan::{TextRange, TextSize};
use swc_common::{BytePos, Span, SyntaxContext};

use swc_petal_ast::*;
mod module;
mod types;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    source: &'a str,
    current: Option<Token>,
    error_state: Vec<Diagnostic>,
    last_position: usize,
    position: usize,
}

#[derive(Debug)]
pub enum ParseErr {
    CatchAll,
    UnexpectedParserState(Span, String),
    MissingSemicolon(Span),
}

type ParseResult<T> = Result<T, ParseErr>;

impl<'a> Parser<'a> {
    fn advance(&mut self) {
        let mut next = None;
        for (token, error) in self.lexer.by_ref() {
            self.last_position = self.position;
            self.position += token.len;

            if token.kind != SyntaxKind::WHITESPACE {
                next = Some((token, error));
                break;
            }
        }

        if let Some((token, diagnostic)) = next {
            self.current = Some(token);

            if let Some(diagnostic) = diagnostic {
                self.error_state.push(diagnostic)
            }
        }
    }

    fn expected_state(&self, expected: &str) -> ParseErr {
        ParseErr::UnexpectedParserState(self.span(), format!("expected {}", expected))
    }

    fn eat(&mut self, kind: SyntaxKind) -> Option<Token> {
        if self.current.is_none() {
            self.advance();
        }

        match self.current {
            Some(next) if next.kind == kind => {
                self.advance();
                Some(next)
            }
            _ => None,
        }
    }

    fn eat_raw(&mut self, input: &str) -> Option<Token> {
        if self.is(input) {
            let current = self.current;
            self.advance();
            return current;
        }

        None
    }

    fn expect(&mut self, token: SyntaxKind) -> ParseResult<Token> {
        self.eat(token)
            .ok_or(self.expected_state(token.to_string().unwrap_or("unknown")))
    }

    fn expect_raw(&mut self, input: &str) -> ParseResult<Token> {
        self.eat_raw(input).ok_or(self.expected_state(input))
    }

    fn raw(&mut self) -> &'a str {
        &self.source[TextRange::new(
            TextSize::from(self.last_position as u32),
            TextSize::from(self.position as u32),
        )]
    }

    fn raw_from_token(&self, start: BytePos, token: Token) -> &'a str {
        &self.source[TextRange::new(
            TextSize::from(start.0),
            TextSize::from(start.0 + token.len as u32),
        )]
    }

    fn raw_from_range(&self, start: u32, end: u32) -> &'a str {
        &self.source[TextRange::new(TextSize::from(start), TextSize::from(end))]
    }

    fn is(&mut self, input: &str) -> bool {
        if self.current.is_none() {
            self.advance()
        }

        self.raw() == input
    }

    fn is_kind(&mut self, token: SyntaxKind) -> bool {
        if self.current.is_none() {
            self.advance()
        }

        if let Some(current_token) = self.current {
            return current_token.kind == token;
        }

        false
    }

    fn eof(&mut self) -> bool {
        self.is_kind(SyntaxKind::EOF) || self.current.is_none()
    }

    fn span(&self) -> Span {
        Span::new(
            BytePos(self.last_position as u32),
            BytePos(self.position as u32),
            SyntaxContext::empty(),
        )
    }

    fn span_start(&self) -> BytePos {
        BytePos(self.last_position as u32)
    }

    fn finish_span(&self, span_start: BytePos) -> Span {
        Span::new(
            span_start,
            BytePos(self.position as u32),
            SyntaxContext::empty(),
        )
    }

    fn semicolon(&mut self) -> ParseResult<()> {
        self.eat(SyntaxKind::SEMICOLON)
            .map(|_| ())
            .ok_or(ParseErr::MissingSemicolon(
                self.finish_span(self.span_start()),
            ))
    }

    fn finish_trailing_comma(&mut self, terminal: SyntaxKind) -> ParseResult<()> {
        // if the next two things aren't a comma AND the closing of the argument list, we're in an invalid state
        if self.eat(SyntaxKind::COMMA).is_none() && !self.is_kind(terminal) {
            return Err(self.expected_state(&format!(
                "expected '{}' or ','",
                terminal.to_string().unwrap_or("unknown")
            )));
        };

        Ok(())
    }

    pub fn new(source: &'a str) -> Self {
        let lexer = Lexer::from_str(source, 0);

        Self {
            lexer,
            source,
            current: None,
            error_state: vec![],
            position: 0,
            last_position: 0,
        }
    }

    pub fn parse(input: &'a str) -> Result<Program, ParseErr> {
        let mut parser = Parser::new(input);
        parser.advance();

        Ok(Program::Module(parser.parse_module()?))
    }

    pub(crate) fn parse_ident(&mut self) -> ParseResult<Ident> {
        let start = self.span_start();
        let ident_token = self.expect(SyntaxKind::IDENT)?;
        let ident_name = self.raw_from_token(start, ident_token);
        Ok(Ident {
            span: self.finish_span(start),
            sym: ident_name.into(),
            optional: false,
        })
    }

    pub(crate) fn parse_str(&mut self) -> ParseResult<Str> {
        let start = self.span_start();
        let token = self
            .eat(SyntaxKind::STRING)
            .ok_or(ParseErr::UnexpectedParserState(
                self.finish_span(start),
                "expected to find string, this is a parser problem".into(),
            ))?;

        Ok(Str {
            span: self.finish_span(start),
            value: self
                .raw_from_range(start.0 + 1, start.0 + (token.len as u32) - 1)
                .into(),
            raw: None,
        })
    }

    pub(crate) fn parse_number(&mut self) -> ParseResult<Number> {
        let start = self.span_start();
        let number_token = self.expect(SyntaxKind::NUMBER)?;
        let raw_number = self.raw_from_token(start, number_token);
        let parsed_number = raw_number.parse::<f64>().map_err(|_| ParseErr::UnexpectedParserState(self.finish_span(start),  format!("Fatal parser error: expected a number, but could not convert string into number: {}", raw_number)))?;
        Ok(Number {
            span: self.finish_span(start),
            value: parsed_number,
            raw: Some(raw_number.into()),
        })
    }

    pub(crate) fn parse_bool(&mut self) -> ParseResult<Bool> {
        let start = self.span_start();
        let value = self
            .eat(SyntaxKind::TRUE_KW)
            .map(|_| true)
            .or_else(|| self.eat(SyntaxKind::FALSE_KW).map(|_| false))
            .ok_or(ParseErr::UnexpectedParserState(
                self.span(),
                "Expected true or false keyword".into(),
            ))?;

        Ok(Bool {
            span: self.finish_span(start),
            value,
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn is() {
        let source = "abra kadabra alakazam";
        let mut parser = Parser::new(source);

        assert!(parser.is("abra"));
        parser.advance();
        assert!(parser.is("kadabra"));
        parser.advance();
        assert!(parser.is("alakazam"));
    }
}
