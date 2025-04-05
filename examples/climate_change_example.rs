use openmeteo_rs_ureq::{ClimateChangeRequest, OpenMeteoClient};

fn main() {
    let client = OpenMeteoClient::default();
    let request = ClimateChangeRequest::new(52.52, 13.41)
        .add_daily("temperature_2m_max")
        .add_daily("precipitation_sum")
        .start_date("2023-01-01")
        .end_date("2050-12-31")
        .models("CMCC_CM2_VHR4,EC_Earth3P_HR")
        .temperature_unit("celsius");
    
    println!("Got climate change projection for 52.52, 13.41: {:#?}\n", client.get_climate_change(request).unwrap());
}

