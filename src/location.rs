use ureq::Response;
use ureq::serde_json;

pub struct Location {
    pub city: String,
    pub administration: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Location {
    pub fn from_city_name(city_name: &str) -> Result<Location, &'static str> {
        let req_url: String = format!(
            "https://geocoding-api.open-meteo.com/v1/search?name={}",
            city_name
        );

        let res: Response = ureq::get(req_url.as_str()).call().unwrap();
        let json: serde_json::Value = res.into_json().unwrap();
        let results: &[serde_json::Value];

        match &json["results"].as_array() {
            Some(arr) => results = arr,
            None => {
                return Err("No city of that name found...");
            }
        }

        let result: &serde_json::Value = &results[0];

        return Ok(Location {
            city: result["name"].as_str().unwrap_or("?").to_string(),
            country: result["country"].as_str().unwrap_or("?").to_string(),
            administration: result["admin1"].as_str().unwrap_or("?").to_string(),
            latitude: result["latitude"].as_f64().unwrap(),
            longitude: result["longitude"].as_f64().unwrap(),
        })
    }
}

impl ToString for Location {
    fn to_string(&self) -> String {
        return format!("{}°, {}° ({}, {}, {})", self.latitude, self.longitude, self.city.as_str(), self.administration.as_str(), self.country.as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_city_name_oslo() {
        let location = Location::from_city_name("Oslo").unwrap();
        assert_eq!(location.city, "Oslo");
        assert_eq!(location.administration, "Oslo County");
        assert_eq!(location.country, "Norway");
    }

    #[test]
    fn from_city_name_copenhagen() {
        let location = Location::from_city_name("Copenhagen").unwrap();
        assert_eq!(location.city, "Copenhagen");
        assert_eq!(location.administration, "Capital Region");
        assert_eq!(location.country, "Denmark");
    }

    #[test]
    fn from_city_name_krakow() {
        let location = Location::from_city_name("Krakow").unwrap();
        assert_eq!(location.city, "Krakow");
        assert_eq!(location.administration, "Lesser Poland");
        assert_eq!(location.country, "Poland");
    }
}
