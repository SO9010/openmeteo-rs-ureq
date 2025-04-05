use openmeteo_rs_ureq::{ElevationRequest, OpenMeteoClient};

fn main() {
    let client = OpenMeteoClient::default();
    let request = ElevationRequest::new(52.52, 13.41);
    println!("Got elevation for 52.52, 13.41: {:#?}\n", client.get_elevation(request).unwrap());
}
