

/*
https://api.open-meteo.com/v1/forecast?
    latitude=52.52
    &longitude=13.41
    &daily=weather_code,rain_sum
    &hourly=temperature_2m,precipitation,uv_index,shortwave_radiation,temperature_1000hPa
    &minutely_15=temperature_2m
    &current=temperature_2m,relative_humidity_2m,precipitation
    &models=best_match
    &timezone=auto
    &timeformat=unixtime
    &wind_speed_unit=ms
    &temperature_unit=fahrenheit
    &precipitation_unit=inch
    &start_date=2025-03-26
    &end_date=2025-04-09
*/
/// https://open-meteo.com/en/docs
#[derive(Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct WeatherRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub daily: Option<Vec<String>>,         // E.g: ["weather_code,rain_sum"]
    pub hourly: Option<Vec<String>>,        // E.g: ["temperature_2m", "precipitation"]
    pub minutely: Option<Vec<String>>,      // E.g: ["temperature_2m"]
    pub current: Option<Vec<String>>,       // E.g: ["temperature_2m", "relative_humidity_2m", "precipitation"]
    pub models: Option<String>,             // E.g: "best_match"
    pub timezone: Option<String>,           // E.g: "auto" | "GMT" | "UTC"
    pub timeformat: Option<String>,         // E.g: "unixtime" | "iso8601"
    pub wind_speed_unit: Option<String>,    // E.g: "kmh" | "mph" | "ms"
    pub temperature_unit: Option<String>,   // E.g: "celsius" | "fahrenheit"
    pub precipitation_unit: Option<String>, // E.g: "mm" | "inch"
    pub start_date: Option<String>,         // E.g: "2025-03-26"
    pub end_date: Option<String>,           // E.g: "2025-04-09"
    pub past_days: Option<u32>,             // E.g: 7
    pub forecast_days: Option<u32>,         // E.g: 7
}

impl WeatherRequest {
    pub fn new(
        latitude: f64,
        longitude: f64,
    ) -> Self {
        WeatherRequest {
            latitude,
            longitude,
            daily: None,
            hourly: None,
            minutely: None,
            current: None,
            models: None,
            timezone: None,
            timeformat: None,
            wind_speed_unit: None,
            temperature_unit: None,
            precipitation_unit: None,
            start_date: None,
            end_date: None,
            past_days: None,
            forecast_days: None
        }
    }
    pub fn start_date(mut self, date: &str) -> Self {
        self.start_date = Some(date.to_string());
        self
    }

    pub fn end_date(mut self, date: &str) -> Self {
        self.end_date = Some(date.to_string());
        self
    }

    pub fn temperature_unit(mut self, unit: &str) -> Self {
        self.temperature_unit = Some(unit.to_string());
        self
    }

    pub fn wind_speed_unit(mut self, unit: &str) -> Self {
        self.wind_speed_unit = Some(unit.to_string());
        self
    }

    pub fn precipitation_unit(mut self, unit: &str) -> Self {
        self.precipitation_unit = Some(unit.to_string());
        self
    }

    pub fn timeformat(mut self, format: &str) -> Self {
        self.timeformat = Some(format.to_string());
        self
    }

    pub fn models(mut self, model: &str) -> Self {
        self.models = Some(model.to_string());
        self
    }

    pub fn hourly(mut self, params: &[&str]) -> Self {
        self.hourly = Some(params.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn daily(mut self, params: &[&str]) -> Self {
        self.daily = Some(params.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn add_daily(mut self, param: &str) -> Self {
        self.daily.get_or_insert_with(Vec::new).push(param.to_string());
        self
    }

    pub fn add_hourly(mut self, param: &str) -> Self {
        self.hourly.get_or_insert_with(Vec::new).push(param.to_string());
        self
    }

    pub fn add_minutely(mut self, param: &str) -> Self {
        self.minutely.get_or_insert_with(Vec::new).push(param.to_string());
        self
    }

    pub fn add_current(mut self, param: &str) -> Self {
        self.current.get_or_insert_with(Vec::new).push(param.to_string());
        self
    }
}