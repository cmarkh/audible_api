/// see API docs at https://audible.readthedocs.io/en/latest/misc/external_api.html
use serde_json::Value;

use super::Client;
use crate::Result;

impl Client {
    /// GET /1.0/catalog/categories
    ///
    /// Query Parameters:
    /// - `response_groups` (string) – [category_metadata, products]
    /// - `products_plan` (string) – [Enterprise, RodizioFreeBasic, AyceRomance, AllYouCanEat, US Minerva, Universal, AmazonEnglish,
    ///   ComplimentaryOriginalMemberBenefit, Radio, SpecialBenefit, Rodizio]
    /// - `products_in_plan_timestamp` (string)
    /// - `products_num_results` (integer)
    /// - `runtime_length_min` (integer)
    /// - `content_level` (string)
    /// - `content_type` (string)
    /// - `categories_num_levels` (integer) – (greater than or equal to 1)
    /// - `ids` (string) – \d+(,\d+)*
    /// - `root` (string) – [InstitutionsHpMarketing, ChannelsConfigurator, AEReadster, ShortsPrime, ExploreBy, RodizioBuckets, EditorsPicks,
    ///   ClientContent, RodizioGenres, AmazonEnglishProducts, ShortsSandbox, Genres, Curated, ShortsIntroOutroRemoval, Shorts,
    ///   RodizioEpisodesAndSeries, ShortsCurated]
    pub async fn get_catalog_categories(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/catalog/categories", self.base_url);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/catalog/categories/(category_id)
    ///
    /// Parameters:
    /// - `category_id` (string)
    ///
    /// Query Parameters:
    /// - `image_dpi` (integer)
    /// - `image_sizes` (string)
    /// - `image_variants` (string)
    /// - `products_in_plan_timestamp` (string)
    /// - `products_num_results` (integer)
    /// - `products_plan` (string) – [Enterprise, RodizioFreeBasic, AyceRomance, AllYouCanEat, AmazonEnglish, ComplimentaryOriginalMemberBenefit, Radio, SpecialBenefit, Rodizio]
    /// - `products_sort_by` (string) – [-ReleaseDate, ContentLevel, -Title, AmazonEnglish, AvgRating, BestSellers, -RuntimeLength, ReleaseDate, ProductSiteLaunchDate, -ContentLevel, Title, Relevance, RuntimeLength]
    /// - `reviews_num_results` (integer)
    /// - `reviews_sort_by` (string) – [MostHelpful, MostRecent]
    pub async fn get_catalog_category_by_id(
        &self,
        category_id: &str,
        params: Option<Value>,
    ) -> Result<Value> {
        let url = format!("{}/1.0/catalog/categories/{}", self.base_url, category_id);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/catalog/products
    ///
    /// Query Parameters:
    /// - `author` (string)
    /// - `browse_type` (string)
    /// - `category_id` (integer) – \d+(,\d+)*
    /// - `disjunctive_category_ids` (string)
    /// - `image_dpi` (integer)
    /// - `image_sizes` (string)
    /// - `in_plan_timestamp` (string)
    /// - `keywords` (string)
    /// - `narrator` (string)
    /// - `not_in_plan_timestamp` (string)
    /// - `num_most_recent` (integer)
    /// - `num_results` (integer) – (max: 50)
    /// - `page` (integer)
    /// - `plan` (string) – [Enterprise, RodizioFreeBasic, AyceRomance, AllYouCanEat, AmazonEnglish, ComplimentaryOriginalMemberBenefit, Radio, SpecialBenefit, Rodizio]
    /// - `products_since_timestamp` (string)
    /// - `products_sort_by` (string) – [-ReleaseDate, ContentLevel, -Title, AmazonEnglish, AvgRating, BestSellers, -RuntimeLength, ReleaseDate, ProductSiteLaunchDate, -ContentLevel, Title, Relevance, RuntimeLength]
    /// - `publisher` (string)
    /// - `response_groups` (string) – [contributors, media, price, product_attrs, product_desc, product_extended_attrs, product_plan_details, product_plans, rating, review_attrs, reviews, sample, series, sku]
    /// - `reviews_num_results` (integer) – (max: 10)
    /// - `reviews_sort_by` (string) – [MostHelpful, MostRecent]
    /// - `title` (string)
    pub async fn get_products(&self, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/catalog/products", self.base_url);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/catalog/products/(string:asin)
    ///
    /// Parameters:
    /// - `asin` (string) – The ASIN of the book
    ///
    /// Query Parameters:
    /// - `image_dpi` (integer)
    /// - `image_sizes` (string)
    /// - `response_groups` (string) – [contributors, media, price, product_attrs, product_desc, product_details, product_extended_attrs,
    ///   product_plan_details, product_plans, rating, sample, sku, series, reviews, relationships, review_attrs, category_ladders,
    ///   claim_code_url, provided_review, rights, customer_rights]
    /// - `reviews_num_results` (integer) – \d+ (max: 10)
    /// - `reviews_sort_by` (string) – [MostHelpful, MostRecent]
    /// - `asins` (string)
    pub async fn get_products_by_asin(&self, asin: &str, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/catalog/products/{}", self.base_url, asin);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/catalog/products/(string:asin)/reviews
    ///
    /// Parameters:
    /// - `asin` (string) – The ASIN of the book
    ///
    /// Query Parameters:
    /// - `sort_by` (string) – [MostHelpful, MostRecent]
    /// - `num_results` (integer) – (max: 50)
    /// - `page` (integer)
    pub async fn get_product_reviews(&self, asin: &str, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/catalog/products/{}/reviews", self.base_url, asin);

        let mut req = self.client.get(url);
        if let Some(params) = params {
            req = req.query(&params);
        }
        let req = req.build()?;

        let res = self.send_request(req).await?;
        let json: Value = res.json().await?;
        Ok(json)
    }

    /// GET /1.0/catalog/products/(string:asin)/sims
    ///
    /// Parameters:
    /// - `asin` (string) – The ASIN of the book
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
    /// - `response_groups` (string) – [contributors, media, price, product_attrs, product_desc, product_extended_attrs, product_plans, rating, review_attrs, reviews, sample, sku]
    /// - `reviews_num_results` (integer) – (max: 10)
    /// - `reviews_sort_by` (string) – [MostHelpful, MostRecent]
    /// - `similarity_type` (string) – [InTheSameSeries, ByTheSameNarrator, RawSimilarities, ByTheSameAuthor, NextInSameSeries]
    pub async fn get_similar_products(&self, asin: &str, params: Option<Value>) -> Result<Value> {
        let url = format!("{}/1.0/catalog/products/{}/sims", self.base_url, asin);

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
