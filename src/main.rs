use ureq::serde_json;
use ureq::Response;

struct Location {
    latitude: f64,
    longitude: f64,
}

impl ToString for Location {
    fn to_string(&self) -> String {
        return format!("{}° {}°", self.latitude, self.longitude);
    }
}

struct Weather {
    location: Location,
    temperature_celsius: f32,
    apparent_temperature_celsius: f32,
    wind_speed: f32,
    humidity: f32,
}

enum InputMode {
    Coordinate,
    City,
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

fn main() {
    let location: Location = input_location();
    let weather: Weather = get_weather_from_api(location);
    println!("\n{}", weather.to_string());
}

fn input_location() -> Location {
    println!("1. Use location (latitude, longitude)\n2. Use city name");
    let input_mode: InputMode;

    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<i32>() {
            Ok(1) => {
                input_mode = InputMode::Coordinate;
                break;
            }
            Ok(2) => {
                input_mode = InputMode::City;
                break;
            }
            _ => println!("Invalid input. Please enter 1 or 2."),
        }
    }

    return match input_mode {
        InputMode::Coordinate => read_location(),
        InputMode::City => read_city(),
    };

    fn read_location() -> Location {
        fn read_coordinate(name: &'static str, min_valid: f64, max_valid: f64) -> f64 {
            loop {
                println!("Enter {}: ", name);
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                match input.trim().parse::<f64>() {
                    Ok(num) => {
                        if num < min_valid || num > max_valid {
                            println!(
                                "Invalid input. Please enter a number between {} and {}.",
                                min_valid, max_valid
                            );
                            continue;
                        }
                        return num;
                    }
                    Err(_) => println!("Invalid input. Please enter a number."),
                }
            }
        }

        let latitude: f64 = read_coordinate("latitude", -90.0, 90.0);
        let longitude: f64 = read_coordinate("longitude", -180.0, 180.0);

        return Location {
            latitude,
            longitude,
        };
    }

    fn read_city() -> Location {
        let latitude: f64;
        let longitude: f64;

        loop {
            println!("Enter city name: ");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            let city_input: String = input.trim().to_string();

            let req_url: String = format!("https://geocoding-api.open-meteo.com/v1/search?name={}", city_input);
            let res: Response = ureq::get(req_url.as_str()).call().unwrap();
            let json: serde_json::Value = res.into_json().unwrap();
            let results: &[serde_json::Value]; 

            match &json["results"].as_array() {
                Some(arr) => results = arr,
                None => {
                    println!("No city of that name found...");
                    continue;
                }
            }

            let result: &serde_json::Value = &results[0];

            let name: &str = result["name"].as_str().unwrap();
            let country: &str = result["country"].as_str().unwrap();
            let administration: &str = result["admin1"].as_str().unwrap_or(country);
            println!("Found city: {}, {}, {}", name, administration, country);

            latitude = result["latitude"].as_f64().unwrap();
            longitude = result["longitude"].as_f64().unwrap();

            break;
        }

        return Location {
            latitude,
            longitude,
        };
    }
}

fn get_weather_from_api(location: Location) -> Weather {
    let req_url: String = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,apparent_temperature,wind_speed_10m,relative_humidity_2m&forecast_days=1", location.latitude, location.longitude);
    let res: Response = ureq::get(req_url.as_str()).call().unwrap();
    let json: serde_json::Value = res.into_json().unwrap();

    let weather_json: &serde_json::Value = &json["current"];

    return Weather {
        location: location,
        temperature_celsius: weather_json["temperature_2m"].as_f64().unwrap() as f32,
        apparent_temperature_celsius: weather_json["apparent_temperature"].as_f64().unwrap() as f32,
        wind_speed: weather_json["wind_speed_10m"].as_f64().unwrap() as f32,
        humidity: weather_json["relative_humidity_2m"].as_f64().unwrap() as f32,
    };
}
