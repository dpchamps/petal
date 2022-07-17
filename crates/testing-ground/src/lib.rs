#[macro_use]
extern crate swc_common;
extern crate swc_ecma_parser;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, FilePathMapping, SourceMap,
    comments::{SingleThreadedComments}
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc::{Compiler};

use std::{env, sync::Arc, path::Path};
use std::borrow::Borrow;
use swc::ecmascript::ast::EsVersion;
use swc_common::comments::Comments;
use swc_common::input::SourceFileInput;
use swc_ecma_ast::SourceMapperExt;
use swc_ecma_parser::token::{Token, TokenAndSpan};
use swc_ecma_parser::{Tokens};

fn main() {
    let sm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    println!("Current dir {}", env::current_dir().unwrap().display());
    let source_file = sm.load_file(Path::new(&format!("{}{}", env::current_dir().unwrap().display(), "/tests/fixtures/test"))).expect("oopsie daisys");

    let comments = SingleThreadedComments::default();
    let lexer = Lexer::new(Syntax::Typescript(Default::default()), EsVersion::Es2022, SourceFileInput::from(source_file.borrow()), Some(&comments as &dyn Comments));
    println!("{:#?}", lexer.into_iter().collect::<Vec<TokenAndSpan>>());
}



#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        main();
        panic!();
    }
}
