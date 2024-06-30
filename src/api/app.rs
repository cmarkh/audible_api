/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/app/upgradestatus
    ///
    /// Query Parameters:
    /// - `version` (string) – [3.68]
    /// - `app_id` (string) – [A2CZJZGLK2JJVM]
    /// - `operating_system` (string) – [iOS15.4]
    pub async fn get_app_upgrade_status(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/app/upgradestatus", self.base_url);

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
