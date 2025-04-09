# Open-Meteo API Rust SDK

[![Crates.io](https://img.shields.io/crates/v/openmeteo-rs-ureq.svg)](https://crates.io/crates/openmeteo-rs-ureq)
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
use openmeteo_rs_ureq::{OpenMeteoClient, WeatherRequest};

fn main() {
    let client = OpenMeteoClient::default();
    let request = WeatherRequest::new(52.52, 13.41)
        .add_hourly("temperature_2m")
        .add_hourly("relative_humidity_2m")
        .add_daily("temperature_2m_max")
        .add_daily("temperature_2m_min")
        .add_current("temperature_2m")
        .temperature_unit("celsius")
        .wind_speed_unit("kmh");

    println!(
        "Got weather forecast for 52.52, 13.41: {:#?}\n",
        client.get_weather(request).unwrap().decode_buffer()
    );
}
```

### Historical Weather Data

```rust
use openmeteo_rs_ureq::{HistoricalWeatherRequest, OpenMeteoClient};

fn main() {
    let client = OpenMeteoClient::default();
    let request = HistoricalWeatherRequest::new(52.52, 13.41)
        .add_hourly("temperature_2m")
        .add_daily("temperature_2m_max")
        .start_date("2022-01-01")
        .end_date("2022-01-10")
        .temperature_unit("celsius");

    println!("Got historical weather for 52.52, 13.41: {:#?}\n", client.get_historical_weather(request).unwrap().decode_buffer());
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
