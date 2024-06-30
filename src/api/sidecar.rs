/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET https://cde-ta-g7g.amazon.com/FionaCDEServiceEngine/sidecar
    /// Returns the clips, notes, and bookmarks of a book
    ///
    /// Query Parameters:
    /// - `type` (string) – ["AUDI"]
    /// - `key` (string) – ASIN of the book
    pub async fn get_sidecar(&self, params: Option<Value>) -> Result<Value> {
        let url = "https://cde-ta-g7g.amazon.com/FionaCDEServiceEngine/sidecar";

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }
}
