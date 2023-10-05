use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
pub struct LexerConfig {}

impl LexerConfig {
    pub fn init() -> Self {
        if dotenvy::dotenv().is_ok() {
            println!("Loaded .env file");
        }

        Self::init_from_env().expect("LexerConfig init failed")
    }
}
