use serde::{Deserialize, Serialize};

use crate::{
    login::{self, external_login},
    register::{self, register},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub locale: crate::localization::Locale,
    pub auth: login::Auth,
    pub device_registration: register::Registration,
}

impl Auth {
    pub async fn default(country_code: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if let Ok(auth) = Auth::from_file("auth.json") {
            return Ok(auth);
        }
        let auth = Auth::from_external_login(country_code).await?;
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

    pub async fn from_external_login(
        country_code: &str,
    ) -> Result<Auth, Box<dyn std::error::Error>> {
        let locale = crate::localization::find_by_country_code(country_code).unwrap();
        let auth = external_login(
            &locale.country_code,
            locale.domain.clone(),
            locale.market_place_id.as_str(),
            None,
            false,
        )?;
        let device_registration = register(
            &auth.authorization_code,
            &auth.code_verifier,
            &locale.domain,
            &auth.device_serial,
            false,
        )
        .await?;

        Ok(Auth {
            locale,
            auth,
            device_registration,
        })
    }
}
