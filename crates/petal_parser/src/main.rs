use crate::parser::Parser;
use rslint_lexer::Lexer;

pub mod parser;
fn main() {
    let _javascript = r#"
        const x = 1;
        const y = "Hello";
        const z = x + y;
    "#;

    let typed_javascript = r#"
        export {A as B};
    "#;

    // let result = Parser::parse(typed_javascript);

    // println!("{:?}", result);

    let (_tokens, _) =
        Lexer::from_str(typed_javascript, 0).fold((vec![], vec![]), |mut acc, (token, error)| {
            println!("{:?}", token);
            acc.0.push(token);
            if let Some(error) = error {
                acc.1.push(error)
            }
            acc
        });

    let mut parser = Parser::new(typed_javascript);
    let ident = parser.parse_ident();

    println!("{:?}", ident);
}
