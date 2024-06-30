/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/content/(string:asin)/metadata
    ///
    /// Parameters:
    /// - `asin` (string) – The ASIN of the book
    ///
    /// Query Parameters:
    /// - `response_groups` (string) – [chapter_info, always-returned, content_reference, content_url]
    /// - `acr` (string)
    /// - `quality` (string) – [High, Normal]
    /// - `chapter_titles_type` (string) – [Tree, Flat]
    /// - `drm_type` (string) – [Mpeg, PlayReady, Hls, Dash, FairPlay, Widevine, HlsCmaf, Adrm]
    pub async fn get_content_metadata(&self, asin: &str, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/content/{}/metadata", self.base_url, asin);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// POST /1.0/content/(string:asin)/drmlicense
    ///
    /// Parameters:
    /// - `asin` (string) – The ASIN of the book
    ///
    /// Request JSON Object:
    /// - `licenseChallenge` (string) – The license challenge
    /// - `asin` (string) – The ASIN of the book
    /// - `consumption_type` (string) – "Download"
    /// - `drm_type` (string) – "FairPlay"
    ///
    /// Response JSON Object:
    /// - `license` (string) – The encrypted license
    pub async fn post_drm_license(&self, asin: &str, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/content/{}/drmlicense", self.base_url, asin);

        let mut req = self.client.post(url);
        if let Some(params) = params {
            req = req.json(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET 1.0/content/FairPlay/certificate
    ///
    /// Response JSON Object:
    /// - `certificate` (string) – The base64 encoded FairPlay certificate
    pub async fn get_fairplay_certificate(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/content/FairPlay/certificate", self.base_url);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// POST /1.0/content/(string:asin)/licenserequest
    ///
    /// Parameters:
    /// - `asin` (string) – The ASIN of the book
    ///
    /// Request JSON Object:
    /// - `use_adaptive_bit_rate` (boolean) – [true, false]
    /// - `quality` (string) – [High, Normal]
    /// - `chapter_titles_type` (string) – [Tree, Flat]
    /// - `response_groups` (string) – [chapter_info, content_reference, last_position_heard, pdf_url, ad_insertion, certificate]
    /// - `consumption_type` (string) – [Streaming, Offline, Download]
    /// - `spatial` (boolean) – [true, false]
    /// - `supported_media_features` (dict) – [codecs, drm_types]
    ///   - `codecs` (list) – [mp4a.40.2, mp4a.40.42, ec+3, ac-4]
    ///   - `drm_types` (list) – [Mpeg, PlayReady, Hls, Dash, Adrm, FairPlay, Widevine, HlsCmaf]
    /// - `num_active_offline_licenses` (integer) – (max: 10)
    ///
    /// Example:
    /// ```json
    /// {
    ///     "quality": "High",
    ///     "response_groups": "chapter_info,content_reference,last_position_heard,pdf_url,ad_insertion,certificate",
    ///     "consumption_type": "Download",
    ///     "supported_media_features": {
    ///         "codecs": [
    ///             "mp4a.40.2",
    ///             "mp4a.40.42",
    ///             "ec+3",
    ///             "ac-4"
    ///         ],
    ///         "drm_types": [
    ///             "Mpeg",
    ///             "PlayReady",
    ///             "Hls",
    ///             "Dash",
    ///             "Adrm",
    ///             "FairPlay",
    ///             "Widevine",
    ///             "HlsCmaf"
    ///         ]
    ///     },
    ///     "spatial": false
    /// }
    /// ```
    ///
    /// For a succesful request, returns JSON body with content_url.
    pub async fn post_license_request(&self, asin: &str, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/content/{}/licenserequest", self.base_url, asin);

        let mut req = self.client.post(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }
}
