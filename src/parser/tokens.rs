#[derive(Debug, PartialOrd, PartialEq)]
pub enum Token {
    IdentifierName(String),
    Punctuator(String),
    NullLiteral,
    BooleanLiteral(bool),
    NumericLiteral(f64),
    BigIntLiteral(String, u32),
    StringLiteral(String),
    Keyword(String),
    ConditionalKeyword(String),
    ReservedKeyword(String),
    EOF,
}
