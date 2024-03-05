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
    pub fn from_json(location: Location, json: &serde_json::Value) -> Option<Weather> {
        return Some(Weather {
            location,
            temperature_celsius: json["temperature_2m"].as_f64()? as f32,
            apparent_temperature_celsius: json["apparent_temperature"].as_f64()? as f32,
            wind_speed: json["wind_speed_10m"].as_f64()? as f32,
            humidity: json["relative_humidity_2m"].as_f64()? as f32,
        });
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_json_some() {
        let location: Location = Location::from_city_name("Oslo").unwrap();
        let weather: Weather = Weather::from_json(location, &serde_json::json!({ "temperature_2m": 10.0, "apparent_temperature": 9.0, "wind_speed_10m": 1.0, "relative_humidity_2m": 50.0 })).unwrap();
        assert_eq!(weather.temperature_celsius, 10.0);
        assert_eq!(weather.apparent_temperature_celsius, 9.0);
        assert_eq!(weather.wind_speed, 1.0);
        assert_eq!(weather.humidity, 50.0);
    }

    #[test]
    fn from_json_none() {
        let location: Location = Location::from_city_name("Oslo").unwrap();
        let weather = Weather::from_json(location, &serde_json::json!({ "temperature": 10.0, "apparent_temperature": 9.0, "wind_speed_10m": 1.0, "relative_humidity_2m": 50.0 }));

        assert!(weather.is_none());
    }
}
