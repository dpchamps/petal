use std::u64;
pub fn to_code_point(c: &char) -> u32 {
    *c as u32
}

pub fn from_code_point(d: u8) -> char { d as char }

pub fn is_whitespace(c: &char) -> bool {
    match to_code_point(c) {
        0x9 | 0xB | 0xC | 0x20 | 0xA0 | 0xFEFF | 0x1680 | 0x202F | 0x205F | 0x3000 => true,
        x if x >= 0x2000 && x <= 0x200A => true,
        _ => false,
    }
}

pub fn is_line_terminator(c: &char) -> bool {
    match to_code_point(c) {
        0xA | 0xD | 0x2028 | 0x2029 => true,
        _ => false,
    }
}

pub fn is_separator(c: &char) -> bool {
    match to_code_point(c) {
        2028 | 2029 => true,
        _ => false,
    }
}

pub fn is_zwnj(c: &char) -> bool {
    to_code_point(c) == 0x200C
}

pub fn is_zwj(c: &char) -> bool {
    to_code_point(c) == 0x200D
}

pub fn is_valid_punctuator_char(c: &char) -> bool {
    match c {
        '{' | '(' | ')' | '[' | ']' | '.' | ';' | ',' | '<' | '>' | '=' | '!' | '+' | '-' | '*'
        | '%' | '&' | '|' | '^' | '~' | '?' | ':' => true,
        _ => false,
    }
}

pub fn is_valid_punctuator(p: &str) -> bool {
    match p {
        "?." | "{" | "(" | ")" | "[" | "]" | "." | "..." | ";" | "," | "<" | ">" | "<=" | ">="
        | "==" | "!=" | "===" | "!==" | "+" | "-" | "*" | "%" | "++" | "--" | "<<" | ">>"
        | ">>>" | "&" | "|" | "^" | "!" | "~" | "&&" | "||" | "?" | ":" | "=" | "+=" | "-="
        | "*=" | "%=" | "<<=" | ">>=" | ">>>=" | "&=" | "|=" | "^=" | "=>" => true,
        _ => false,
    }
}

pub fn is_keyword(ident: &str) -> bool {
    match ident {
        "break" | "do" | "in" | "typeof" | "case" | "else" | "instanceof" | "var" | "catch"
        | "export" | "new" | "void" | "class" | "extends" | "return" | "while" | "const"
        | "finally" | "super" | "with" | "continue" | "for" | "switch" | "debugger"
        | "function" | "this" | "" | "default" | "if" | "throw" | "delete" | "import" | "try" => {
            true
        }
        _ => false,
    }
}

pub fn is_conditional_keyword(ident: &str) -> bool {
    match ident {
        "as" | "async" | "from" | "get" | "of" | "set" | "target" | "let" | "static"
        | "implements" | "interface" | "package" | "private" | "protected" | "public" | "await"
        | "yield" => true,
        _ => false,
    }
}

pub fn is_reserved_word(ident: &str) -> bool {
    match ident {
        "enum" => true,
        _ => false,
    }
}

pub fn is_decimal_digit(c: &char) -> bool {
    match c.to_digit(10) {
        Some(x) if x >= 0 && x <= 9 => true,
        _ => false,
    }
}

pub fn is_single_escape_character(c: &char) -> bool {
    match c {
        '\'' | '"' | '\\' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' => true,
        _ => false,
    }
}

pub fn parse_unicode_to_string(input: &str) -> Result<String, ()> {
    if let Ok(parsed) = u64::from_str_radix(input, 16){
        if let Ok(utf16String) = String::from_utf16(&[parsed as u16]) {
            return Ok(utf16String)
        }
    }

    Err(())
}

pub fn parse_utf8_to_string(input: &str) -> Result<String, ()> {
    if let Ok(parsed) = u64::from_str_radix(input, 16){
        return Ok((parsed as u8 as char).to_string())
    }

    Err(())
}