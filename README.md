# Open-Meteo API Rust SDK

[![Crates.io](https://img.shields.io/crates/v/openmeteo-rs-ureq.svg)](https://crates.io/crates/openmeteo-rs-ureq)
[![Documentation](https://docs.rs/openmeteo-rs-ureq/badge.svg)](https://docs.rs/openmeteo-rs-ureq)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust SDK for the [Open-Meteo](https://open-meteo.com/) Weather API using the `ureq` HTTP client.

This SDK is based on the official [Open-Meteo SDK](https://github.com/open-meteo/sdk) collection and provides a type-safe way to interact with Open-Meteo's free weather forecast APIs.

## Features

- Type-safe API for Open-Meteo services
- Built on the lightweight `ureq` HTTP client
- Support for weather forecast, historical weather data, and more
- Ergonomic Rust interface for all API parameters
- No API key required for most endpoints

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
openmeteo-rs-ureq = "0.1.0"  # replace with current version
```

## Usage

### Basic Weather Forecast

```rust
use openmeteo_rs_ureq::{Client, WeatherForecastRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let forecast = client.weather_forecast(
        WeatherForecastRequest::new()
            .latitude(52.52)
            .longitude(13.41)
            .current(vec!["temperature_2m", "relative_humidity_2m"])
            .hourly(vec!["temperature_2m", "precipitation_probability"])
            .daily(vec!["temperature_2m_max", "temperature_2m_min"])
            .timezone("Europe/Berlin")
    )?;
    
    println!("Current temperature: {} °C", forecast.current().unwrap().temperature_2m().unwrap());
    
    // Access hourly and daily data
    for hour in forecast.hourly().unwrap().iter() {
        println!("Time: {}, Temp: {} °C", hour.time(), hour.temperature_2m().unwrap());
    }
    
    Ok(())
}
```

### Historical Weather Data

```rust
use openmeteo_rs_ureq::{Client, HistoricalWeatherRequest};
use chrono::{NaiveDate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let historical = client.historical_weather(
        HistoricalWeatherRequest::new()
            .latitude(40.71)
            .longitude(-74.01)
            .start_date(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
            .end_date(NaiveDate::from_ymd_opt(2022, 1, 7).unwrap())
            .daily(vec!["temperature_2m_max", "precipitation_sum"])
    )?;
    
    // Process historical data
    
    Ok(())
}
```

## Available APIs

- Weather Forecast API
- Historical Weather API
- Marine Forecast API
- Air Quality API
- Flood API
- Climate Change API

## Documentation

For detailed API documentation, please visit [docs.rs/openmeteo-rs-ureq](https://docs.rs/openmeteo-rs-ureq).

For information about the Open-Meteo API itself, visit [open-meteo.com/en/docs](https://open-meteo.com/en/docs).

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Open-Meteo](https://open-meteo.com/) for providing the free weather API
- The [Open-Meteo SDK](https://github.com/open-meteo/sdk) project that this implementation is based on

