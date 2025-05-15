/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/wishlist
    ///
    /// Query Parameters:
    /// - `num_results` (integer) – (max: 50)
    /// - `page` (integer) – (wishlist starts at page 0)
    /// - `locale` (string) – e.g. de-DE
    /// - `response_groups` (string) – [contributors, media, price, product_attrs, product_desc, product_extended_attrs,
    ///   product_plan_details, product_plans, rating, sample, sku, customer_rights, relationships]
    /// - `sort_by` (string) – [-Author, -DateAdded, -Price, -Rating, -Title, Author, DateAdded, Price, Rating, Title]
    pub async fn get_wishlist(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/wishlist", self.base_url);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// POST /1.0/wishlist
    ///
    /// Request JSON Object:
    /// - `asin` (string) – The ASIN of the book to add
    ///
    /// Status Codes:
    /// - `201 Created` – Returns the Location to the resource.
    pub async fn post_wishlist(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/wishlist", self.base_url);

        let mut req = self.client.post(url);
        if let Some(params) = params {
            req = req.json(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// DELETE /1.0/wishlist/(string:asin)
    ///
    /// Parameters:
    /// - `asin` (string) – The ASIN of the book
    ///
    /// Status Codes:
    /// - `204 No Content` – Removes the item from the wishlist using the given ASIN.
    pub async fn delete_from_wishlist(&self, asin: &str, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/wishlist/{}", self.base_url, asin);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::Auth;

    #[tokio::test]
    async fn test_get_wishlist() {
        let auth = Auth::default("us").await.unwrap();
        let client = Client::new(auth).unwrap();
        let res = client.get_wishlist(None).await.unwrap();
        dbg!(res);
        // let json = serde_json::to_string_pretty(&res).unwrap();
        // std::fs::write("wishlist.json", json).unwrap();
    }
}
