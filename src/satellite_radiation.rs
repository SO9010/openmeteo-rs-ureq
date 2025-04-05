/*
https://satellite-api.open-meteo.com/v1/archive?latitude=52.52&longitude=13.41&daily=sunrise&hourly=shortwave_radiation,diffuse_radiation_instant,is_day,sunshine_duration&models=eumetsat_lsa_saf_iodc,ukmo_global_deterministic_10km&tilt=1&azimuth=-1&timeformat=unixtime&forecast_hours=12&past_hours=12&temporal_resolution=hourly_3&cell_selection=sea
*/
/// https://open-meteo.com/en/docs/satellite-radiation-api
pub struct SatelliteRadiationRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub daily: Option<Vec<String>>,         // E.g: ["sunrise"]
    pub hourly: Option<Vec<String>>,        // E.g: ["shortwave_radiation", "is_day"]
    pub models: Option<String>,             // E.g: "eumetsat_lsa_saf_iodc,ukmo_global_deterministic_10km"
    pub tilt: Option<u32>,                  // E.g: 1
    pub azimuth: Option<i32>,               // E.g: -1
    pub timeformat: Option<String>,         // E.g: "unixtime" | "iso8601"
    pub forecast_hours: Option<u32>,        // E.g: 12
    pub past_hours: Option<u32>,            // E.g: 12
    pub temporal_resolution: Option<String>,// E.g: "hourly_3"
    pub cell_selection: Option<String>,     // E.g: "sea"
    pub start_date: Option<String>,         // Added missing field
    pub end_date: Option<String>,           // Added missing field
    pub past_minutely: Option<u32>,         // Added missing field
    pub forecast_minutely: Option<u32>,     // Added missing field
}

impl SatelliteRadiationRequest {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        SatelliteRadiationRequest {
            latitude,
            longitude,
            daily: None,
            hourly: None,
            models: None,
            tilt: None,
            azimuth: None,
            timeformat: None,
            forecast_hours: None,
            past_hours: None,
            temporal_resolution: None,
            cell_selection: None,
            start_date: None,
            end_date: None,
            past_minutely: None,
            forecast_minutely: None,
        }
    }

    pub fn add_daily(mut self, param: &str) -> Self {
        self.daily.get_or_insert_with(Vec::new).push(param.to_string());
        self
    }

    pub fn add_hourly(mut self, param: &str) -> Self {
        self.hourly.get_or_insert_with(Vec::new).push(param.to_string());
        self
    }

    pub fn models(mut self, model: &str) -> Self {
        self.models = Some(model.to_string());
        self
    }

    pub fn tilt(mut self, tilt: u32) -> Self {
        self.tilt = Some(tilt);
        self
    }

    pub fn azimuth(mut self, azimuth: i32) -> Self {
        self.azimuth = Some(azimuth);
        self
    }

    pub fn timeformat(mut self, format: &str) -> Self {
        self.timeformat = Some(format.to_string());
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

    pub fn temporal_resolution(mut self, resolution: &str) -> Self {
        self.temporal_resolution = Some(resolution.to_string());
        self
    }

    pub fn cell_selection(mut self, selection: &str) -> Self {
        self.cell_selection = Some(selection.to_string());
        self
    }

    pub fn start_date(mut self, date: &str) -> Self {
        self.start_date = Some(date.to_string());
        self
    }

    pub fn end_date(mut self, date: &str) -> Self {
        self.end_date = Some(date.to_string());
        self
    }

    pub fn past_minutely(mut self, minutes: u32) -> Self {
        self.past_minutely = Some(minutes);
        self
    }

    pub fn forecast_minutely(mut self, minutes: u32) -> Self {
        self.forecast_minutely = Some(minutes);
        self
    }
}
