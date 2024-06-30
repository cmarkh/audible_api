use serde_json::json;

use super::Client;
use crate::Result;

impl Client {
    pub async fn get_library(&self) -> Result<()> {
        let path = "1.0/library";
        let url = format!("{}/{}", self.base_url, path);
        let query = json! {{
                "num_results": 1000,
                "response_groups": "product_desc,product_attrs",
                "sort_by": "-PurchaseDate"
        }};
        let req = self.client.get(url).query(&query).build()?;

        let res = self.send_request(req).await?;
        dbg!(res.text().await?);

        todo!()
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
        client.get_library().await.unwrap();
    }
}
