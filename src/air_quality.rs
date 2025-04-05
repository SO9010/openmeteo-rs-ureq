
/*
https://air-quality-api.open-meteo.com/v1/air-quality?latitude=52.52&longitude=13.41&hourly=pm10,pm2_5,grass_pollen,european_aqi,us_aqi,non_methane_volatile_organic_compounds&current=us_aqi,birch_pollen,ammonia&past_days=1&forecast_days=1&domains=cams_global&timeformat=unixtime&forecast_hours=1&past_hours=1&temporal_resolution=hourly_3&cell_selection=sea
*/
/// https://open-meteo.com/en/docs/air-quality-api
pub struct AirQualityRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub hourly: Option<Vec<String>>,        // E.g: ["pm10", "pm2_5"]
    pub current: Option<Vec<String>>,       // E.g: ["us_aqi", "birch_pollen"]
    pub past_days: Option<u32>,             // E.g: 1
    pub forecast_days: Option<u32>,         // E.g: 1
    pub domains: Option<String>,            // E.g: "cams_global"
    pub timeformat: Option<String>,         // E.g: "unixtime" | "iso8601"
    pub forecast_hours: Option<u32>,        // E.g: 1
    pub past_hours: Option<u32>,            // E.g: 1
    pub temporal_resolution: Option<String>,// E.g: "hourly_3"
    pub cell_selection: Option<String>,     // E.g: "sea"
}

impl AirQualityRequest {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        AirQualityRequest {
            latitude,
            longitude,
            hourly: None,
            current: None,
            past_days: None,
            forecast_days: None,
            domains: None,
            timeformat: None,
            forecast_hours: None,
            past_hours: None,
            temporal_resolution: None,
            cell_selection: None,
        }
    }

    pub fn add_hourly(mut self, param: &str) -> Self {
        self.hourly.get_or_insert_with(Vec::new).push(param.to_string());
        self
    }

    pub fn add_current(mut self, param: &str) -> Self {
        self.current.get_or_insert_with(Vec::new).push(param.to_string());
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

    pub fn domains(mut self, domain: &str) -> Self {
        self.domains = Some(domain.to_string());
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
}
