/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/recommendations
    ///
    /// Query Parameters:
    /// - `category_image_variants` (string)
    /// - `image_dpi` (integer)
    /// - `image_sizes` (string)
    /// - `in_plan_timestamp` (string)
    /// - `language` (string)
    /// - `not_in_plan_timestamp` (string)
    /// - `num_results` (integer) – (max: 50)
    /// - `plan` (string) – [Enterprise, RodizioFreeBasic, AyceRomance, AllYouCanEat, AmazonEnglish, ComplimentaryOriginalMemberBenefit, Radio, SpecialBenefit, Rodizio]
    /// - `response_groups` (string) – [contributors, media, price, product_attrs, product_desc, product_extended_attrs, product_plan_details, product_plans, rating, sample, sku]
    /// - `reviews_num_results` (integer) – (max: 10)
    /// - `reviews_sort_by` (string) – [MostHelpful, MostRecent]
    pub async fn get_recommendations(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/recommendations", self.base_url);

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
