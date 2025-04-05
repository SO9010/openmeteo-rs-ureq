use ureq::Agent;
use std::{io::Read, time::Duration};

use crate::{
    air_quality::AirQualityRequest, climate_change::ClimateChangeRequest, elevation::{ElevationRequest, ElevationResponse}, ensemble_weather::EnsembleWeatherRequest, flood::FloodRequest, geocoding::{GeocodingRequest, GeocodingResponse}, historical_weather::HistoricalWeatherRequest, marine_weather::MarineWeatherRequest, satellite_radiation::SatelliteRadiationRequest, weather::WeatherRequest, weather_api_generated::openmeteo_sdk::{size_prefixed_root_as_weather_api_response, WeatherApiResponse}, OpenMeteoClient
};

/// Weather forcast
impl OpenMeteoClient {
    pub fn get_weather(&self, request: WeatherRequest) -> Result<WeatherApiResponse<'_>, ureq::Error> {
        let mut url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}", request.latitude, request.longitude).to_string(); 
        if let Some(daily) = request.daily {
            let daily_str = daily.join(",");
            url.push_str(&format!("&daily={}", daily_str));
        }
        if let Some(hourly) = request.hourly {
            let hourly_str = hourly.join(",");
            url.push_str(&format!("&hourly={}", hourly_str));
        }
        if let Some(minutely) = request.minutely {
            let minutely_str = minutely.join(",");
            url.push_str(&format!("&minutely_15={}", minutely_str));
        }
        if let Some(current) = request.current {
            let current_str = current.join(",");
            url.push_str(&format!("&current={}", current_str));
        }
        if let Some(models) = request.models {
            url.push_str(&format!("&models={}", models));
        }
        if let Some(timezone) = request.timezone {
            url.push_str(&format!("&timezone={}", timezone));
        }
        if let Some(timeformat) = request.timeformat {
            url.push_str(&format!("&timeformat={}", timeformat));
        }
        if let Some(wind_speed_unit) = request.wind_speed_unit {
            url.push_str(&format!("&wind_speed_unit={}", wind_speed_unit));
        }
        if let Some(temperature_unit) = request.temperature_unit {
            url.push_str(&format!("&temperature_unit={}", temperature_unit));
        }
        if let Some(precipitation_unit) = request.precipitation_unit {
            url.push_str(&format!("&precipitation_unit={}", precipitation_unit));
        }
        if let Some(start_date) = request.start_date {
            url.push_str(&format!("&start_date={}", start_date));
        }
        if let Some(end_date) = request.end_date {
            url.push_str(&format!("&end_date={}", end_date));
        }
        if let Some(past_days) = request.past_days {
            url.push_str(&format!("&past_days={}", past_days));
        }
        if let Some(forecast_days) = request.forecast_days {
            url.push_str(&format!("&forecast_days={}", forecast_days));
        }
        println!("url: {}", url);
        url.push_str("&format=flatbuffers");

        let response = self.agent.get(&url).call()?;
        let mut bytes = Vec::new();
        response.into_body().into_reader().read_to_end(&mut bytes)?;
        
        let bytes = Box::leak(bytes.into_boxed_slice());

        let message = match size_prefixed_root_as_weather_api_response(bytes) {
            Ok(weather) => {
                weather.clone()
            },
            Err(e) => {
                return Err(ureq::Error::BadUri(format!("Failed to parse FlatBuffers message: {}", e)));
            }
        };
        
        Ok(message)
    }
}

/// Historical weather
impl OpenMeteoClient {
    pub fn get_historical_weather(&self, request: HistoricalWeatherRequest) -> Result<WeatherApiResponse<'_>, ureq::Error> {
        let mut url = format!("https://archive-api.open-meteo.com/v1/archive?latitude={}&longitude={}", request.latitude, request.longitude).to_string(); 
        if let Some(start_date) = request.start_date {
            url.push_str(&format!("&start_date={}", start_date));
        }
        if let Some(end_date) = request.end_date {
            url.push_str(&format!("&end_date={}", end_date));
        }
        if let Some(daily) = request.daily {
            let daily_str = daily.join(",");
            url.push_str(&format!("&daily={}", daily_str));
        }
        if let Some(hourly) = request.hourly {
            let hourly_str = hourly.join(",");
            url.push_str(&format!("&hourly={}", hourly_str));
        }
        if let Some(models) = request.models {
            url.push_str(&format!("&models={}", models));
        }
        if let Some(timeformat) = request.timeformat {
            url.push_str(&format!("&timeformat={}", timeformat));
        }
        if let Some(wind_speed_unit) = request.wind_speed_unit {
            url.push_str(&format!("&wind_speed_unit={}", wind_speed_unit));
        }
        if let Some(temperature_unit) = request.temperature_unit {
            url.push_str(&format!("&temperature_unit={}", temperature_unit));
        }
        if let Some(precipitation_unit) = request.precipitation_unit {
            url.push_str(&format!("&precipitation_unit={}", precipitation_unit));
        }
        if let Some(temporal_resolution) = request.temporal_resolution {
            url.push_str(&format!("&temporal_resolution={}", temporal_resolution));
        }
        if let Some(cell_selection) = request.cell_selection {
            url.push_str(&format!("&cell_selection={}", cell_selection));
        }
        if let Some(tilt) = request.tilt {
            url.push_str(&format!("&tilt={}", tilt));
        }
        if let Some(azimuth) = request.azimuth {
            url.push_str(&format!("&azimuth={}", azimuth));
        }
        url.push_str("&format=flatbuffers");

        let response = self.agent.get(&url).call()?;
        let mut bytes = Vec::new();
        response.into_body().into_reader().read_to_end(&mut bytes)?;
        
        let bytes = Box::leak(bytes.into_boxed_slice());

        let message = match size_prefixed_root_as_weather_api_response(bytes) {
            Ok(weather) => {
                weather.clone()
            },
            Err(e) => {
                return Err(ureq::Error::BadUri(format!("Failed to parse FlatBuffers message: {}", e)));
            }
        };
        
        Ok(message)
    }
}

/// Marine weather
impl OpenMeteoClient {
    pub fn get_marine_weather(&self, request: MarineWeatherRequest) -> Result<WeatherApiResponse<'_>, ureq::Error> {
        let mut url = format!("https://marine-api.open-meteo.com/v1/marine?latitude={}&longitude={}", request.latitude, request.longitude).to_string(); 
        if let Some(daily) = request.daily {
            let daily_str = daily.join(",");
            url.push_str(&format!("&daily={}", daily_str));
        }
        if let Some(hourly) = request.hourly {
            let hourly_str = hourly.join(",");
            url.push_str(&format!("&hourly={}", hourly_str));
        }
        if let Some(current) = request.current {
            let current_str = current.join(",");
            url.push_str(&format!("&current={}", current_str));
        }
        if let Some(models) = request.models {
            url.push_str(&format!("&models={}", models));
        }
        if let Some(timeformat) = request.timeformat {
            url.push_str(&format!("&timeformat={}", timeformat));
        }
        if let Some(length_unit) = request.length_unit {
            url.push_str(&format!("&length_unit={}", length_unit));
        }
        if let Some(wind_speed_unit) = request.wind_speed_unit {
            url.push_str(&format!("&wind_speed_unit={}", wind_speed_unit));
        }
        if let Some(past_days) = request.past_days {
            url.push_str(&format!("&past_days={}", past_days));
        }
        if let Some(forecast_days) = request.forecast_days {
            url.push_str(&format!("&forecast_days={}", forecast_days));
        }
        if let Some(past_minutely) = request.past_minutely {
            url.push_str(&format!("&past_minutely={}", past_minutely));
        }
        if let Some(forecast_minutely) = request.forecast_minutely {
            url.push_str(&format!("&forecast_minutely={}", forecast_minutely));
        }
        if let Some(forecast_hours) = request.forecast_hours {
            url.push_str(&format!("&forecast_hours={}", forecast_hours));
        }
        if let Some(past_hours) = request.past_hours {
            url.push_str(&format!("&past_hours={}", past_hours));
        }
        if let Some(temporal_resolution) = request.temporal_resolution {
            url.push_str(&format!("&temporal_resolution={}", temporal_resolution));
        }
        if let Some(cell_selection) = request.cell_selection {
            url.push_str(&format!("&cell_selection={}", cell_selection));
        }
        if let Some(start_date) = request.start_date {
            url.push_str(&format!("&start_date={}", start_date));
        }
        if let Some(end_date) = request.end_date {
            url.push_str(&format!("&end_date={}", end_date));
        }
        if let Some(minutely) = request.minutely {
            let minutely_str = minutely.join(",");
            url.push_str(&format!("&minutely_15={}", minutely_str));
        }
        url.push_str("&format=flatbuffers");

        let response = self.agent.get(&url).call()?;
        let mut bytes = Vec::new();
        response.into_body().into_reader().read_to_end(&mut bytes)?;
        
        let bytes = Box::leak(bytes.into_boxed_slice());

        let message = match size_prefixed_root_as_weather_api_response(bytes) {
            Ok(weather) => {
                weather.clone()
            },
            Err(e) => {
                return Err(ureq::Error::BadUri(format!("Failed to parse FlatBuffers message: {}", e)));
            }
        };
        
        Ok(message)
    }
}

/// Ensemble weather
impl OpenMeteoClient {
    pub fn get_ensemble_weather(&self, request: EnsembleWeatherRequest) -> Result<WeatherApiResponse<'_>, ureq::Error> {
        let mut url = format!("https://ensemble-api.open-meteo.com/v1/ensemble?latitude={}&longitude={}", request.latitude, request.longitude).to_string(); 
        if let Some(daily) = request.daily {
            let daily_str = daily.join(",");
            url.push_str(&format!("&daily={}", daily_str));
        }
        if let Some(hourly) = request.hourly {
            let hourly_str = hourly.join(",");
            url.push_str(&format!("&hourly={}", hourly_str));
        }
        if let Some(models) = request.models {
            url.push_str(&format!("&models={}", models));
        }
        if let Some(timeformat) = request.timeformat {
            url.push_str(&format!("&timeformat={}", timeformat));
        }
        if let Some(wind_speed_unit) = request.wind_speed_unit {
            url.push_str(&format!("&wind_speed_unit={}", wind_speed_unit));
        }
        if let Some(temperature_unit) = request.temperature_unit {
            url.push_str(&format!("&temperature_unit={}", temperature_unit));
        }
        if let Some(precipitation_unit) = request.precipitation_unit {
            url.push_str(&format!("&precipitation_unit={}", precipitation_unit));
        }
        if let Some(forecast_hours) = request.forecast_hours {
            url.push_str(&format!("&forecast_hours={}", forecast_hours));
        }
        if let Some(past_hours) = request.past_hours {
            url.push_str(&format!("&past_hours={}", past_hours));
        }
        if let Some(forecast_days) = request.forecast_days {
            url.push_str(&format!("&forecast_days={}", forecast_days));
        }
        if let Some(past_days) = request.past_days {
            url.push_str(&format!("&past_days={}", past_days));
        }
        if let Some(start_date) = request.start_date {
            url.push_str(&format!("&start_date={}", start_date));
        }
        if let Some(end_date) = request.end_date {
            url.push_str(&format!("&end_date={}", end_date));
        }
        if let Some(temporal_resolution) = request.temporal_resolution {
            url.push_str(&format!("&temporal_resolution={}", temporal_resolution));
        }
        if let Some(cell_selection) = request.cell_selection {
            url.push_str(&format!("&cell_selection={}", cell_selection));
        }
        if let Some(tilt) = request.tilt {
            url.push_str(&format!("&tilt={}", tilt));
        }
        if let Some(azimuth) = request.azimuth {
            url.push_str(&format!("&azimuth={}", azimuth));
        }
        url.push_str("&format=flatbuffers");

        let response = self.agent.get(&url).call()?;
        let mut bytes = Vec::new();
        response.into_body().into_reader().read_to_end(&mut bytes)?;
        
        let bytes = Box::leak(bytes.into_boxed_slice());

        let message = match size_prefixed_root_as_weather_api_response(bytes) {
            Ok(weather) => {
                weather.clone()
            },
            Err(e) => {
                return Err(ureq::Error::BadUri(format!("Failed to parse FlatBuffers message: {}", e)));
            }
        };
        
        Ok(message)
    }
}

/// Satellite radiation
impl OpenMeteoClient {
    pub fn get_satellite_radiation(&self, request: SatelliteRadiationRequest) -> Result<WeatherApiResponse<'_>, ureq::Error> {
        let mut url = format!("https://satellite-api.open-meteo.com/v1/archive?latitude={}&longitude={}", request.latitude, request.longitude).to_string(); 
        if let Some(daily) = request.daily {
            let daily_str = daily.join(",");
            url.push_str(&format!("&daily={}", daily_str));
        }
        if let Some(hourly) = request.hourly {
            let hourly_str = hourly.join(",");
            url.push_str(&format!("&hourly={}", hourly_str));
        }
        if let Some(models) = request.models {
            url.push_str(&format!("&models={}", models));
        }
        if let Some(tilt) = request.tilt {
            url.push_str(&format!("&tilt={}", tilt));
        }
        if let Some(azimuth) = request.azimuth {
            url.push_str(&format!("&azimuth={}", azimuth));
        }
        if let Some(timeformat) = request.timeformat {
            url.push_str(&format!("&timeformat={}", timeformat));
        }
        if let Some(forecast_hours) = request.forecast_hours {
            url.push_str(&format!("&forecast_hours={}", forecast_hours));
        }
        if let Some(past_hours) = request.past_hours {
            url.push_str(&format!("&past_hours={}", past_hours));
        }
        if let Some(temporal_resolution) = request.temporal_resolution {
            url.push_str(&format!("&temporal_resolution={}", temporal_resolution));
        }
        if let Some(cell_selection) = request.cell_selection {
            url.push_str(&format!("&cell_selection={}", cell_selection));
        }
        if let Some(past_minutely) = request.past_minutely {
            url.push_str(&format!("&past_minutely_15={}", past_minutely));
        }
        if let Some(forecast_minutely) = request.forecast_minutely {
            url.push_str(&format!("&forecast_minutely_15={}", forecast_minutely));
        }
        if let Some(start_date) = request.start_date {
            url.push_str(&format!("&start_date={}", start_date));
        }
        if let Some(end_date) = request.end_date {
            url.push_str(&format!("&end_date={}", end_date));
        }
        url.push_str("&format=flatbuffers");

        let response = self.agent.get(&url).call()?;
        let mut bytes = Vec::new();
        response.into_body().into_reader().read_to_end(&mut bytes)?;
        
        let bytes = Box::leak(bytes.into_boxed_slice());

        let message = match size_prefixed_root_as_weather_api_response(bytes) {
            Ok(weather) => {
                weather.clone()
            },
            Err(e) => {
                return Err(ureq::Error::BadUri(format!("Failed to parse FlatBuffers message: {}", e)));
            }
        };
        
        Ok(message)
    }
}

/// Climate Change
impl OpenMeteoClient {
    pub fn get_climate_change(&self, request: ClimateChangeRequest) -> Result<WeatherApiResponse<'_>, ureq::Error> {
        let mut url = format!(
            "https://climate-api.open-meteo.com/v1/climate?latitude={}&longitude={}",
            request.latitude, request.longitude
        );
        if let Some(start_date) = request.start_date {
            url.push_str(&format!("&start_date={}", start_date));
        }
        if let Some(end_date) = request.end_date {
            url.push_str(&format!("&end_date={}", end_date));
        }
        if let Some(daily) = request.daily {
            let daily_str = daily.join(",");
            url.push_str(&format!("&daily={}", daily_str));
        }
        if let Some(models) = request.models {
            url.push_str(&format!("&models={}", models));
        }
        if let Some(timeformat) = request.timeformat {
            url.push_str(&format!("&timeformat={}", timeformat));
        }
        if let Some(wind_speed_unit) = request.wind_speed_unit {
            url.push_str(&format!("&wind_speed_unit={}", wind_speed_unit));
        }
        if let Some(temperature_unit) = request.temperature_unit {
            url.push_str(&format!("&temperature_unit={}", temperature_unit));
        }
        if let Some(precipitation_unit) = request.precipitation_unit {
            url.push_str(&format!("&precipitation_unit={}", precipitation_unit));
        }
        url.push_str("&format=flatbuffers");

        let response = self.agent.get(&url).call()?;
        let mut bytes = Vec::new();
        response.into_body().into_reader().read_to_end(&mut bytes)?;
        
        let bytes = Box::leak(bytes.into_boxed_slice());

        let message = match size_prefixed_root_as_weather_api_response(bytes) {
            Ok(weather) => {
                weather.clone()
            },
            Err(e) => {
                return Err(ureq::Error::BadUri(format!("Failed to parse FlatBuffers message: {}", e)));
            }
        };
        
        Ok(message)
    } 
}

/// Air Quality
impl OpenMeteoClient {
    pub fn get_air_quality(&self, request: AirQualityRequest) -> Result<WeatherApiResponse<'_>, ureq::Error> {
        let mut url = format!(
            "https://air-quality-api.open-meteo.com/v1/air-quality?latitude={}&longitude={}",
            request.latitude, request.longitude
        );
        if let Some(hourly) = request.hourly {
            let hourly_str = hourly.join(",");
            url.push_str(&format!("&hourly={}", hourly_str));
        }
        if let Some(current) = request.current {
            let current_str = current.join(",");
            url.push_str(&format!("&current={}", current_str));
        }
        if let Some(past_days) = request.past_days {
            url.push_str(&format!("&past_days={}", past_days));
        }
        if let Some(forecast_days) = request.forecast_days {
            url.push_str(&format!("&forecast_days={}", forecast_days));
        }
        if let Some(domains) = request.domains {
            url.push_str(&format!("&domains={}", domains));
        }
        if let Some(timeformat) = request.timeformat {
            url.push_str(&format!("&timeformat={}", timeformat));
        }
        if let Some(forecast_hours) = request.forecast_hours {
            url.push_str(&format!("&forecast_hours={}", forecast_hours));
        }
        if let Some(past_hours) = request.past_hours {
            url.push_str(&format!("&past_hours={}", past_hours));
        }
        if let Some(temporal_resolution) = request.temporal_resolution {
            url.push_str(&format!("&temporal_resolution={}", temporal_resolution));
        }
        if let Some(cell_selection) = request.cell_selection {
            url.push_str(&format!("&cell_selection={}", cell_selection));
        }
        url.push_str("&format=flatbuffers");

        let response = self.agent.get(&url).call()?;
        let mut bytes = Vec::new();
        response.into_body().into_reader().read_to_end(&mut bytes)?;
        
        let bytes = Box::leak(bytes.into_boxed_slice());

        let message = match size_prefixed_root_as_weather_api_response(bytes) {
            Ok(weather) => {
                weather.clone()
            },
            Err(e) => {
                return Err(ureq::Error::BadUri(format!("Failed to parse FlatBuffers message: {}", e)));
            }
        };
        
        Ok(message)
    }
}

/// Flood
impl OpenMeteoClient {
    pub fn get_flood(&self, request: FloodRequest) -> Result<WeatherApiResponse<'_>, ureq::Error> {
        let mut url = format!(
            "https://flood-api.open-meteo.com/v1/flood?latitude={}&longitude={}",
            request.latitude, request.longitude
        );
        if let Some(daily) = request.daily {
            let daily_str = daily.join(",");
            url.push_str(&format!("&daily={}", daily_str));
        }
        if let Some(models) = request.models {
            url.push_str(&format!("&models={}", models));
        }
        if let Some(timeformat) = request.timeformat {
            url.push_str(&format!("&timeformat={}", timeformat));
        }
        if let Some(past_days) = request.past_days {
            url.push_str(&format!("&past_days={}", past_days));
        }
        if let Some(forecast_days) = request.forecast_days {
            url.push_str(&format!("&forecast_days={}", forecast_days));
        }
        url.push_str("&format=flatbuffers");

        let response = self.agent.get(&url).call()?;
        let mut bytes = Vec::new();
        response.into_body().into_reader().read_to_end(&mut bytes)?;
        
        let bytes = Box::leak(bytes.into_boxed_slice());

        let message = match size_prefixed_root_as_weather_api_response(bytes) {
            Ok(weather) => {
                weather.clone()
            },
            Err(e) => {
                return Err(ureq::Error::BadUri(format!("Failed to parse FlatBuffers message: {}", e)));
            }
        };
        
        Ok(message)
    }
}

/// Elevation
impl OpenMeteoClient {
    pub fn get_elevation(&self, request: ElevationRequest) -> Result<ElevationResponse, ureq::Error> {
        let url = format!(
            "https://api.open-meteo.com/v1/elevation?latitude={}&longitude={}",
            request.latitude, request.longitude
        );

        let body: ElevationResponse = self.agent.get(url)
        .call()?
        .body_mut()
        .read_json::<ElevationResponse>()?;

        Ok(body)
    }
}

/// Geocoding
impl OpenMeteoClient {
    pub fn get_geocoding(&self, request: GeocodingRequest) -> Result<GeocodingResponse, ureq::Error> {
        let mut url = format!("https://geocoding-api.open-meteo.com/v1/search?name={}", request.name);
        if let Some(count) = request.count {
            url.push_str(&format!("&count={}", count));
        }
        if let Some(language) = request.language {
            url.push_str(&format!("&language={}", language));
        }
        if let Some(format) = request.format {
            url.push_str(&format!("&format={}", format));
        }
        if let Some(country_code) = request.country_code {
            url.push_str(&format!("&countryCode={}", country_code));
        }

        let body: GeocodingResponse = self.agent.get(url)
        .call()?
        .body_mut()
        .read_json::<GeocodingResponse>()?;

        Ok(body)
    }
}