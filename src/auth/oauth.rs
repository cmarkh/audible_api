use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use sha2::Digest;

use crate::error::Result;

// returns (url, code_verifier, serial)
pub fn build_oauth_url(
    country_code: &str,
    domain: &str,
    market_place_id: &str,
    device_serial: Option<String>,
    with_username: bool,
) -> Result<(String, String, String)> {
    if with_username && !["de", "com", "co.uk"].contains(&domain.to_lowercase().as_str()) {
        return Err("Username is not supported for this domain".into());
    }
    let code_verifier = create_code_verifier();
    let serial = match device_serial {
        Some(device_serial) => device_serial,
        None => build_device_serial(),
    };
    let client_id = build_client_id(&serial);
    let code_challenge = create_s256_code_challenge(code_verifier.as_bytes());

    let mut base_url = format!("https://www.amazon.{domain}/ap/signin");
    let mut return_to = format!("https://www.amazon.{domain}/ap/maplanding");
    let mut assoc_handle = format!("amzn_audible_ios_{country_code}");
    let mut page_id = "amzn_audible_ios";

    if with_username {
        base_url = format!("https://www.audible.{domain}/ap/signin");
        return_to = format!("https://www.audible.{domain}/ap/maplanding");
        assoc_handle = format!("amzn_audible_ios_lap_{country_code}");
        page_id = "amzn_audible_ios_privatepool";
    }

    let oauth_params = [
        ("openid.oa2.response_type", "code"),
        ("openid.oa2.code_challenge_method", "S256"),
        ("openid.oa2.code_challenge", &code_challenge),
        ("openid.return_to", &return_to),
        ("openid.assoc_handle", &assoc_handle),
        (
            "openid.identity",
            "http://specs.openid.net/auth/2.0/identifier_select",
        ),
        ("pageId", &page_id),
        ("accountStatusPolicy", "P1"),
        (
            "openid.claimed_id",
            "http://specs.openid.net/auth/2.0/identifier_select",
        ),
        ("openid.mode", "checkid_setup"),
        ("openid.ns.oa2", "http://www.amazon.com/ap/ext/oauth/2"),
        ("openid.oa2.client_id", &format!("device:{}", client_id)),
        (
            "openid.ns.pape",
            "http://specs.openid.net/extensions/pape/1.0",
        ),
        ("marketPlaceId", market_place_id),
        ("openid.oa2.scope", "device_auth_access"),
        ("forceMobileLayout", "true"),
        ("openid.ns", "http://specs.openid.net/auth/2.0"),
        ("openid.pape.max_auth_age", "0"),
    ];

    let encoded_params: String = url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(oauth_params)
        .finish();
    let url = format!("{}?{}", base_url, encoded_params);

    Ok((url, code_verifier, serial))
}

fn create_code_verifier() -> String {
    let verifier: Vec<u8> = rand::thread_rng().gen::<[u8; 32]>().to_vec();
    BASE64_URL_SAFE_NO_PAD.encode(verifier)
}

pub fn build_device_serial() -> String {
    uuid::Uuid::new_v4()
        .to_string()
        .replace('-', "")
        .to_uppercase()
}

pub fn build_client_id(serial: &str) -> String {
    let mut client_id_bytes = serial.as_bytes().to_vec();
    client_id_bytes.extend_from_slice(b"#A2CZJZGLK2JJVM");
    hex::encode(client_id_bytes)
}

fn create_s256_code_challenge(verifier: &[u8]) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(verifier);
    let digest = hasher.finalize();
    BASE64_URL_SAFE_NO_PAD.encode(digest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_code_verifier() {
        let verifier = create_code_verifier();
        assert_eq!(verifier.len(), 43);
        dbg!(verifier);
    }

    #[test]
    fn test_build_oauth_url() {
        let (url, _, _) = build_oauth_url("us", "com", "ATVPDKIKX0DER", None, false)
            .expect("Failed to build OAuth URL");
        dbg!(url);
    }
}
