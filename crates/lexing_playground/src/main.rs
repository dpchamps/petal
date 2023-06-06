use crate::parser::Parser;
use rslint_lexer::{Lexer, SyntaxKind};
use rslint_parser::TokenSource;

pub mod parser;
fn main() {
    let javascript = r#"
        const x = 1;
        const y = "Hello";
        const z = x + y;
    "#;

    let typed_javascript = r#"
        x
        import Type from "hello";
        import * as T from "hype";
        export const xport = "y";

        const x: number = 1;
        const y: string = "Hello";
        const z = x+y;
    "#;

    // let result = Parser::parse(typed_javascript);

    // println!("{:?}", result);

    let (tokens, _) = Lexer::from_str(typed_javascript, 0).into_iter().fold(
        (vec![], vec![]),
        |mut acc, (token, error)| {
            println!("{:?}", token);
            acc.0.push(token);
            if let Some(error) = error {
                acc.1.push(error)
            }
            acc
        },
    );

    let mut parser = Parser::new(typed_javascript);
    let ident = parser.parse_ident();

    println!("{:?}", ident);
}
