use crate::prelude::*;
use ureq::serde_json;
use ureq::Response;

pub fn get_weather(location: Location) -> Option<Weather> {
    let req_url: String = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,apparent_temperature,wind_speed_10m,relative_humidity_2m&forecast_days=1", location.latitude, location.longitude);
    let res: Response = ureq::get(req_url.as_str()).call().unwrap();
    let json: serde_json::Value = res.into_json().unwrap();

    let weather_json: &serde_json::Value = &json["current"];
    return Weather::from_json(location, weather_json);
}
