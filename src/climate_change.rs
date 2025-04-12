/*
https://climate-api.open-meteo.com/v1/climate?
    latitude=52.52
    &longitude=13.41
    &start_date=1950-01-01
    &end_date=2050-12-31
    &daily=temperature_2m_max,relative_humidity_2m_max
    &models=CMCC_CM2_VHR4,FGOALS_f3_H,HiRAM_SIT_HR,MRI_AGCM3_2_S,EC_Earth3P_HR,MPI_ESM1_2_XR,NICAM16_8S
    &timeformat=unixtime
    &wind_speed_unit=mph
    &temperature_unit=fahrenheit
    &precipitation_unit=inch
*/
/// https://open-meteo.com/en/docs/climate-api
#[derive(Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ClimateChangeRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub start_date: Option<String>,         // E.g: "2025-03-26"
    pub end_date: Option<String>,           // E.g: "2025-04-09"
    pub daily: Option<Vec<String>>,         // E.g: ["weather_code,rain_sum"]
    pub models: Option<String>,             // E.g: "CMCC_CM2_VHR4,FGOALS_f3_H,HiRAM_SIT_HR,MRI_AGCM3_2_S,EC_Earth3P_HR,MPI_ESM1_2_XR,NICAM16_8S"
    pub timeformat: Option<String>,         // E.g: "unixtime" | "iso8601"
    pub wind_speed_unit: Option<String>,    // E.g: "kmh" | "mph" | "ms"
    pub temperature_unit: Option<String>,   // E.g: "celsius" | "fahrenheit"
    pub precipitation_unit: Option<String>, // E.g: "mm" | "inch"
}

impl ClimateChangeRequest {
pub fn new(
    latitude: f64,
    longitude: f64,
) -> Self {
    ClimateChangeRequest {
        latitude,
        longitude,
        daily: None,
        models: None,
        timeformat: None,
        wind_speed_unit: None,
        temperature_unit: None,
        precipitation_unit: None,
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
    pub fn add_daily(mut self, param: &str) -> Self {
        self.daily.get_or_insert_with(Vec::new).push(param.to_string());
        self
    }
}