use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
pub struct WebcConfig {}

impl WebcConfig {
    pub fn init() -> Self {
        if dotenvy::dotenv().is_ok() {
            println!("Loaded .env file");
        }

        Self::init_from_env().expect("WebcConfig init failed")
    }
}
