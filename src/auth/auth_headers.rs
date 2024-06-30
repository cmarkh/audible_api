use base64::prelude::*;
use reqwest::header::HeaderMap;
use rsa::{pkcs1::DecodeRsaPrivateKey, sha2::Sha256, Pkcs1v15Sign};
use sha2::Digest;

use crate::Result;

pub fn auth_headers(
    method: &str,
    path: &str,
    body: &[u8],
    adp_token: &str,
    device_private_key: &str,
) -> Result<HeaderMap> {
    let date = chrono::Utc::now().to_rfc3339() + "Z";
    let str_body = std::str::from_utf8(body).unwrap();
    let data = format!(
        "{}\n{}\n{}\n{}\n{}",
        method, path, date, str_body, adp_token
    );

    let key = rsa::RsaPrivateKey::from_pkcs1_pem(device_private_key)?;
    let hashed = Sha256::digest(data.as_bytes());
    let padding_scheme = Pkcs1v15Sign::new::<Sha256>();
    let signature = key.sign(padding_scheme, &hashed)?;
    let signed_encoded = BASE64_STANDARD.encode(signature);
    let signature = format!("{}:{}", signed_encoded, date);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("x-adp-token", adp_token.parse()?);
    headers.insert("x-adp-alg", "SHA256withRSA:1.0".parse()?);
    headers.insert("x-adp-signature", signature.parse()?);

    Ok(headers)
}
