use std::error::Error;

#[derive(Clone)]
pub struct Config;

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        Ok(Config)
    }
}
