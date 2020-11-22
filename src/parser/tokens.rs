#[derive(Debug, Clone, PartialOrd, PartialEq)]
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
    NoSubstitutionTemplate(String),
    TemplateHead(String),
    TemplateMiddle(String),
    TemplateTail(String),
    RegexLiteral(String, String),
    EOF,
}
