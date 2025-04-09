use openmeteo_rs_ureq::{AirQualityRequest, OpenMeteoClient};

fn main() {
    let client = OpenMeteoClient::default();
    let request = AirQualityRequest::new(52.52, 13.41)
        .add_hourly("pm10")
        .add_current("pm10")
        .temporal_resolution("hourly");
    println!(
        "Got air quality for 52.52, 13.41: {:#?}\n",
        client.get_air_quality(request).unwrap().decode_buffer()
    );
}
