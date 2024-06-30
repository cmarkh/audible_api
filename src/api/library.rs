/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use json_value_merge::Merge;
use serde_json::{json, Value};

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/library
    ///
    /// The audible library of current user
    ///
    /// Query Parameters:
    /// - `num_results` (integer) – (max: 1000)
    /// - `page` (integer) – page
    /// - `purchased_after` (string) – [RFC3339](https://tools.ietf.org/html/rfc3339) (e.g. 2000-01-01T00:00:00Z)
    /// - `title` (string) – a title
    /// - `author` (string) – an author
    /// - `response_groups` (string) – [contributors, customer_rights, media, price, product_attrs, product_desc,
    ///   product_details, product_extended_attrs, product_plan_details, product_plans, rating, sample, sku, series,
    ///   reviews, ws4v, origin, relationships, review_attrs, categories, badge_types, category_ladders, claim_code_url,
    ///   in_wishlist, is_archived, is_downloaded, is_finished, is_playable, is_removable, is_returnable, is_visible,
    ///   listening_status, order_details, origin_asin, pdf_url, percent_complete, periodicals, provided_review]
    /// - `image_sizes` (string) – 1215,408,360,882,315,570,252,558,900,500
    /// - `sort_by` (string) – [-Author, -Length, -Narrator, -PurchaseDate, -Title, Author, Length, Narrator, PurchaseDate, Title]
    /// - `status` (string) – [Active, Revoked] (‘Active’ is the default, ‘Revoked’ returns audiobooks the user has returned for a refund.)
    /// - `parent_asin` (string) – asin
    /// - `include_pending` (string) – [true, false]
    /// - `marketplace` (string) – [e.g. AN7V1F1VY261K]
    /// - `state_token` (string)
    pub async fn get_library(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/library", self.base_url);

        let mut query = json! {{
                "num_results": 1000,
                "response_groups": "product_desc,product_attrs",
                "sort_by": "-PurchaseDate"
        }};
        if let Some(params) = params {
            query.merge(&params);
        }
        let req = self.client.get(url).query(&query).build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/library/(string:asin)
    ///
    /// Parameters:
    /// - `asin` (string) – The ASIN of the book
    ///
    /// Query Parameters:
    /// - `response_groups` (string) – [contributors, media, price, product_attrs, product_desc, product_details,
    ///   product_extended_attrs, product_plan_details, product_plans, rating, sample, sku, series, reviews, ws4v,
    ///   origin, relationships, review_attrs, categories, badge_types, category_ladders, claim_code_url,
    ///   is_downloaded, is_finished, is_returnable, origin_asin, pdf_url, percent_complete, periodicals, provided_review]
    pub async fn get_library_item_by_asin(
        &self,
        asin: &str,
        params: Option<Value>,
    ) -> Result<Value> {
        let url = format!("{}/1.0/library/{}", self.base_url, asin);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// POST /1.0/library/item
    ///
    /// Request JSON Object:
    /// - `asin` (string) – The ASIN of the book
    pub async fn post_library_item_by_asin(
        &self,
        asin: &str,
        params: Option<Value>,
    ) -> Result<Value> {
        let url = format!("{}/1.0/library/item", self.base_url);

        let mut json = json! {{
                "asin": asin,
        }};
        if let Some(params) = params {
            json.merge(&params);
        }
        let req = self.client.post(url).json(&json).build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// PUT /1.0/library/item
    /// Add an (AYCL) item to the library
    ///
    /// Request JSON Object:
    /// - `asin` (string) – The ASIN of the book
    pub async fn put_library_item_by_asin(
        &self,
        asin: &str,
        params: Option<Value>,
    ) -> Result<Value> {
        let url = format!("{}/1.0/library/item", self.base_url);

        let mut json = json! {{
                "asin": asin,
        }};
        if let Some(params) = params {
            json.merge(&params);
        }
        let req = self.client.put(url).query(&json).build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// POST /1.0/library/item/(param1)/(param2)
    ///
    /// Parameters:
    /// - `param1` (string)
    /// - `param2` (string)
    ///
    /// Request JSON Object:
    /// - `unknown` (object) – The structure of the JSON object is not specified
    pub async fn post_library_item_with_two_params(
        &self,
        param1: &str,
        param2: &str,
        params: Option<Value>,
    ) -> Result<Value> {
        let url = format!("{}/1.0/library/item/{}/{}", self.base_url, param1, param2);

        let mut req = self.client.post(url);
        if let Some(params) = params {
            req = req.json(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// POST /1.0/library/collections/(param1)/channels/(param2)
    ///
    /// Parameters:
    /// - `param1` (string)
    /// - `param2` (string)
    ///
    /// Request JSON Object:
    /// - `customer_id` (string)
    /// - `marketplace` (string)
    pub async fn post_library_collection_channels(
        &self,
        param1: &str,
        param2: &str,
        params: Option<Value>,
    ) -> Result<Value> {
        let url = format!(
            "{}/1.0/library/collections/{}/channels/{}",
            self.base_url, param1, param2
        );

        let mut req = self.client.post(url);
        if let Some(params) = params {
            req = req.json(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// POST /1.0/library/collections/(param1)/products/(param2)
    ///
    /// Parameters:
    /// - `param1` (string)
    /// - `param2` (string)
    ///
    /// Request JSON Object:
    /// - `channel_id` (string)
    pub async fn post_library_collection_products(
        &self,
        param1: &str,
        param2: &str,
        params: Option<Value>,
    ) -> Result<Value> {
        let url = format!(
            "{}/1.0/library/collections/{}/products/{}",
            self.base_url, param1, param2
        );

        let mut req = self.client.post(url);
        if let Some(params) = params {
            req = req.json(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/library/collections
    ///
    /// Query Parameters:
    /// - `customer_id` (string)
    /// - `marketplace` (string)
    pub async fn get_library_collections(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/library/collections", self.base_url);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.json(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// POST /1.0/library/collections
    ///
    /// Request JSON Object:
    /// - `collection_type` (string)
    pub async fn post_library_collections(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/library/collections", self.base_url);

        let mut req = self.client.post(url);
        if let Some(params) = params {
            req = req.json(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/library/collections/(param1)
    ///
    /// Parameters:
    /// - `param1` (string)
    ///
    /// Query Parameters:
    /// - `customer_id` (string)
    /// - `marketplace` (string)
    /// - `page_size` (integer)
    /// - `continuation_token` (string)
    pub async fn get_library_collections_with_param(
        &self,
        param1: &str,
        params: Option<Value>,
    ) -> Result<Value> {
        let url = format!("{}/1.0/library/collections/{}", self.base_url, param1);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/library/collections/(param1)/products
    ///
    /// Parameters:
    /// - `param1` (string)
    ///
    /// Query Parameters:
    /// - `customer_id` (string)
    /// - `marketplace` (string)
    /// - `page_size` (integer)
    /// - `continuation_token` (string)
    /// - `image_sizes` (string)
    pub async fn get_library_collections_with_param_products(
        &self,
        param1: &str,
        params: Option<Value>,
    ) -> Result<Value> {
        let url = format!(
            "{}/1.0/library/collections/{}/products",
            self.base_url, param1
        );

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::Auth;

    #[tokio::test]
    async fn test_get_library() {
        let auth = Auth::default("us").await.unwrap();
        let client = Client::new(auth).unwrap();
        client.get_library(None).await.unwrap();
    }
}
