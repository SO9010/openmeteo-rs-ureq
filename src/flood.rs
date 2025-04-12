/*
https://flood-api.open-meteo.com/v1/flood?
    latitude=59.91
    &longitude=10.75
    &daily=river_discharge&models=forecast_v4
    &timeformat=unixtime
    &past_days=2
    &forecast_days=31
*/
/// https://open-meteo.com/en/docs/flood-api
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FloodRequest {
    pub latitude: f64,                     // E.g: 59.91
    pub longitude: f64,                    // E.g: 10.75
    pub daily: Option<Vec<String>>,         // E.g: ["river_discharge"]
    pub models: Option<String>,             // E.g: "forecast_v4"
    pub timeformat: Option<String>,         // E.g: "unixtime" | "iso8601"
    pub past_days: Option<u32>,             // E.g: 2
    pub forecast_days: Option<u32>,         // E.g: 31
}
impl FloodRequest {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        FloodRequest {
            latitude,
            longitude,
            daily: None,
            models: None,
            timeformat: None,
            past_days: None,
            forecast_days: None,
        }
    }
    pub fn daily(mut self, params: &[&str]) -> Self {
        self.daily = Some(params.iter().map(|s| s.to_string()).collect());
        self
    }
    pub fn models(mut self, model: &str) -> Self {
        self.models = Some(model.to_string());
        self
    }

    pub fn add_daily(mut self, param: &str) -> Self {
        self.daily.get_or_insert_with(Vec::new).push(param.to_string());
        self
    }

    pub fn timeformat(mut self, format: &str) -> Self {
        self.timeformat = Some(format.to_string());
        self
    }

    pub fn past_days(mut self, days: u32) -> Self {
        self.past_days = Some(days);
        self
    }

    pub fn forecast_days(mut self, days: u32) -> Self {
        self.forecast_days = Some(days);
        self
    }
}