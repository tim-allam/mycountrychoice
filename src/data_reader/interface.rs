use crate::cdata::record::Record;

pub type ReaderResult = Result<Vec<Record>, csv::Error>;

pub trait CountryRecordReader {
    fn extract_records(&self) -> ReaderResult;
}
