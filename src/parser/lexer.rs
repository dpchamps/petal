use crate::parser::string_utils;
use crate::parser::tokens::Token;
use std::i64;
use std::str::Chars;
use std::thread::current;
use unic_ucd_ident::{is_xid_continue, is_xid_start};

// TODO: This incorrectly assumes that all source input will be a utf-8 string.
//  However, ECMA-2020 supports unicode strings.
pub struct Lexer {
    cursor: usize,
    source: Vec<char>,
    context: LexerContext,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum LexingError {
    ArbitraryError,
    ErrorWithMessage(&'static str),
    InvalidToken,
    UnexpectedToken(String),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum LexerContext {
    InputElementDiv,
    InputElementRegExp,
    InputElementRegExpOrTemplateTail,
    InputElementTemplateTail
}

type LexerResult<T> = Result<Option<T>, LexingError>;

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            cursor: 0,
            source: source
                .as_bytes()
                .iter()
                .map(|x| x.clone() as char)
                .collect(),
            context: LexerContext::InputElementDiv
        }
    }

    fn advance(&mut self) {
        self.cursor += 1;
    }

    fn retreat(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    fn backtrack(&mut self, n: usize) {
        for _ in 0..n {
            self.retreat()
        }
    }

    fn consume(&mut self, c: char) -> Option<char> {
        match self.current() {
            Some(x) if x == &c => self.chomp(),
            _ => None,
        }
    }

    fn eat(&mut self, c: char) -> bool {
        return self.consume(c).is_some();
    }

    fn chomp(&mut self) -> Option<char> {
        if let Some(x) = self.current() {
            let result = x.clone();
            self.advance();

            return Some(result);
        }

        None
    }

    fn consume_push(&mut self, c: char, output: &mut String) -> bool {
        if let Some(x) = self.consume(c) {
            output.push_str(&x.to_string());

            return true;
        }

        false
    }

    fn consume_sequence(&mut self, seq: &str) -> Option<String> {
        let mut result = String::from("");

        seq.chars().for_each(|c| {
            if let Some(next) = self.consume(c) {
                result.push_str(&c.to_string())
            } else {
                return ();
            }
        });

        if result.len() != seq.len() {
            self.backtrack(result.len());
            return None;
        }

        return Some(result);
    }

    fn consume_alpha_numeric(&mut self) -> Option<char> {
        match self.current() {
            Some(x) if x.is_alphanumeric() => self.chomp(),
            _ => None,
        }
    }

    fn consume_digit(&mut self, radix: u32) -> Option<char> {
        match self.current() {
            Some(x) if x.to_digit(radix).is_some() => self.chomp(),
            _ => None,
        }
    }

    fn consume_codepoint_id_start(&mut self) -> Option<char> {
        match self.current() {
            Some(x) if is_xid_start(*x) => self.chomp(),
            _ => None,
        }
    }

    fn consume_codepoint_id_continue(&mut self) -> Option<char> {
        match self.current() {
            Some(x) if is_xid_continue(*x) => self.chomp(),
            _ => None,
        }
    }

    fn consume_zero_width_codepoint(&mut self) -> Option<char> {
        match self.current() {
            Some(x) if string_utils::is_zwj(x) || string_utils::is_zwnj(x) => self.chomp(),
            _ => None,
        }
    }

    fn consume_single_punctuator(&mut self) -> Option<char> {
        match self.current() {
            Some(x) if string_utils::is_valid_punctuator_char(x) => self.chomp(),
            _ => None,
        }
    }

    fn peek(&self, c: char, i: usize) -> bool {
        match self.lookahead(i) {
            Some(x) => return x == &c,
            _ => false,
        }
    }

    fn is(&self, c: char) -> bool {
        match self.current() {
            Some(x) if *x == c => true,
            _ => false,
        }
    }

    fn lookahead(&self, i: usize) -> Option<&char> {
        self.source.get(self.cursor + i)
    }

    fn current(&self) -> Option<&char> {
        self.lookahead(0)
    }

    fn is_hex_digit(&self) -> bool {
        if let Some(x) = self.current() {
            return x.to_digit(16).is_some();
        }

        false
    }

    fn consume_whitespace(&mut self) -> Option<()> {
        match self.current() {
            Some(x) if string_utils::is_whitespace(x) || string_utils::is_line_terminator(x) => {
                self.advance();
                Some(())
            }
            _ => None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(_) = self.consume_whitespace() {}
    }

    fn skip_single_line_comment(&mut self) {
        while let Some(x) = self.current() {
            if string_utils::is_line_terminator(x) {
                break;
            }
            self.advance();
        }
    }

    fn skip_multiline_comments(&mut self) {
        loop {
            if self.eat('*') && self.eat('/') {
                break;
            }

            self.advance();
        }
    }

    fn skip_comments(&mut self) {
        if self.eat('/') {
            if self.eat('*') {
                self.skip_multiline_comments();
            } else if self.eat('/') {
                self.skip_single_line_comment();
            }
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        self.skip_whitespace();
        self.skip_comments();

        if let Some(x) = self.current() {
            if string_utils::is_whitespace(x) || string_utils::is_line_terminator(x) {
                self.skip_whitespace_and_comments();
            }
        }
    }

    fn consume_hex_4_digits(&mut self) -> Option<String> {
        let mut result = String::from("");

        for _ in 0..4 {
            if let Some(x) = self.consume_digit(16) {
                result.push_str(&x.to_string());
            } else {
                self.backtrack(result.len());
                return None;
            }
        }

        Some(result)
    }

    fn consume_code_point(&mut self) -> Option<String> {
        let mut result = String::from("");

        while let Some(x) = self.consume_digit(16) {
            result.push_str(&x.to_string());
        }

        if i64::from_str_radix(&result, 16).unwrap() > 0x10FFFF {
            self.backtrack(result.len());
            result = String::from("");
        }

        return if result.len() > 0 { Some(result) } else { None };
    }

    // TODO: This is _definitely_ not correct
    //  validation needs to take place here.
    fn consume_unicode_escape_seq(&mut self) -> LexerResult<String> {
        let mut result = String::from("");

        if self.consume_push('\\', &mut result) && self.consume_push('u', &mut result) {
            if let Some(hex_4) = self.consume_hex_4_digits() {
                let utf_16_str = string_utils::parse_unicode_to_string(&hex_4)
                    .or(Err(LexingError::InvalidToken))?;
                return Ok(Some(utf_16_str));
            } else if self.consume_push('{', &mut result) {
                if let Some(code_point) = self.consume_code_point() {
                    result.push_str(&code_point);

                    if self.consume_push('}', &mut result) {
                        return Ok(Some(result));
                    }
                }
            }
        }

        self.backtrack(result.len());
        return Ok(None);
    }

    fn consume_iden_start(&mut self) -> Option<String> {
        return if let Some(an) = self.consume_codepoint_id_start() {
            Some(an.to_string())
        } else if let Some(q) = self.consume('$') {
            Some(q.to_string())
        } else if let Some(u) = self.consume('_') {
            Some(u.to_string())
        } else {
            self.consume_unicode_escape_seq().unwrap_or(None)
        };
    }

    fn consume_id_part(&mut self) -> Option<String> {
        if let Some(id_part) = self.consume_codepoint_id_continue() {
            return Some(id_part.to_string());
        } else if let Some(iden) = self.consume_iden_start() {
            return Some(iden.to_string());
        } else if let Some(zwcp) = self.consume_zero_width_codepoint() {
            return Some(zwcp.to_string());
        }

        None
    }

    fn consume_identifier_name(&mut self) -> LexerResult<Token> {
        let mut result = String::from("");

        if let Some(id_start) = self.consume_iden_start() {
            result.push_str(&id_start);

            while let Some(id_part) = self.consume_id_part() {
                result.push_str(&id_part);
            }

            return if string_utils::is_keyword(&result) {
                Ok(Some(Token::Keyword(result)))
            } else if string_utils::is_conditional_keyword(&result) {
                Ok(Some(Token::ConditionalKeyword(result)))
            } else if string_utils::is_reserved_word(&result) {
                Ok(Some(Token::ReservedKeyword(result)))
            } else {
                Ok(Some(Token::IdentifierName(result)))
            };
        }

        Ok(None)
    }

    fn consume_div_punctuator(&mut self) -> Option<Token> {
        if let Some(div) = self.consume('/') {
            return Some(Token::Punctuator(div.to_string()));
        } else if let Some(stream) = self.consume_sequence("/=") {
            return Some(Token::Punctuator(stream));
        }

        None
    }

    fn consume_right_brace_punc(&mut self) -> Option<Token> {
        if let Some(rb) = self.consume('{') {
            return Some(Token::Punctuator(rb.to_string()));
        }

        None
    }

    fn consume_punctuator(&mut self) -> Option<Token> {
        let mut result = String::from("");

        if let Some(punc) = self.consume_single_punctuator() {
            result.push_str(&punc.to_string());

            while let Some(next) = self.consume_single_punctuator() {
                result.push_str(&next.to_string());

                if let ("?.", Some(la)) = (result.as_str(), self.lookahead(1)) {
                    if !string_utils::is_decimal_digit(la) {
                        break;
                    }
                }
            }

            if string_utils::is_valid_punctuator(&result) {
                return Some(Token::Punctuator(result));
            }

            while self.cursor > 0 {
                self.backtrack(1);
                result.pop();
                if string_utils::is_valid_punctuator(&result) {
                    return Some(Token::Punctuator(result));
                }
            }
        }

        None
    }

    fn consume_digit_end_source_char(&mut self) -> LexerResult<()> {
        if let Some(_) = self.consume_iden_start() {
            return Err(LexingError::InvalidToken);
        } else if let Some(_) = self.consume_digit(10) {
            return Err(LexingError::InvalidToken);
        }

        Ok(None)
    }

    fn consume_non_decimal_digits(&mut self, radix: u32) -> LexerResult<Token> {
        // eat radix specifier
        self.advance();
        let mut result = String::from("");

        while let Some(digit) = self.consume_digit(radix) {
            result.push_str(&digit.to_string())
        }

        return if let Some(_) = self.consume('n') {
            Ok(Some(Token::BigIntLiteral(result, radix)))
        } else if let Ok(parsed) = i64::from_str_radix(&result, radix) {
            Ok(Some(Token::NumericLiteral(parsed as f64)))
        } else {
            Err(LexingError::ArbitraryError)
        };
    }

    fn consume_decimal_stream(&mut self) -> Option<String> {
        let mut result = String::from("");

        while let Some(digit) = self.consume_digit(10) {
            result.push_str(&digit.to_string())
        }

        return if result.len() > 0 { Some(result) } else { None };
    }

    fn consume_fraction_stream(&mut self) -> LexerResult<String> {
        let mut result = String::from("");

        if self.consume_push('.', &mut result) {
            if let Some(decimalStream) = self.consume_decimal_stream() {
                result.push_str(&decimalStream);
                if let Some(exp_fragment) = self.consume_decimal_exponent_fragment()? {
                    result.push_str(&exp_fragment);
                }

                return Ok(Some(result));
            }
        }

        Ok(None)
    }

    fn consume_decimal_exponent_fragment(&mut self) -> LexerResult<String> {
        let mut result = String::from("");
        match self.current() {
            Some(x) if x.to_ascii_lowercase() == 'e' => {
                result.push_str(&x.to_string());
                self.advance();

                if let Some(sign) = self.consume('+').or(self.consume('-')) {
                    result.push_str(&sign.to_string());
                }

                return if let Some(digit_stream) = self.consume_decimal_stream() {
                    Ok(Some(format!("{}{}", result, digit_stream)))
                } else {
                    Err(LexingError::ErrorWithMessage(
                        "Invalid format for exponent fragment",
                    ))
                };
            }
            _ => Ok(None),
        }
    }

    fn consume_decimal_big_int_literal(&mut self) -> LexerResult<Token> {
        let mut result = String::from("");

        if let Some(_) = self.consume('0') {
            if let None = self.consume('n') {
                return Ok(None);
            }

            return Ok(Some(Token::BigIntLiteral(String::from("0"), 10)));
        }

        if let Some(digits) = self.consume_decimal_stream() {
            result.push_str(&digits);

            if let Some(_) = self.consume('n') {
                return Ok(Some(Token::BigIntLiteral(result, 10)));
            }

            self.backtrack(result.len());
        }

        return Ok(None);
    }

    fn consume_decimal_literal(&mut self) -> LexerResult<Token> {
        let mut result = String::from("");

        // consume numbers starting with .
        if let Some(fraction_stream) = self.consume_fraction_stream()? {
            result.push_str(&fraction_stream);
        }
        // consume numbers starting with decimal digits
        else if let Some(decimal_stream) = self.consume_decimal_stream() {
            result.push_str(&decimal_stream);
            if let Some(exp_fragment) = self.consume_decimal_exponent_fragment()? {
                result.push_str(&exp_fragment);
            } else if let Some(fraction_stream) = self.consume_fraction_stream()? {
                result.push_str(&fraction_stream);
            }
        }

        return if result.len() > 0 {
            Ok(Some(Token::NumericLiteral(result.parse::<f64>().unwrap())))
        } else {
            Ok(None)
        };
    }

    fn consume_non_decimal_integer_literal(&mut self) -> LexerResult<Token> {
        if let Some(prefix) = self.consume('0') {
            if let Some(specifier) = self.current() {
                return match specifier.to_ascii_lowercase() {
                    'b' => self.consume_non_decimal_digits(2),
                    'o' => self.consume_non_decimal_digits(8),
                    'x' => self.consume_non_decimal_digits(16),
                    _ => {
                        self.backtrack(1);
                        Ok(None)
                    }
                };
            }
        }

        Ok(None)
    }

    fn consume_number_literal(&mut self) -> LexerResult<Token> {
        let result = if let Some(non_decimal_integer_literal) =
            self.consume_non_decimal_integer_literal()?
        {
            Ok(Some(non_decimal_integer_literal))
        } else if let Some(decimal_big_int_literal) = self.consume_decimal_big_int_literal()? {
            Ok(Some(decimal_big_int_literal))
        } else if let Some(decimal_literal) = self.consume_decimal_literal()? {
            Ok(Some(decimal_literal))
        } else {
            Ok(None)
        };

        if let Ok(None) = result {
            return result;
        }

        self.consume_digit_end_source_char()?;

        return result;
    }

    fn consume_line_terminator_sequence(&mut self) -> Option<()> {
        match self.current() {
            Some(x) => match string_utils::to_code_point(x) {
                0x000A | 0x2028 | 0x2029 => {
                    self.advance();
                    Some(())
                }
                0x000D => {
                    self.advance();
                    if let Some(lf) = self.current() {
                        if string_utils::to_code_point(lf) == 0x000A {
                            self.advance()
                        }
                    }
                    Some(())
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn consume_escape_character(&mut self) -> Option<char> {
        match self.current() {
            Some(c)
                if string_utils::is_single_escape_character(c)
                    || (c != &'x' && c != &'u' && !string_utils::is_line_terminator(c)) =>
            {
                let result = c.clone();
                self.advance();
                match result {
                    'b' => Some(string_utils::from_code_point(0x8)),
                    'f' => Some(string_utils::from_code_point(0xC)),
                    'n' => Some(string_utils::from_code_point(0xA)),
                    'r' => Some(string_utils::from_code_point(0xD)),
                    't' => Some(string_utils::from_code_point(0x9)),
                    'v' => Some(string_utils::from_code_point(0xB)),
                    _ => Some(result),
                }
            }
            _ => None,
        }
    }

    fn consume_string_escape_sequence(&mut self) -> Option<String> {
        if let None = self.consume('\\') {
            return None;
        }

        if let Some(_) = self.consume('x') {
            let mut result = String::from(r"");

            for _ in 0..2 {
                if let Some(digit) = self.consume_digit(16) {
                    result.push_str(&digit.to_string());
                } else {
                    self.backtrack(result.len());
                    return None;
                }
            }

            return string_utils::parse_utf8_to_string(&result).ok().or(None);
        } else if let (Some(x), Some(la)) = (self.consume('0'), self.lookahead(1)) {
            if !string_utils::is_decimal_digit(la) {
                return Some(String::from('\0'.to_string()));
            }

            self.backtrack(1);
        }

        self.consume_escape_character()
            .and_then(|x| Some(x.to_string()))
    }

    fn consume_source_character(&mut self, quote: &char) -> Option<String> {
        // Template literal special case
        if let Some(x) = self.current() {
            if quote == &'`' && x == &'$' {
                return None;
            }
        }

        match self.current() {
            Some(c)
                if c != &'\\' && c != quote && !string_utils::is_line_terminator(c)
                    || string_utils::is_separator(c) =>
            {
                let result = c.clone();
                self.advance();
                Some(result.to_string())
            }
            Some(c) if c == &'\\' => {
                if let Ok(Some(unicode)) = self.consume_unicode_escape_seq() {
                    return Some(unicode);
                } else if let Some(string_esc) = self.consume_string_escape_sequence() {
                    return Some(string_esc);
                } else if let Some(line_cont) = self.consume_line_terminator_sequence() {
                    return Some('\n'.to_string());
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn consume_string_literal(&mut self) -> LexerResult<Token> {
        if let Some(quote) = self.consume('\'').or(self.consume('"')) {
            let mut result = String::from("");

            while let Some(x) = self.consume_source_character(&quote) {
                result.push_str(&x)
            }

            if let None = self.consume(quote) {
                return Err(LexingError::InvalidToken);
            }

            return Ok(Some(Token::StringLiteral(result)));
        }

        Ok(None)
    }

    fn consume_literal(&mut self) -> LexerResult<Token> {
        return if let Some(null_literal) = self.consume_sequence("null") {
            Ok(Some(Token::NullLiteral))
        } else if let Some(boolean_literal) = self
            .consume_sequence("true")
            .or(self.consume_sequence("false"))
        {
            Ok(Some(Token::BooleanLiteral(&boolean_literal == "true")))
        } else {
            Ok(None)
        };
    }

    fn consume_template_character(&mut self) -> Option<char> {
        if let Some(c) = self.current() {
            if c != &'`' && c != &'\\' && c != &'$' && !string_utils::is_line_terminator(c) {
                return self.chomp();
            }
        };

        None
    }

    fn consume_not_escape_seq(&mut self) -> Option<String> {
        // TODO: This is an annoying production, don't need it for most things I don't think.
        //  Come back after a while

        None
    }

    fn consume_template_esc_seq(&mut self) -> LexerResult<String> {
        if !self.is('\\') {
            return Ok(None);
        }

        return if let Some(like_esc_seq) = self.consume_not_escape_seq() {
            Ok(Some(like_esc_seq))
        } else if let Some(unicode_esc_seq) = self.consume_unicode_escape_seq()? {
            Ok(Some(unicode_esc_seq))
        } else if let Some(string_esc) = self.consume_string_escape_sequence() {
            Ok(Some(string_esc))
        } else if let Some(_) = self.consume_line_terminator_sequence() {
            Ok(Some('\n'.to_string()))
        } else {
            Ok(None)
        };
    }

    fn consume_template(&mut self) -> LexerResult<Token> {
        let mut result = String::from("");

        if let Some(_) = self.consume('`') {
            loop {
                if let Some(_) = self.consume('`') {
                    break;
                }

                if let Some(source_char) = self.consume_template_character() {
                    result.push_str(&source_char.to_string());
                } else if let Some(line_term) = self.consume_line_terminator_sequence() {
                    result.push_str(&'\n'.to_string());
                } else if let Some(template_esc) = self.consume_template_esc_seq()? {
                    result.push_str(&template_esc);
                } else if let Some(dollar_sign) = self.consume('$') {
                    if !self.peek('{', 1) {
                        result.push_str(&dollar_sign.to_string());
                    } else {
                        return Ok(Some(Token::TemplateHead(result)));
                    }
                }
            }

            return Ok(Some(Token::NoSubstitutionTemplate(result)));
        }

        Ok(None)
    }

    fn consume_template_sub_tail(&mut self) -> LexerResult<Token> {
        unimplemented!()
    }

    fn consume_regex_flags(&mut self) -> String {
        let mut result = String::new();

        while let Some(x) = self.consume_id_part(){
            result.push_str(&x)
        }

        result
    }

    fn consume_regex_non_terminator(&mut self) -> Option<String> {
        match self.current() {
            Some(x) if !string_utils::is_line_terminator(x) => {
                self.chomp().map(|x| x.to_string())
            },
            _ => None
        }
    }

    fn consume_regex_backslash_seq(&mut self) -> Option<String> {
        if let None = self.consume('\\') { return None }

        self.consume_regex_non_terminator()
    }

    fn consume_regex_non_term_exception(&mut self) -> Option<String> {
        match self.current() {
            Some(x) => {
                if x == &'*' || x == &'\\' || x == &'/' || x == &'[' {
                    return None
                }

                self.consume_regex_non_terminator()
            },
            _ => None
        }
    }

    fn consume_regex_non_term_class_exception(&mut self) -> Option<String> {
        match self.current() {
            Some(x) => {
                if x == &'\\' || x == &'[' {
                    return None
                }

                self.consume_regex_non_terminator()
            },
            _ => None
        }
    }

    fn consume_regex_class_char(&mut self) -> Option<String> {
        return if let Some(backslash_seq) = self.consume_regex_backslash_seq() {
            Some(backslash_seq)
        }else if let Some(non_term) = self.consume_regex_non_term_class_exception() {
            Some(non_term)
        } else{
            None
        }
    }

    fn consume_regex_class_chars(&mut self) -> String {
        let mut result = String::new();

        while let Some(next) = self.consume_regex_class_char(){
            result.push_str(&next)
        }

        result
    }

    fn consume_regex_class(&mut self) -> LexerResult<String> {
        if let None = self.consume('['){ return Ok(None)}

        let chars = self.consume_regex_class_chars();

        if let None = self.consume(']'){ return Err(LexingError::InvalidToken)}

        Ok(Some(chars))
    }

    fn consume_regex_char(&mut self) -> LexerResult<String> {
        return if let Some(class) = self.consume_regex_class()?{
            Ok(Some(class))
        } else if let Some(backslash_seq) = self.consume_regex_backslash_seq() {
            Ok(Some(backslash_seq))
        } else if let Some(non_term) = self.consume_regex_non_term_exception() {
            Ok(Some(non_term))
        } else {
            Ok(None)
        }
    }

    fn consume_regex_chars(&mut self) -> LexerResult<String> {
        let mut result = String::new();;

        while let Some(next) = self.consume_regex_char()? {
            result.push_str(&next)
        }

        return if result.len() > 0 {
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    fn consume_regex_body(&mut self) -> LexerResult<String> {
        self.consume_regex_char()?
            .map(|first_char| {
                let regex_chars = self.consume_regex_chars()?.unwrap_or(String::new());

                Ok(Some(format!("{}{}", first_char, regex_chars)))
            })
            .unwrap_or(Ok(None))
    }

    fn consume_regex_literal(&mut self) -> LexerResult<Token> {
        if let None = self.consume('/') {
            return Ok(None);
        }

        self.consume_regex_body()?
            .map(|body| {
                let flags = self.consume_regex_flags();
                self.consume('/')
                    .map_or(Err(LexingError::InvalidToken), |_| {
                        Ok(Some(Token::RegexLiteral(body, flags)))
                    })
            })
            .unwrap_or(Ok(None))
    }

    fn consume_common_token(&mut self) -> LexerResult<Token> {
        return if let Some(template) = self.consume_template()? {
            Ok(Some(template))
        } else if let Some(string) = self.consume_string_literal()? {
            Ok(Some(string))
        } else if let Some(numeric) = self.consume_number_literal()? {
            Ok(Some(numeric))
        } else if let Some(punc) = self.consume_punctuator() {
            Ok(Some(punc))
        } else if let Some(literal) = self.consume_literal()? {
            Ok(Some(literal))
        } else if let Some(ident) = self.consume_identifier_name()? {
            Ok(Some(ident))
        } else {
            Ok(None)
        };
    }

    fn set_context_from_token(&mut self, token: &Token){
        match token {
            _ => self.context = LexerContext::InputElementDiv
        }
    }

    fn next_input_el_div(&mut self) -> Result<Token, LexingError> {
        return if let Some(rbp) = self.consume_right_brace_punc(){
            Ok(rbp)
        } else if let Some(dp) = self.consume_div_punctuator() {
           Ok(dp)
        } else if let Some(common) = self.consume_common_token()? {
            Ok(common)
        }else {
            Ok(Token::EOF)
        };
    }

    fn next_input_el_regex(&mut self) -> Result<Token, LexingError> {
        return if let Some(regex) = self.consume_regex_literal()? {
            Ok(regex)
        }else if let Some(rbp) = self.consume_right_brace_punc(){
            Ok(rbp)
        } else if let Some(common) = self.consume_common_token()? {
            Ok(common)
        }else {
            Ok(Token::EOF)
        };
    }

    fn next_input_el_regex_template_tail(&mut self) -> Result<Token, LexingError> {
        return if let Some(template_tail) = self.consume_template_sub_tail()?{
            Ok(template_tail)
        } else if let Some(regex) = self.consume_regex_literal()? {
            Ok(regex)
        }else if let Some(common) = self.consume_common_token()? {
            Ok(common)
        }else {
            Ok(Token::EOF)
        };
    }

    fn next_input_el_template_tail(&mut self) -> Result<Token, LexingError> {
        return if let Some(template_tail) = self.consume_template_sub_tail()?{
            Ok(template_tail)
        } else if let Some(div) = self.consume_div_punctuator() {
            Ok(div)
        }else if let Some(common) = self.consume_common_token()? {
            Ok(common)
        }else {
            Ok(Token::EOF)
        };
    }

    fn next_from_context(&mut self) -> Result<Token, LexingError> {
        self.skip_whitespace_and_comments();

        match self.context {
            LexerContext::InputElementDiv => self.next_input_el_div(),
            LexerContext::InputElementRegExp => self.next_input_el_regex(),
            LexerContext::InputElementRegExpOrTemplateTail => self.next_input_el_regex_template_tail(),
            LexerContext::InputElementTemplateTail => self.next_input_el_template_tail(),
        }
    }

    pub fn next(&mut self) -> Result<Token, LexingError> {
        let token = self.next_from_context()?;

        self.set_context_from_token(&token);

        Ok(token)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn whitespace() {
        let mut lexer = Lexer::new(String::from("      \n\n     "));

        assert_eq!(Ok(Token::EOF), lexer.next());
    }

    #[test]
    fn single_line_comment() {
        let mut lexer = Lexer::new(String::from("// This is a comment"));

        assert_eq!(Ok(Token::EOF), lexer.next());
    }

    #[test]
    fn single_line_comment_newline() {
        let mut lexer = Lexer::new(String::from("// This is a comment \n 10"));

        assert_eq!(Ok(Token::NumericLiteral(10.0)), lexer.next());
    }

    #[test]
    fn multiline_comment() {
        let input = "/** \
                     This is a multi line doc block **\
                     ** Still going\
                     */";
        let mut lexer = Lexer::new(String::from(input));

        assert_eq!(Ok(Token::EOF), lexer.next());
    }

    #[test]
    fn multiline_comment_continue() {
        let input = "/** \
                     This is a multi line doc block **\
                     ** Still going\
                     */\
                     500";
        let mut lexer = Lexer::new(String::from(input));

        assert_eq!(Ok(Token::NumericLiteral(500.0)), lexer.next());
        assert_eq!(Ok(Token::EOF), lexer.next());
    }

    #[test]
    fn lexer_digit() {
        let mut lexer = Lexer::new(String::from("18"));

        assert_eq!(Token::NumericLiteral(18.0), lexer.next().unwrap());
    }

    #[test]
    fn lexer_decimal_digit() {
        let mut lexer = Lexer::new(String::from("18.123"));

        assert_eq!(Token::NumericLiteral(18.123), lexer.next().unwrap());
    }

    #[test]
    fn lexer_decimal_digit_exponent() {
        let mut lexer = Lexer::new(String::from("18e10"));

        assert_eq!(Token::NumericLiteral(180000000000.0), lexer.next().unwrap());
    }

    #[test]
    fn lexer_digit_signed_exponent() {
        let mut lexer = Lexer::new(String::from("18.12E-5"));

        assert_eq!(Token::NumericLiteral(0.0001812), lexer.next().unwrap());
    }

    #[test]
    fn lexer_digit_big_int() {
        let mut lexer = Lexer::new(String::from("100234n"));

        assert_eq!(
            Token::BigIntLiteral(String::from("100234"), 10),
            lexer.next().unwrap()
        );
    }

    #[test]
    fn lexer_bigint_zero() {
        let mut lexer = Lexer::new(String::from("0n"));

        assert_eq!(
            Ok(Token::BigIntLiteral(String::from("0"), 10)),
            lexer.next()
        )
    }

    #[test]
    fn lexer_digit_binary() {
        let mut lexer = Lexer::new(String::from("0b10"));
        let mut lexer_bigint = Lexer::new(String::from("0b10n"));

        assert_eq!(Token::NumericLiteral(2.0), lexer.next().unwrap());
        assert_eq!(
            Token::BigIntLiteral(String::from("10"), 2),
            lexer_bigint.next().unwrap()
        );
    }

    #[test]
    fn lexer_digit_octal() {
        let mut lexer = Lexer::new(String::from("0o55"));
        let mut lexer_bigint = Lexer::new(String::from("0o55n"));

        assert_eq!(Token::NumericLiteral(45.0), lexer.next().unwrap());
        assert_eq!(
            Token::BigIntLiteral(String::from("55"), 8),
            lexer_bigint.next().unwrap()
        );
    }

    #[test]
    fn lexer_digit_hex() {
        let mut lexer = Lexer::new(String::from("0xFFFF"));
        let mut lexer_bigint = Lexer::new(String::from("0xFFFFn"));

        assert_eq!(Token::NumericLiteral(65535.0), lexer.next().unwrap());
        assert_eq!(
            Token::BigIntLiteral(String::from("FFFF"), 16),
            lexer_bigint.next().unwrap()
        );
    }

    #[test]
    fn lexer_number_invalid_token() {
        let mut lexer_char = Lexer::new(String::from("10i"));
        let mut lexer_digit = Lexer::new(String::from("20n100"));

        assert_eq!(Err(LexingError::InvalidToken), lexer_char.next());
        assert_eq!(Err(LexingError::InvalidToken), lexer_digit.next());
    }

    #[test]
    fn null_literal() {
        let mut lexer = Lexer::new(String::from("null"));

        assert_eq!(Ok(Token::NullLiteral), lexer.next());
    }

    #[test]
    fn boolean_literal() {
        let mut lexer = Lexer::new(String::from("true false"));

        assert_eq!(Ok(Token::BooleanLiteral(true)), lexer.next());
        assert_eq!(Ok(Token::BooleanLiteral(false)), lexer.next());
    }

    #[test]
    fn punctuator() {
        let mut lexer = Lexer::new(String::from("= >>> !=="));

        assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("="))));
        assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from(">>>"))));
        assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("!=="))));
    }

    #[test]
    fn punctuator_greedy_backtrack() {
        let mut lexer = Lexer::new(String::from("<<< -->"));

        assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("<<"))));
        assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("<"))));

        assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("--"))));

        assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from(">"))));
    }

    #[test]
    fn ident_keyword() {
        let mut lexer = Lexer::new(String::from("break do in super this"));

        assert_eq!(lexer.next(), Ok(Token::Keyword(String::from("break"))));
        assert_eq!(lexer.next(), Ok(Token::Keyword(String::from("do"))));
        assert_eq!(lexer.next(), Ok(Token::Keyword(String::from("in"))));
        assert_eq!(lexer.next(), Ok(Token::Keyword(String::from("super"))));
        assert_eq!(lexer.next(), Ok(Token::Keyword(String::from("this"))));
    }

    #[test]
    fn ident_conditional_keyword() {
        let mut lexer = Lexer::new(String::from("as async package yield"));

        assert_eq!(
            lexer.next(),
            Ok(Token::ConditionalKeyword(String::from("as")))
        );
        assert_eq!(
            lexer.next(),
            Ok(Token::ConditionalKeyword(String::from("async")))
        );
        assert_eq!(
            lexer.next(),
            Ok(Token::ConditionalKeyword(String::from("package")))
        );
        assert_eq!(
            lexer.next(),
            Ok(Token::ConditionalKeyword(String::from("yield")))
        );
    }

    #[test]
    fn ident_reserved_keyword() {
        let mut lexer = Lexer::new(String::from("enum"));

        assert_eq!(
            lexer.next(),
            Ok(Token::ReservedKeyword(String::from("enum")))
        );
    }

    #[test]
    fn ident_name() {
        let mut lexer = Lexer::new(String::from(r"x $ _ \uAAFF $x$_\uA38D"));

        assert_eq!(lexer.next(), Ok(Token::IdentifierName(String::from("x"))));
        assert_eq!(lexer.next(), Ok(Token::IdentifierName(String::from("$"))));
        assert_eq!(lexer.next(), Ok(Token::IdentifierName(String::from("_"))));
        assert_eq!(
            lexer.next(),
            Ok(Token::IdentifierName(String::from("\u{aaff}")))
        );
        assert_eq!(
            lexer.next(),
            Ok(Token::IdentifierName(String::from("$x$_ꎍ")))
        );
    }

    #[test]
    fn string_literal_single_quote() {
        let mut lexer = Lexer::new(String::from(r"'abra\tkadabra \n\n hullo\0 \ufa45 \xFA'"));
        let expectation = String::from("abra	kadabra \n\n hullo  海 ú");

        assert_eq!(Ok(Token::StringLiteral(expectation)), lexer.next());
    }

    #[test]
    fn string_literal_double_quote() {
        let mut lexer = Lexer::new(String::from(r#""abra\tkadabra \n\n hullo\0 \ufa45 \xFA""#));
        let expectation = String::from("abra	kadabra \n\n hullo  海 ú");

        assert_eq!(Ok(Token::StringLiteral(expectation)), lexer.next());
    }

    #[test]
    fn fun_happy_path_integration_test() {
        let source = String::from(
            "\
        let x = 'hello';\
        let y = 100;
        const z = 0x10n;
        ",
        );
        let mut lexer = Lexer::new(source);

        assert_eq!(
            lexer.next(),
            Ok(Token::ConditionalKeyword(String::from("let")))
        );
        assert_eq!(lexer.next(), Ok(Token::IdentifierName(String::from("x"))));
        assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("="))));
        assert_eq!(
            lexer.next(),
            Ok(Token::StringLiteral(String::from("hello")))
        );
        assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from(";"))));

        assert_eq!(
            lexer.next(),
            Ok(Token::ConditionalKeyword(String::from("let")))
        );
        assert_eq!(lexer.next(), Ok(Token::IdentifierName(String::from("y"))));
        assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("="))));
        assert_eq!(lexer.next(), Ok(Token::NumericLiteral(100.0)));
        assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from(";"))));

        assert_eq!(lexer.next(), Ok(Token::Keyword(String::from("const"))));
        assert_eq!(lexer.next(), Ok(Token::IdentifierName(String::from("z"))));
        assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from("="))));
        assert_eq!(
            lexer.next(),
            Ok(Token::BigIntLiteral(String::from("10"), 16))
        );
        assert_eq!(lexer.next(), Ok(Token::Punctuator(String::from(";"))));
    }

    #[test]
    fn no_substitution_template() {
        let mut lexer = Lexer::new(String::from(r"`abra\tkadabra \n\n hullo\0 \ufa45 \xFA`"));
        let expectation = String::from("abra	kadabra \n\n hullo  海 ú");

        assert_eq!(Ok(Token::NoSubstitutionTemplate(expectation)), lexer.next());
    }
}
