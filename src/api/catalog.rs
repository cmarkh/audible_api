/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/collections
    ///
    /// Query Parameters:
    /// - `state_token` (string) – [ey…]
    /// - `visibility_types` (string) – [Private, Discoverable]
    pub async fn get_collections(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/collections", self.base_url);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// POST /1.0/collections
    /// Create a new collection
    ///
    /// Request JSON Object:
    /// - `name` (string)
    /// - `asins` (array of strings) – []
    /// - `description` (string)
    ///
    /// Response JSON Object:
    /// - `collection_id` (string)
    /// - `creation_date` (string)
    /// - `customer_id` (string)
    /// - `marketplace` (string)
    pub async fn post_collections(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/collections", self.base_url);

        let mut req = self.client.post(url);
        if let Some(params) = params {
            req = req.json(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/collections/(collection_id)
    ///
    /// Parameters:
    /// - `collection_id` (string)
    pub async fn get_collection_by_id(
        &self,
        collection_id: &str,
        params: Option<Value>,
    ) -> Result<Value> {
        let url = format!("{}/1.0/collections/{}", self.base_url, collection_id);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// PUT /1.0/collections/(collection_id)
    /// Modify a collection
    ///
    /// Parameters:
    /// - `collection_id` (string)
    ///
    /// Request JSON Object:
    /// - `state_token` (string)
    /// - `collection_id` (string)
    /// - `name` (string)
    /// - `description` (string)
    ///
    /// Response JSON Object:
    /// - `state_token` (string)
    /// - `collection_id` (string)
    /// - `name` (string)
    /// - `description` (string)
    pub async fn put_collection_by_id(
        &self,
        collection_id: &str,
        params: Option<Value>,
    ) -> Result<Value> {
        let url = format!("{}/1.0/collections/{}", self.base_url, collection_id);

        let mut req = self.client.put(url);
        if let Some(params) = params {
            req = req.json(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/collections/(collection_id)/items
    ///
    /// Parameters:
    /// - `collection_id` (string) – e.g. __FAVORITES
    ///
    /// Query Parameters:
    /// - `response_groups` (string) – [always-returned]
    pub async fn get_items_by_collection_id(
        &self,
        collection_id: &str,
        params: Option<Value>,
    ) -> Result<Value> {
        let url = format!("{}/1.0/collections/{}/items", self.base_url, collection_id);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// POST /1.0/collections/(collection_id)/items
    /// Add item(s) to a collection
    ///
    /// Parameters:
    /// - `collection_id` (string)
    ///
    /// Request JSON Object:
    /// - `collection_id` (string)
    /// - `asins` (array of strings) – []
    ///
    /// Response JSON Object:
    /// - `description` (string)
    /// - `name` (string)
    /// - `num_items_added` (integer)
    /// - `state_token` (string)
    pub async fn post_items_by_collection_id(
        &self,
        collection_id: &str,
        params: Option<Value>,
    ) -> Result<Value> {
        let url = format!("{}/1.0/collections/{}/items", self.base_url, collection_id);

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
