use openmeteo_rs_ureq::{GeocodingRequest, OpenMeteoClient};

fn main() {
    let client = OpenMeteoClient::default();
    let request = GeocodingRequest::new("Berlin")
        .country_code("DE")
        .language("en");
    
    println!("Geocoding results for 'Berlin': {:#?}\n", client.get_geocoding(request).unwrap());
}
