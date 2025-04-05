use openmeteo_rs_ureq::{MarineWeatherRequest, OpenMeteoClient};

fn main() {
    let client = OpenMeteoClient::default();
    let request = MarineWeatherRequest::new(41.0, -7.5)
        .add_hourly("wave_height")
        .add_hourly("wave_direction")
        .add_hourly("wave_period")
        .add_daily("wave_height_max")
        .forecast_days(5);
    
    println!("Got marine weather for 41.0, -7.5: {:#?}\n", client.get_marine_weather(request).unwrap());
}
