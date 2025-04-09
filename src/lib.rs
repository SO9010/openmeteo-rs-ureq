//! SDK and api bindings for Open Meteo

use std::time::Duration;
use ureq::Agent;

pub mod air_quality;
pub mod api;
pub mod climate_change;
pub mod elevation;
pub mod ensemble_weather;
pub mod flood;
pub mod geocoding;
pub mod historical_weather;
pub mod marine_weather;
pub mod satellite_radiation;
pub mod weather;
pub mod weather_api_generated;

pub struct OpenMeteoClient {
    agent: Agent,
}

impl Default for OpenMeteoClient {
    fn default() -> Self {
        let config = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(5)))
            .build();
        let agent: Agent = config.into();
        OpenMeteoClient { agent }
    }
}

/// For a lot of the response types, we need to decode the buffer into a flatbuffer message
pub struct WeatherData {
    buffer: Vec<u8>,
}

impl WeatherData {
    pub fn from_buffer(buffer: Vec<u8>) -> Self {
        Self { buffer }
    }

    /// Parse a single size-prefixed flatbuffer messages from the owned buffer into an Open-Meteo API-response
    pub fn decode_buffer(&self) -> Result<WeatherApiResponse, ureq::Error> {
        size_prefixed_root_as_weather_api_response(&self.buffer)
            .map_err(|e| ureq::Error::BadUri(format!("Failed to parse: {}", e)))
    }
}

// Reexporting modules for easier access
pub use air_quality::*;
pub use climate_change::*;
pub use elevation::*;
pub use ensemble_weather::*;
pub use flood::*;
pub use geocoding::*;
pub use historical_weather::*;
pub use marine_weather::*;
pub use satellite_radiation::*;
pub use weather::*;
use weather_api_generated::openmeteo_sdk::{
    WeatherApiResponse, size_prefixed_root_as_weather_api_response,
};
pub use weather_api_generated::*;
