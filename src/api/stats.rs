/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/stats/aggregates
    ///
    /// Query Parameters:
    /// - `daily_listening_interval_duration` (string) – ([012]?[0-9])|(30) (0 to 30, inclusive)
    /// - `daily_listening_interval_start_date` (string) – YYYY-MM-DD (e.g. 2019-06-16)
    /// - `locale` (string) – en_US
    /// - `monthly_listening_interval_duration` (string) – 0?[0-9]|1[012] (0 to 12, inclusive)
    /// - `monthly_listening_interval_start_date` (string) – YYYY-MM (e.g. 2019-02)
    /// - `response_groups` (string) – [total_listening_stats]
    /// - `store` (string) – [AudibleForInstitutions, Audible, AmazonEnglish, Rodizio]
    pub async fn get_stats_aggregates(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/stats/aggregates", self.base_url);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/stats/status/finished
    ///
    /// Query Parameters:
    /// - `asin` (string) – ASIN
    /// - `start_date` (string) – [RFC3339](https://tools.ietf.org/html/rfc3339) (e.g. 2000-01-01T00:00:00Z)
    pub async fn get_stats_status_finished(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/stats/status/finished", self.base_url);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// POST /1.0/stats/status/finished
    ///
    /// Request JSON Object:
    /// - `start_date` (string)
    /// - `status` (string)
    /// - `continuation_token` (string)
    pub async fn post_stats_status_finished(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/stats/status/finished", self.base_url);

        let mut req = self.client.post(url);
        if let Some(params) = params {
            req = req.json(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// PUT /1.0/stats/events
    ///
    /// Request JSON Object:
    /// - `stats` (object)
    ///
    /// Example:
    /// ```json
    /// {
    ///     "stats": [
    ///         {
    ///             "download_start": {
    ///                 "country_code": "de",
    ///                 "download_host": "xxxxx.cloudfront.net",
    ///                 "user_agent": "Audible, iPhone, 3.35.1 (644), iPhone XS (iPhone11,2), 238 GB, iOS, 14.1, Wifi",
    ///                 "request_id": "xxxxxxxxxxxx",
    ///                 "codec": "AAX_44_128",
    ///                 "source": "audible_iPhone"
    ///             },
    ///             "social_network_site": "Unknown",
    ///             "event_type": "DownloadStart",
    ///             "listening_mode": "Offline",
    ///             "local_timezone": "Europe/Berlin",
    ///             "asin_owned": false,
    ///             "playing_immersion_reading": false,
    ///             "audio_type": "FullTitle",
    ///             "event_timestamp": "2020-10-23T21:29:06.985Z",
    ///             "asin": "xxxxxxx",
    ///             "store": "Audible",
    ///             "delivery_type": "Download"
    ///         }
    ///     ]
    /// }
    /// ```
    pub async fn put_stats_events(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/stats/events", self.base_url);

        let mut req = self.client.put(url);
        if let Some(params) = params {
            req = req.json(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }
}
