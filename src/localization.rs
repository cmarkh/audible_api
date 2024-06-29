use std::collections::HashMap;

use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    static ref LOCALES: HashMap<String, Locale> = {
        let locales = include_str!("locales.json");
        serde_json::from_str(locales).unwrap()
    };
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Locale {
    pub country_code: String,
    pub domain: String,
    pub market_place_id: String,
}

pub fn find_by_country_code(country_code: &str) -> Option<Locale> {
    LOCALES
        .iter()
        .find(|(_, locale)| locale.country_code == country_code)
        .map(|(_, locale)| locale)
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locales() {
        dbg!(LOCALES.get("united_states"));
        assert!(LOCALES.contains_key("united_states"));
    }

    #[test]
    fn test_find_by_country_code() {
        let locale = find_by_country_code("us").unwrap();
        assert_eq!(locale.country_code, "us");
    }
}
