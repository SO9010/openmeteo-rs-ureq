use openmeteo_rs_ureq::{OpenMeteoClient, SatelliteRadiationRequest};

fn main() {
    let client = OpenMeteoClient::default();
    let request = SatelliteRadiationRequest::new(52.52, 13.41)
        .add_daily("sunrise")
        .add_hourly("shortwave_radiation")
        .models("satellite_radiation_seamless");
    println!("Got satallite radiation for 52.52, 13.41: {:#?}\n", client.get_satellite_radiation(request).unwrap());
}
