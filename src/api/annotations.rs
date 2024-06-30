/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/annotations/lastpositions
    ///
    /// Query Parameters:
    /// - `asins` (string) – ASINs (comma-separated), e.g. ?asins=B01LWUJKQ7,B01LWUJKQ7,B01LWUJKQ7
    pub async fn get_annotations_lastpositions(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/annotations/lastpositions", self.base_url);

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
