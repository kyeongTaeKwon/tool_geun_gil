use crate::location::{calculate_distance, get_location};
use crate::parser::ReadCSV;
use csv::StringRecord;
use std::collections::HashMap;

pub struct GeoCoordinate {
    lat: f64,
    lon: f64,
}
pub struct NearestBusStop {
    nearest: Option<StringRecord>,
}

impl NearestBusStop {
    pub fn new() -> Self {
        Self { nearest: None }
    }

    pub fn find(file: &str) {
        let delimiter = b',';
        let mut values: HashMap<i64, StringRecord> = HashMap::new();
        let mut array: Vec<i64> = Vec::new();

        match ReadCSV::read_csv_file(file, false, delimiter) {
            Ok(rows) => {
                for row in &rows {
                    let my_location = get_location(37.517030, 127.113625);

                    let lat = row[2].parse::<f64>().unwrap_or(1000.0);
                    let lon = row[3].parse::<f64>().unwrap_or(1000.0);

                    let location = get_location(lat, lon);
                    let distance = calculate_distance(my_location, location);

                    if distance < 500 {
                        values.insert(distance, row.clone());
                        array.push(distance);
                    }
                }

                array.sort();

                println!("{:?}", values.get(&array[1]));
            }
            Err(e) => println!("{}", e),
        }
    }
}
