use anyhow::Result;
use csv::{ReaderBuilder, StringRecord};
use std::fs::File;

pub struct ReadCSV;

impl ReadCSV {
    pub fn read_csv_file(
        file_name: &str,
        has_headers: bool,
        delimiter: u8,
    ) -> Result<Vec<StringRecord>> {
        // Open the CSV file
        let file = File::open(file_name)?;

        let mut reader = ReaderBuilder::new()
            .has_headers(has_headers)
            .delimiter(delimiter)
            .from_reader(file);

        Ok(reader.records().filter_map(|row| row.ok()).collect())
    }
}
