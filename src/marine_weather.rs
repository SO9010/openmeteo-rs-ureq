

/*
https://marine-api.open-meteo.com/v1/marine?
    latitude=54.544587
    &longitude=10.227487
    &daily=wave_height_max,swell_wave_peak_period_max
    &hourly=wave_height,wind_wave_height,wave_peak_period
    &models=best_match,meteofrance_currents
    &current=wave_height,wind_wave_peak_period
    &past_days=1
    &forecast_days=1
    &timeformat=unixtime
    &length_unit=imperial
    &wind_speed_unit=ms
    &minutely_15=ocean_current_velocity
    &past_minutely_15=4
    &forecast_minutely_15=4
    &forecast_hours=1
    &past_hours=1
    &temporal_resolution=hourly_3
    &cell_selection=sea
*/
/// https://open-meteo.com/en/docs/marine-weather-api
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MarineWeatherRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub daily: Option<Vec<String>>,         // E.g: ["wave_height_max,swell_wave_peak_period_max"]
    pub hourly: Option<Vec<String>>,        // E.g: ["wave_height", "wind_wave_height", "wave_peak_period"]
    pub models: Option<String>,             // E.g: "best_match,meteofrance_currents"
    pub current: Option<Vec<String>>,       // E.g: ["wave_height", "wind_wave_peak_period"]
    pub past_days: Option<u32>,             // E.g: 1
    pub forecast_days: Option<u32>,         // E.g: 1
    pub timeformat: Option<String>,         // E.g: "unixtime" | "iso8601"
    pub length_unit: Option<String>,        // E.g: "imperial" | "metric"
    pub wind_speed_unit: Option<String>,    // E.g: "kmh" | "mph" | "ms"
    pub minutely: Option<Vec<String>>,     // E.g: ["ocean_current_velocity"]
    pub past_minutely: Option<u32>,         // E.g: 4
    pub forecast_minutely: Option<u32>,     // E.g: 4
    pub forecast_hours: Option<u32>,        // E.g: 1
    pub past_hours: Option<u32>,            // E.g: 1
    pub temporal_resolution: Option<String>,// E.g: "hourly_3"
    pub cell_selection: Option<String>,     // E.g: "sea"
    pub start_date: Option<String>,         // E.g: "2025-03-26"
    pub end_date: Option<String>,           // E.g: "2025-04-09"
}

impl MarineWeatherRequest {
pub fn new(
    latitude: f64,
    longitude: f64,
) -> Self {
    MarineWeatherRequest {
        latitude,
        longitude,
        daily: None,
        hourly: None,
        models: None,
        current: None,
        past_days: None,
        forecast_days: None,
        timeformat: None,
        length_unit: None,
        wind_speed_unit: None,
        minutely: None,
        past_minutely: None,
        forecast_minutely: None,
        forecast_hours: None,
        past_hours: None,
        temporal_resolution: None,
        cell_selection: None,
        start_date: None,
        end_date: None,
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
    pub fn wind_speed_unit(mut self, unit: &str) -> Self {
        self.wind_speed_unit = Some(unit.to_string());
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
    pub fn current(mut self, params: &[&str]) -> Self {
        self.current = Some(params.iter().map(|s| s.to_string()).collect());
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
    pub fn forecast_hours(mut self, hours: u32) -> Self {
        self.forecast_hours = Some(hours);
        self
    }
    pub fn past_hours(mut self, hours: u32) -> Self {
        self.past_hours = Some(hours);
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
    pub fn temporal_resolution(mut self, resolution: &str) -> Self {
        self.temporal_resolution = Some(resolution.to_string());
        self
    }
    pub fn cell_selection(mut self, selection: &str) -> Self {
        self.cell_selection = Some(selection.to_string());
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

    pub fn add_current(mut self, param: &str) -> Self {
        self.current.get_or_insert_with(Vec::new).push(param.to_string());
        self
    }

    pub fn add_minutely(mut self, param: &str) -> Self {
        self.minutely.get_or_insert_with(Vec::new).push(param.to_string());
        self
    }
}
