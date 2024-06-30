use std::collections::HashMap;
use std::sync::Arc;

use askama::Template;
use axum::extract::Query;
use axum::response::Redirect;
use axum::routing::post;
use axum::Json;
use axum::{routing::get, Extension, Router};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tokio::sync::Notify;

use crate::db::auth::PartialDevice;
use crate::error::Result;

use super::oauth::build_oauth_url;
use super::register::{register, Registration};

lazy_static::lazy_static! {
    static ref DEVICES : Mutex<HashMap<String, PartialDevice>> = Mutex::new(HashMap::new());
    pub static ref REGISTRATIONS : Mutex<HashMap<String, Registration>> = Mutex::new(HashMap::new());
}

pub async fn router() -> Result<Router> {
    Ok(Router::new()
        .route("/audible-signin", get(audible_signin))
        .route("/audible-signin/capture", post(audible_capture))
        .route("/audible-signin/success", get(success_page)))
}

pub async fn router_with_done_notification(notifier: Arc<Notify>) -> Result<Router> {
    Ok(Router::new()
        .route("/audible-signin", get(audible_signin))
        .route("/audible-signin/capture", post(audible_capture))
        .route(
            "/audible-signin/success",
            get(success_page_with_sender).layer(Extension(notifier.clone())),
        ))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct AudibleSignInQuery {
    country_code: String,
    device_id: Option<String>,
}

#[derive(Template)]
#[template(path = "audible_signin/signin.html")]
struct AudibleSignIn {
    oauth_url: String,
}

async fn audible_signin(
    // Extension(pool): Extension<Pool>,
    Query(query): Query<AudibleSignInQuery>,
) -> Result<AudibleSignIn> {
    let locale = crate::localization::find_by_country_code(&query.country_code)
        .ok_or("Country code not found")?;
    let (oauth_url, code_verifier, device_serial) = build_oauth_url(
        &locale.country_code,
        &locale.domain,
        &locale.market_place_id,
        query.device_id,
        false,
    )?;

    let device = PartialDevice {
        device_serial: device_serial.clone(),
        code_verifier,
        oauth_url: oauth_url.clone(),
        country_code: locale.country_code,
        domain: locale.domain,
        market_place_id: locale.market_place_id,
    };
    DEVICES.lock().await.insert(device_serial, device);

    // let conn = pool.get()?;
    // db::auth::insert_device(&conn, &device_serial, &code_verifier, &oauth_url)?;

    dbg!(&oauth_url);

    Ok(AudibleSignIn { oauth_url })
}

#[derive(Serialize, Deserialize)]
struct AudibleCapture {
    response_url: String,
}

async fn audible_capture(
    // Extension(pool): Extension<Pool>,
    Query(device_id): Query<String>,
    Json(captured): Json<AudibleCapture>,
) -> Result<Redirect> {
    let response_url = url::Url::parse(&captured.response_url)?;
    let query_pairs: HashMap<_, _> = response_url.query_pairs().into_owned().collect();

    let authorization_code = match query_pairs.get("openid.oa2.authorization_code") {
        Some(auth_code) => auth_code.to_string(),
        None => return Err("Authorization code not found in response URL".into()),
    };

    let device = DEVICES
        .lock()
        .await
        .remove(&device_id)
        .ok_or("Device not found")?;

    let device_registration = register(
        &authorization_code,
        &device.code_verifier,
        &device.domain,
        &device.device_serial,
        false,
    )
    .await?;

    REGISTRATIONS
        .lock()
        .await
        .insert(device.device_serial.clone(), device_registration);

    Ok(Redirect::to("/signin/success"))
}

#[derive(Template)]
#[template(path = "audible_signin/success.html")]
struct Success {}

async fn success_page() -> Success {
    Success {}
}

async fn success_page_with_sender(Extension(done): Extension<Arc<Notify>>) -> Success {
    done.notify_waiters();
    Success {}
}
