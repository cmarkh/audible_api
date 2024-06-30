/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/pages/(string:param1)
    ///
    /// Parameters:
    /// - `param1` (string) – [ios-app-home]
    ///
    /// Query Parameters:
    /// - `image_dpi` (integer) – [489]
    /// - `local_time` (string) – [2022-01-01T12:00:00+01:00]
    /// - `locale` (string) – en-US
    /// - `os` (string) – [15.2]
    /// - `reviews_num_results` (integer)
    /// - `reviews_sort_by` (string)
    /// - `response_groups` (string) – [media, product_plans, view, product_attrs, contributors, product_desc, sample]
    /// - `session_id` (string) – [123-1234567-1234567]
    /// - `surface` (string) – [iOS]
    pub async fn get_pages_param1(&self, param1: &str, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/pages/{}", self.base_url, param1);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }}