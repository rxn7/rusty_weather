use crate::prelude::*;
use ureq::serde_json;

pub struct Weather {
    pub location: Location,
    pub temperature_celsius: f32,
    pub apparent_temperature_celsius: f32,
    pub wind_speed: f32,
    pub humidity: f32,
}

impl Weather {
    pub fn from_json(location: Location, json: &serde_json::Value) -> Weather {
        return Weather {
            location,
            temperature_celsius: json["temperature_2m"].as_f64().unwrap() as f32,
            apparent_temperature_celsius: json["apparent_temperature"].as_f64().unwrap() as f32,
            wind_speed: json["wind_speed_10m"].as_f64().unwrap() as f32,
            humidity: json["relative_humidity_2m"].as_f64().unwrap() as f32,
        };
    }
}

impl ToString for Weather {
    fn to_string(&self) -> String {
        return format!(
            "Location: {}\nTemperature: {}°C\nApparent Temperature: {}°C\nWind Speed: {}m/s\nHumidity: {}%",
            self.location.to_string(),
            self.temperature_celsius,
            self.apparent_temperature_celsius,
            self.wind_speed,
            self.humidity
        );
    }
}
