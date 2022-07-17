use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use pretty_assertions::assert_eq;
use swc_common::{errors::Handler, sync::Lrc, SourceMap};
use swc_ecma_ast::*;
use swc_petal_parser::{lexer::Lexer, PResult, Parser, StringInput};
use swc_ecma_visit::{Fold, FoldWith};
use testing::{run_test, StdErr};

fn parse_module(cm: Lrc<SourceMap>, handler: &Handler, file_name: &Path) -> Result<Module, ()> {
    with_parser(cm, handler, file_name, |p| p.parse_module())
}

fn with_parser<F, Ret>(
    cm: Lrc<SourceMap>,
    handler: &Handler,
    file_name: &Path,
    f: F,
) -> Result<Ret, ()>
where
    F: FnOnce(&mut Parser<Lexer<StringInput<'_>>>) -> PResult<Ret>,
{
    let fm = cm
        .load_file(file_name)
        .unwrap_or_else(|e| panic!("failed to load {}: {}", file_name.display(), e));

    let mut p = Parser::new(
        ::swc_petal_parser::Syntax::Es(::swc_petal_parser::EsConfig {
            jsx: true,
            ..Default::default()
        }),
        (&*fm).into(),
        None,
    );

    let res = f(&mut p).map_err(|e| e.into_diagnostic(handler).emit());

    for e in p.take_errors() {
        e.into_diagnostic(handler).emit();
    }

    res
}

#[testing::fixture("tests/jsx/basic/**/*.js")]
fn references(entry: PathBuf) {
    run_test(false, |cm, handler| {
        let input = read_to_string(&entry).unwrap();

        eprintln!(
            "\n\n========== Running reference test \nSource:\n{}\n",
            input
        );

        // Parse source
        let module = parse_module(cm, handler, &entry)?.fold_with(&mut Normalizer);
        let json =
            serde_json::to_string_pretty(&module).expect("failed to serialize module as json");
        if StdErr::from(json.clone())
            .compare_to_file(format!("{}.json", entry.display()))
            .is_err()
        {
            panic!()
        }

        let deser = serde_json::from_str::<Module>(&json)
            .unwrap_or_else(|err| {
                panic!(
                    "failed to deserialize json back to module: {}\n{}",
                    err, json
                )
            })
            .fold_with(&mut Normalizer);
        assert_eq!(module, deser, "JSON:\n{}", json);

        Ok(())
    })
    .unwrap();
}

#[cfg(feature = "verify")]
#[testing::fixture("tests/jsx/errors/**/*.js")]
fn error(entry: PathBuf) {
    let input = read_to_string(&entry).unwrap();

    eprintln!(
        "\n\n========== Running error reporting test \nSource:\n{}\n",
        input
    );

    let err = run_test(false, |cm, handler| {
        if false {
            // Type annotation
            return Ok(());
        }

        // Parse source
        let _ = parse_module(cm, handler, &entry);
        if !handler.has_errors() {
            panic!("should emit error, but parsed without error")
        }

        Err(())
    })
    .expect_err("should fail, but parsed as");

    if err
        .compare_to_file(format!("{}.stderr", entry.display()))
        .is_err()
    {
        panic!()
    }
}

struct Normalizer;

impl Fold for Normalizer {
    fn fold_pat(&mut self, mut node: Pat) -> Pat {
        node = node.fold_children_with(self);

        if let Pat::Expr(expr) = node {
            match *expr {
                Expr::Ident(i) => return Pat::Ident(i.into()),
                _ => {
                    node = Pat::Expr(expr);
                }
            }
        }

        node
    }

    fn fold_pat_or_expr(&mut self, node: PatOrExpr) -> PatOrExpr {
        let node = node.fold_children_with(self);

        match node {
            PatOrExpr::Pat(pat) => match *pat {
                Pat::Expr(expr) => PatOrExpr::Expr(expr),
                _ => PatOrExpr::Pat(pat),
            },
            _ => node,
        }
    }
}
