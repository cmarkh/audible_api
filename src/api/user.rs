/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/user/settings
    ///
    /// Query Parameters:
    /// - `setting_name` (string) – [captionsEnabled]
    pub async fn get_user_settings(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/user/settings", self.base_url);

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
