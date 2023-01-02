use crate::cdata::record::Record;

pub trait IFilter {
    fn process(&self, country_records: &[Record]) -> Vec<Record>;
}