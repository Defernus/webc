use envconfig::Envconfig;

use crate::{WebcError, WebcResult};

#[derive(Envconfig, Debug)]
pub struct WebcConfig {}

impl WebcConfig {
    pub fn init() -> WebcResult<Self> {
        if dotenvy::dotenv().is_ok() {
            println!("Loaded .env file");
        }

        Self::init_from_env().map_err(|e| WebcError::ConfigParsing(e.to_string()))
    }
}
