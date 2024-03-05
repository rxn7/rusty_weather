mod api;
mod location;
mod weather;

mod prelude {
    pub use crate::weather::*;
    pub use crate::location::*;
}
use prelude::*;

enum InputMode {
    Coordinate,
    City,
}

fn main() {
    let location: Location = input_location();
    let weather: Weather = api::get_weather(location);
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
        InputMode::Coordinate => read_input_location(),
        InputMode::City => read_input_city(),
    };
}

fn read_input_location() -> Location {
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
        city: "".to_string(),
        administration: "".to_string(),
        country: "".to_string(),
        latitude,
        longitude,
    };
}

fn read_input_city() -> Location {
    loop {
        println!("Enter city name: ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let city_input: String = input.trim().to_string();
        match Location::from_city_name(city_input.as_str()) {
            Ok(loc) => {
                return loc;
            },
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
