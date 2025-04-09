/*
https://geocoding-api.open-meteo.com/v1/search?
    name=Berlin
    &count=1
    &language=en
    &format=json
    &countryCode=DE
*/
/// https://open-meteo.com/en/docs/geocoding-api
pub struct GeocodingRequest {
    pub name: String,                 // E.g: "Berlin"
    pub count: Option<u32>,           // E.g: 1
    pub language: Option<String>,     // E.g: "en"
    pub format: Option<String>,       // E.g: "json"
    pub country_code: Option<String>, // E.g: "DE"
}

impl GeocodingRequest {
    pub fn new(name: &str) -> Self {
        GeocodingRequest {
            name: name.to_string(),
            count: None,
            language: None,
            format: None,
            country_code: None,
        }
    }

    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    pub fn language(mut self, language: &str) -> Self {
        self.language = Some(language.to_string());
        self
    }

    pub fn format(mut self, format: &str) -> Self {
        self.format = Some(format.to_string());
        self
    }

    pub fn country_code(mut self, code: &str) -> Self {
        self.country_code = Some(code.to_string());
        self
    }
}

// TODO: FINISH ALL OF THE
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeocodingResponse {
    pub results: Vec<Rslt>,
    #[serde(rename = "generationtime_ms")]
    pub generationtime_ms: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rslt {
    pub id: i64,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    #[serde(rename = "feature_code")]
    pub feature_code: String,
    #[serde(rename = "country_code")]
    pub country_code: String,
    #[serde(rename = "admin1_id")]
    pub admin1_id: Option<i64>,
    #[serde(rename = "admin3_id")]
    pub admin3_id: Option<i64>,
    #[serde(rename = "admin4_id")]
    pub admin4_id: Option<i64>,
    pub timezone: String,
    pub population: i64,
    pub postcodes: Vec<String>,
    #[serde(rename = "country_id")]
    pub country_id: i64,
    pub country: String,
    pub admin1: Option<String>,
    pub admin3: Option<String>,
    pub admin4: Option<String>,
}
