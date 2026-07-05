use serde::Serialize;

#[derive(Serialize)]
pub struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

impl City {
    pub fn new(name: String, population: usize, latitude: f64, longitude: f64) -> Self {
        Self {
            name,
            population,
            latitude,
            longitude,
        }
    }
}
