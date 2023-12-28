extern crate haversine;

use haversine::{distance, Location};

pub fn calculate_distance(location1: Location, location2: Location) -> i64 {
    (distance(location1, location2, haversine::Units::Kilometers) * 1000.0) as i64
}

pub fn get_location(lat: f64, lon: f64) -> Location {
    haversine::Location {
        latitude: lat,
        longitude: lon,
    }
}
