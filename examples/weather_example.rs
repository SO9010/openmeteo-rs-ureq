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
