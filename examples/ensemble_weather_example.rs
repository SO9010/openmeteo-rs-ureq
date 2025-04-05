use openmeteo_rs_ureq::{EnsembleWeatherRequest, OpenMeteoClient};

fn main() {
    let client = OpenMeteoClient::default();
    let request = EnsembleWeatherRequest::new(52.52, 13.41)
        .add_hourly("temperature_2m")
        .add_daily("temperature_2m_max")
        .add_daily("temperature_2m_min");
    
    println!("Got ensemble weather for 52.52, 13.41: {:#?}\n", client.get_ensemble_weather(request).unwrap());
}
