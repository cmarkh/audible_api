/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/orders
    /// Returns order history from at least the past 6 months. Supports pagination.
    ///
    /// Query Parameters:
    /// - `unknown` (object) – The structure of the query parameters is not specified
    pub async fn get_orders(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/orders", self.base_url);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// POST /1.0/orders
    ///
    /// Request JSON Object:
    /// - `asin` (string)
    /// - `audiblecreditapplied` (boolean) – Will specify whether to use available credits or default payment method.
    pub async fn post_orders(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/orders", self.base_url);

        let mut req = self.client.post(url);
        if let Some(params) = params {
            req = req.json(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }
}
