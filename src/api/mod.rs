use reqwest::{Request, Response};

use crate::auth::auth_headers::auth_headers;
use crate::auth::Auth;
use crate::Result;

pub mod library;

const API_URL: &str = "https://api.audible.";

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    auth: Auth,
    base_url: String,
}

impl Client {
    pub fn new(auth: Auth) -> Result<Self> {
        let base_url = format!("{}{}", API_URL, auth.locale.domain);

        let headers = {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
            headers.insert(reqwest::header::ACCEPT_CHARSET, "utf-8".parse().unwrap());
            headers.insert(
                reqwest::header::CONTENT_TYPE,
                "application/json".parse().unwrap(),
            );
            headers.insert(reqwest::header::CONNECTION, "keep-alive".parse().unwrap());
            headers
        };

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()?;

        Ok(Self {
            client,
            auth,
            base_url,
        })
    }

    pub async fn send_request(&self, mut request: Request) -> Result<Response> {
        let body = match request.body() {
            Some(body) => body.as_bytes().unwrap_or_default().to_vec(),
            None => Vec::new(),
        };
        let path = request.url().to_string().replace(&self.base_url, "");
        let auth_headers = auth_headers(
            request.method().as_str(),
            &path,
            &body,
            &self.auth.device_registration.adp_token,
            &self.auth.device_registration.device_private_key,
        )?;
        request.headers_mut().extend(auth_headers);

        Ok(self.client.execute(request).await?)
    }
}
