use register::Registration;
use serde::{Deserialize, Serialize};

use crate::Result;
use localization::Locale;
use sign_in::sign_in;

pub mod localization;
pub mod oauth;
pub mod register;
pub mod sign_in;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub locale: Locale,
    pub device_registration: Registration,
    pub authorization_code: String,
    pub code_verifier: String,
}

impl Auth {
    pub async fn default(country_code: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if let Ok(auth) = Auth::from_file("auth.json") {
            return Ok(auth);
        }
        let auth = sign_in(country_code, None, false, None).await?;
        auth.to_file("auth.json")?;
        Ok(auth)
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let auth = serde_json::from_reader(reader)?;
        Ok(auth)
    }

    pub fn to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    pub async fn from_sign_in() -> Result<Self> {
        let auth = sign_in("us", None, false, None).await?;
        Ok(auth)
    }
}
