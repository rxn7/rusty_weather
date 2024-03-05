pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl ToString for Location {
    fn to_string(&self) -> String {
        return format!("{}°, {}°", self.latitude, self.longitude);
    }
}
