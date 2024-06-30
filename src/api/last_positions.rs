/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// PUT /1.0/lastpositions/(string:asin)
    ///
    /// Parameters:
    /// - `asin` (string) – The ASIN of the book
    ///
    /// Request JSON Object:
    /// - `acr` (string) – Obtained by POST /1.0/content/(string:asin)/licenserequest
    /// - `asin` (string)
    /// - `position_ms` (integer)
    pub async fn put_lastpositions_asin(&self, asin: &str, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/lastpositions/{}", self.base_url, asin);

        let mut req = self.client.put(url);
        if let Some(params) = params {
            req = req.json(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }
}
