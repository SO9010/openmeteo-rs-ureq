use serde_derive::Deserialize;
use serde_derive::Serialize;

/*
https://api.open-meteo.com/v1/elevation?
    latitude=52.52
    &longitude=13.41
*/
/// https://open-meteo.com/en/docs/elevation-api
#[derive(Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ElevationRequest {
    pub latitude: f64,                     // E.g: 52.52
    pub longitude: f64,                    // E.g: 13.41
}
impl ElevationRequest {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        ElevationRequest {
            latitude,
            longitude,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElevationResponse {
    pub elevation: Vec<f64>,
}
