use swc_common::{comments::SingleThreadedComments, FileName, FilePathMapping, SourceMap};
// use swc_petal_parser::{
//     lexer::Lexer,
//     token::{Token, TokenAndSpan},
//     Parser, StringInput, Syntax, Tokens,
// };

use std::borrow::Borrow;
use std::{env, path::Path, sync::Arc};
use swc_common::comments::Comments;
use swc_common::input::SourceFileInput;
use swc_petal_ast::{EsVersion, SourceMapperExt};

fn main() {
    // let sm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    // println!("Current dir {}", env::current_dir().unwrap().display());
    // let source_file = sm
    //     .load_file(Path::new(&format!(
    //         "{}{}",
    //         env::current_dir().unwrap().display(),
    //         "/tests/fixtures/test"
    //     )))
    //     .expect("oopsie daisys");
    //
    // let comments = SingleThreadedComments::default();
    // let lexer = Lexer::new(
    //     Syntax::EsTypeAnnotations(Default::default()),
    //     EsVersion::Es2022,
    //     SourceFileInput::from(source_file.borrow()),
    //     Some(&comments as &dyn Comments),
    // );
    // // println!("{:#?}", lexer.into_iter().collect::<Vec<TokenAndSpan>>());
    // let mut parser = Parser::new_from(lexer);
    // println!("{:#?}", parser.parse_module());
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
