extern crate rst_parser;

use rst_parser::{writers, parser};
use rst_parser::writers::Writer;

macro_rules! check_files {
    ($in_fn:expr, $out_fn:expr) => (
        let input = include_str!($in_fn);
        let output = include_str!($out_fn);
        let ast = parser::document(input.as_bytes()).unwrap().1;
        let html = writers::Html5Writer::translate(ast);
        assert_eq!(html, output);
    )
}

#[test]
fn syntax() {
    check_files!("syntax.rst", "syntax.html");
}
