use std::collections::HashMap;

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::auth::oauth::build_client_id;
use crate::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Registration {
    pub device_serial: String,
    pub client_id: String,
    pub adp_token: String,
    pub device_private_key: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires: i64,
    pub website_cookies: HashMap<String, String>,
    pub store_authentication_cookie: String,
    pub device_info: Value,
    pub customer_info: Value,
}

pub async fn register(
    authorization_code: &str,
    code_verifier: &str,
    domain: &str,
    device_serial: &str,
    with_username: bool,
) -> Result<Registration> {
    let client_id = build_client_id(device_serial);

    let body = json!({
        "requested_token_type": [
            "bearer",
            "mac_dms",
            "website_cookies",
            "store_authentication_cookie"
        ],
        "cookies": {
            "website_cookies": [],
            "domain": format!(".amazon.{}", domain)
        },
        "registration_data": {
            "domain": "Device",
            "app_version": "3.56.2",
            "device_serial": device_serial,
            "device_type": "A2CZJZGLK2JJVM",
            "device_name": "%FIRST_NAME%%FIRST_NAME_POSSESSIVE_STRING%%DUPE_STRATEGY_1ST%Audible for iPhone",
            "os_version": "15.0.0",
            "software_version": "35602678",
            "device_model": "iPhone",
            "app_name": "Audible"
        },
        "auth_data": {
            "client_id": client_id,
            "authorization_code": authorization_code,
            "code_verifier": code_verifier,
            "code_algorithm": "SHA-256",
            "client_domain": "DeviceLegacy"
        },
        "requested_extensions": [
            "device_info",
            "customer_info"
        ]
    });

    let target_domain = if with_username { "audible" } else { "amazon" };

    let resp = reqwest::Client::new()
        .post(&format!(
            "https://api.{}.{}/auth/register",
            target_domain, domain
        ))
        .json(&body)
        .send()
        .await?;
    if !resp.status().is_success() {
        return Err(format!("Failed to register: {}", resp.text().await?).into());
    }

    let resp_json: Value = resp.json().await?;
    let success_response = &resp_json["response"]["success"];

    let tokens = &success_response["tokens"];
    let adp_token = tokens["mac_dms"]["adp_token"]
        .as_str()
        .ok_or("Missing adp_token")?
        .to_string();
    let device_private_key = tokens["mac_dms"]["device_private_key"]
        .as_str()
        .ok_or("Missing device_private_key")?
        .to_string();
    let store_authentication_cookie = tokens["store_authentication_cookie"]["cookie"]
        .as_str()
        .ok_or("Missing store_authentication_cookie")?
        .to_string();
    let access_token = tokens["bearer"]["access_token"]
        .as_str()
        .ok_or("Missing access_token")?
        .to_string();
    let refresh_token = tokens["bearer"]["refresh_token"]
        .as_str()
        .ok_or("Missing refresh_token")?
        .to_string();
    let expires_s: i64 = tokens["bearer"]["expires_in"]
        .as_str()
        .ok_or("Missing expires_in")?
        .parse()?;
    let expires = (Utc::now() + Duration::seconds(expires_s)).timestamp();

    let extensions = &success_response["extensions"];
    let device_info = extensions["device_info"].clone();
    let customer_info = extensions["customer_info"].clone();

    let mut website_cookies = HashMap::new();
    if let Some(cookies) = tokens["website_cookies"].as_array() {
        for cookie in cookies {
            if let (Some(name), Some(value)) = (cookie["Name"].as_str(), cookie["Value"].as_str()) {
                website_cookies.insert(name.to_string(), value.replace('"', ""));
            }
        }
    }

    Ok(Registration {
        device_serial: device_serial.to_string(),
        client_id,
        adp_token,
        device_private_key,
        access_token,
        refresh_token,
        expires,
        website_cookies,
        store_authentication_cookie,
        device_info,
        customer_info,
    })
}
