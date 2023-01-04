use std::fs;

use crate::cdata::record::Record;

use super::interface::{CountryRecordReader, ReaderResult}; 

// Definition of the CSV structre
pub struct CSVReader<'a> {
    filename: &'a str
}

impl<'a> CSVReader<'a> {
    pub fn new(filename_p: &'a str) -> Self {
        CSVReader {
            filename: filename_p
        }
    }
}

impl CountryRecordReader for CSVReader<'_> {
    fn extract_records(&self) -> ReaderResult {
        let data = fs::read_to_string(self.filename)?;
        let mut reader = csv::Reader::from_reader(data.as_bytes());

        let valid_records: Vec<Record> = 
            reader.deserialize()
            .into_iter()
            .filter_map(|r| match r {
                Ok(rec) => rec,
                Err(_) => None,
            })
            .collect();

        Ok(valid_records)
    }
}
