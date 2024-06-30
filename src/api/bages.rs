/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/badges/progress
    ///
    /// Query Parameters:
    /// - `locale` (string) – en_US
    /// - `response_groups` (string) – brag_message
    pub async fn get_badges_progress(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/badges/progress", self.base_url);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/badges/metadata
    ///
    /// Query Parameters:
    /// - `locale` (string) – en_US
    /// - `response_groups` (string) – all_levels_metadata
    pub async fn get_badges_metadata(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/badges/metadata", self.base_url);

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
