use openmeteo_rs_ureq::{HistoricalWeatherRequest, OpenMeteoClient};

fn main() {
    let client = OpenMeteoClient::default();
    let request = HistoricalWeatherRequest::new(52.52, 13.41)
        .add_hourly("temperature_2m")
        .add_daily("temperature_2m_max")
        .start_date("2022-01-01")
        .end_date("2022-01-10")
        .temperature_unit("celsius");

    println!(
        "Got historical weather for 52.52, 13.41: {:#?}\n",
        client
            .get_historical_weather(request)
            .unwrap()
            .decode_buffer()
    );
}
