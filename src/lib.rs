//! SDK and api bindings for Open Meteo

use ureq::Agent;
use std::time::Duration;

pub mod api;
pub mod weather;
pub mod historical_weather;
pub mod geocoding;
pub mod elevation;
pub mod climate_change;
pub mod ensemble_weather;
pub mod flood;
pub mod air_quality;
pub mod marine_weather;
pub mod satellite_radiation;
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
pub use weather_api_generated::*;
