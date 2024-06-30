/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/account/information
    ///
    /// Query Parameters:
    /// - `response_groups` (string) – [delinquency_status, customer_benefits, customer_segments, subscription_details_payment_instrument, plan_summary, subscription_details, directed_ids]
    /// - `source` (string) – [Credit, Enterprise, RodizioFreeBasic, AyceRomance, AllYouCanEat, AmazonEnglish, ComplimentaryOriginalMemberBenefit, Radio, SpecialBenefit, Rodizio]
    pub async fn get_account_information(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/account/information", self.base_url);

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
