
/*
https://archive-api.open-meteo.com/v1/archive?
    latitude=52.52
    &longitude=13.41
    &start_date=2025-03-17
    &end_date=2025-03-31
    &daily=weather_code,apparent_temperature_mean,wind_direction_10m_dominant,dew_point_2m_mean
    &hourly=temperature_2m,weather_code,wind_speed_10m,soil_temperature_0_to_7cm,boundary_layer_height,global_tilted_irradiance,dew_point_2m_spread
    &models=best_match
    &wind_speed_unit=mph
    &temperature_unit=fahrenheit
    &precipitation_unit=inch
    &timeformat=unixtime
    &temporal_resolution=hourly_3
    &cell_selection=sea
    &tilt=20
    &azimuth=2
*/
/// https://open-meteo.com/en/docs/historical-weather-api
pub struct HistoricalWeatherRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub start_date: Option<String>,         // E.g: "2025-03-26"
    pub end_date: Option<String>,           // E.g: "2025-04-09"
    pub daily: Option<Vec<String>>,         // E.g: ["weather_code,rain_sum"]
    pub hourly: Option<Vec<String>>,        // E.g: ["temperature_2m", "precipitation"]
    pub models: Option<String>,             // E.g: "best_match"
    pub timeformat: Option<String>,         // E.g: "unixtime" | "iso8601"
    pub wind_speed_unit: Option<String>,    // E.g: "kmh" | "mph" | "ms"
    pub temperature_unit: Option<String>,   // E.g: "celsius" | "fahrenheit"
    pub precipitation_unit: Option<String>, // E.g: "mm" | "inch"
    pub temporal_resolution: Option<String>,// E.g: "hourly_3"
    pub cell_selection: Option<String>,     // E.g: "sea" | "land"
    pub tilt: Option<u32>,                  // E.g: 20
    pub azimuth: Option<u32>,               // E.g: 2
}

impl HistoricalWeatherRequest {
    pub fn new(
        latitude: f64,
        longitude: f64,
    ) -> Self {
        HistoricalWeatherRequest {
            latitude,
            longitude,
            daily: None,
            hourly: None,
            models: None,
            timeformat: None,
            wind_speed_unit: None,
            temperature_unit: None,
            precipitation_unit: None,
            start_date: None,
            end_date: None,
            temporal_resolution: None,
            cell_selection: None,
            tilt: None,
            azimuth: None,
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
    pub fn temporal_resolution(mut self, resolution: &str) -> Self {
        self.temporal_resolution = Some(resolution.to_string());
        self
    }
    pub fn cell_selection(mut self, selection: &str) -> Self {
        self.cell_selection = Some(selection.to_string());
        self
    }
    pub fn tilt(mut self, tilt: u32) -> Self {
        self.tilt = Some(tilt);
        self
    }
    pub fn azimuth(mut self, azimuth: u32) -> Self {
        self.azimuth = Some(azimuth);
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
}

