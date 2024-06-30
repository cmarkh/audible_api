use base64::prelude::*;
use register::Registration;
use rsa::{pkcs1::DecodeRsaPrivateKey, sha2::Sha256, Pkcs1v15Sign};
use serde::{Deserialize, Serialize};
use sha2::Digest;

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

    pub fn sign_request(
        &self,
        method: &str,
        path: &str,
        body: &[u8],
    ) -> Result<reqwest::header::HeaderMap> {
        let date = chrono::Utc::now().to_rfc3339() + "Z";

        let signature = signature(
            method,
            path,
            body,
            &self.device_registration.adp_token,
            &self.device_registration.device_private_key,
            &date,
        )?;

        let headers = {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("x-adp-token", self.device_registration.adp_token.parse()?);
            headers.insert("x-adp-alg", "SHA256withRSA:1.0".parse()?);
            headers.insert("x-adp-signature", signature.parse()?);
            headers
        };

        // let headers = {
        //     let mut headers = reqwest::header::HeaderMap::new();
        //     headers.insert("x-adp-token", self.device_registration.adp_token.parse()?);
        //     headers.insert("x-adp-alg", "SHA256withRSA:1.0".parse()?);
        //     headers.insert("x-adp-signature", "EPzbuGjvJ7yfh4+GdkFec1xSBG3uYq5fU8qVYidq/JJYXqja9NIt01WYHKl2oIBMr8QNj//JYWtncHOi8y9ZXaUofSu7WAlbTToITdndumfCkygrjVNuZbvTqI8/ocgfPPHezRfg1hT1PzFPq7EJMDWbcPPkdMTh3WzFqow53+Rzey1BBXZko7FTsk7RUmIhbhfjySsEgD4xIswGiwIggNzZgK0kyp79x2wghrPjKQ9WDDiQjfcQV8dxh9e7sNnglMb4k/4sL9ChaCkp7V2KVRvxhfDW8YKREPDrdNGap/L05y41HqGx2WFZDTuq0tfwN/oZUeMOY4EJfzFPALqBqQ==:2024-06-30T17:33:33.610470+00:00Z".parse()?);
        //     headers
        // };

        Ok(headers)
    }
}

pub fn signature(
    method: &str,
    path: &str,
    body: &[u8],
    adp_token: &str,
    device_private_key: &str,
    date: &str,
) -> Result<String> {
    let str_body = std::str::from_utf8(body).unwrap();
    let data = format!(
        "{}\n{}\n{}\n{}\n{}",
        method, path, date, str_body, adp_token
    );
    println!("{}\n", data);

    let key = rsa::RsaPrivateKey::from_pkcs1_pem(device_private_key)?;
    dbg!(&key);
    let hashed = Sha256::digest(data.as_bytes());
    let padding_scheme = Pkcs1v15Sign::new::<Sha256>();
    let signature = key.sign(padding_scheme, &hashed)?;
    let signed_encoded = BASE64_STANDARD.encode(signature);
    let signature = format!("{}:{}", signed_encoded, date);

    Ok(signature)
}
