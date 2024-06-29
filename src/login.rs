use std::collections::HashMap;

use base64::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::Digest;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Auth {
    pub authorization_code: String,
    pub code_verifier: String,
    pub domain: String,
    pub device_serial: String,
}

pub fn external_login(
    country_code: &str,
    domain: String,
    market_place_id: &str,
    device_serial: Option<String>,
    with_username: bool,
    // login_url_callback: Option<impl FnOnce(&str) -> Result<String, Box<dyn std::error::Error>>>,
) -> Result<Auth, Box<dyn std::error::Error>> {
    let (oauth_url, code_verifier, device_serial) = build_oauth_url(
        country_code,
        &domain,
        market_place_id,
        device_serial,
        with_username,
    )?;

    // let response_url = match login_url_callback {
    //     Some(callback) => callback(&oauth_url)?,
    //     None => open_url_and_capture_result(&oauth_url)?,
    // };
    let response_url = open_url_and_capture_result(&oauth_url)?;

    let response_url = url::Url::parse(&response_url)?;
    let query_pairs: HashMap<_, _> = response_url.query_pairs().into_owned().collect();
    let authorization_code = match query_pairs.get("openid.oa2.authorization_code") {
        Some(auth_code) => auth_code.to_string(),
        None => return Err("Authorization code not found in response URL".into()),
    };

    Ok(Auth {
        authorization_code,
        code_verifier,
        domain,
        device_serial,
    })
}

// returns (url, code_verifier, serial)
fn build_oauth_url(
    country_code: &str,
    domain: &str,
    market_place_id: &str,
    device_serial: Option<String>,
    with_username: bool,
) -> Result<(String, String, String), Box<dyn std::error::Error>> {
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

fn open_url_and_capture_result(url: &str) -> Result<String, std::io::Error> {
    // Open the URL in the default web browser
    match webbrowser::open(url) {
        Ok(_) => {
            println!("Opened {} in your default web browser.", url);
            println!("Please log in and copy the resulting URL from your browser's address bar:");

            let term = console::Term::stdout();
            let mut input = String::new();

            loop {
                if let Ok(char) = term.read_char() {
                    match char {
                        '\n' => {
                            println!();
                            break;
                        }
                        c => {
                            if c == '&' || c == '?' {
                                println!();
                            }
                            print!("{c}");
                            input.push(c)
                        }
                    }
                }
            }
            println!();

            Ok(input.trim().to_string())
        }
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to open URL: {}", e),
        )),
    }
}

fn create_code_verifier() -> String {
    let verifier: Vec<u8> = rand::thread_rng().gen::<[u8; 32]>().to_vec();
    BASE64_URL_SAFE_NO_PAD.encode(verifier)
}

fn build_device_serial() -> String {
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

    #[test]
    #[ignore = "opens external browser"]
    fn test_external_login() {
        let locale = crate::localization::find_by_country_code("us").unwrap();
        let auth = external_login(
            &locale.country_code,
            locale.domain.clone(),
            locale.market_place_id.as_str(),
            None,
            false,
        )
        .unwrap();
        dbg!(auth);
    }
}
