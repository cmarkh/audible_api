/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/customer/information
    ///
    /// Query Parameters:
    /// - `response_groups` (string) – [migration_details, subscription_details_rodizio, subscription_details_premium, customer_segment, subscription_details_channels]
    pub async fn get_customer_information(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/customer/information", self.base_url);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/customer/status
    ///
    /// Query Parameters:
    /// - `response_groups` (string) – [benefits_status, member_giving_status, prime_benefits_status, prospect_benefits_status]
    pub async fn get_customer_status(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/customer/status", self.base_url);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/customer/freetrial/eligibility
    pub async fn get_customer_freetrial_eligibility(&self) -> Result<Value> {
        let url = format!("{}/1.0/customer/freetrial/eligibility", self.base_url);

        let req = self.client.get(url).build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }
}
