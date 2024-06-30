use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::Notify};

use super::register::Registration;
use super::router::REGISTRATIONS;
use crate::auth::oauth::build_device_serial;
use crate::{auth::router::router_with_done_notification, error::Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Auth {
    pub authorization_code: String,
    pub code_verifier: String,
    pub domain: String,
    pub device_serial: String,
}

/// Launches the default browser
/// Servers the Audible sign in page
/// Handles capturing the auth response
pub async fn serve_signin() -> Result<Registration> {
    let listener = TcpListener::bind("localhost:0").await?;

    let device_id = build_device_serial();
    let url = format!(
        "http://{}/audible-signin?country-code=US&device-id={}",
        listener.local_addr()?,
        device_id
    );
    println!("Sign in on {url}");

    let notifier = Arc::new(Notify::new());
    let app = router_with_done_notification(notifier.clone()).await?;

    let handle = tokio::spawn(async move { axum::serve(listener, app.into_make_service()).await });

    webbrowser::open(&url)?;

    notifier.notified().await;
    handle.abort();

    let auth = REGISTRATIONS
        .lock()
        .await
        .remove(&device_id)
        .ok_or("No device registration found")?;

    Ok(auth)
}
