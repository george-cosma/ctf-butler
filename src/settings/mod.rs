pub mod constants;
pub use self::constants::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSettings {
    token: String,
}

impl UserSettings {
    pub fn load(path: &str) -> Result<UserSettings, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let settings = serde_json::from_reader(file)?;
        Ok(settings)
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}
