use std::path::Path;

use webc::{ast::WebcAst, config::WebcConfig, lexer::source::WebcSource};

pub fn main() {
    let cfg = WebcConfig::init().unwrap();

    let path = Path::new(&cfg.main_file);

    let src = WebcSource::from_file(path).unwrap();

    let tokens = src.parse().unwrap();

    let ast = WebcAst::parse(tokens).unwrap();

    println!("{:?}", ast);

    todo!();
}
