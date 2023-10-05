use webc_lexer::LexerConfig;

pub fn main() {
    let cfg = LexerConfig::init();

    println!("cfg {:?}", cfg);
}
