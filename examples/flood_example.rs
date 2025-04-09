use openmeteo_rs_ureq::{FloodRequest, OpenMeteoClient};

fn main() {
    let client = OpenMeteoClient::default();
    let request = FloodRequest::new(52.52, 13.41)
        .add_daily("river_discharge")
        .add_daily("river_discharge_max")
        .add_daily("river_discharge_min")
        .add_daily("river_discharge_p75");

    println!(
        "Got flood information for 52.52, 13.41: {:#?}\n",
        client.get_flood(request).unwrap().decode_buffer()
    );
}
