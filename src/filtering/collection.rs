use crate::cdata::record::Record;
use crate::filtering::interface::IFilter;

pub struct BetterTaxFilter {
    tax: u8
}
impl BetterTaxFilter {
    pub fn new(tax_value: u8) -> Self {
        BetterTaxFilter { tax: tax_value }
    }
}

impl IFilter for BetterTaxFilter {
    fn process(&self, records: &[Record]) -> Vec<Record> {
        records
            .iter()
            .filter(|r| r.taxation >= self.tax)
            .cloned()
            .collect()
    }
}

pub struct DualCitizenshipTaxFilter {
    dual_citizenship: u8,
    tax: u8,
}

impl DualCitizenshipTaxFilter {
    pub fn new(dual_citizenship_value: u8, tax_value: u8) -> Self {
        DualCitizenshipTaxFilter {
            dual_citizenship: dual_citizenship_value, 
            tax: tax_value
        }
   }
}

impl IFilter for DualCitizenshipTaxFilter {
    fn process(&self, records: &[Record]) -> Vec<Record> {
        records
            .iter()
            .filter(|r| (r.taxation >= self.tax) &&
                    (r.dual_citizenship >= self.dual_citizenship)
            )
            .cloned()
            .collect()
    }
}

pub struct FreedomTaxCitizenshipiFilter {
    freedom: u8,
    tax: u8,
    dual_citizenship: u8
}
impl FreedomTaxCitizenshipiFilter {
    pub fn new(freedom_value: u8,
               tax_value: u8,
               dual_citizenship_value: u8) -> Self {

        FreedomTaxCitizenshipiFilter {
            freedom: freedom_value,
            tax: tax_value,
            dual_citizenship: dual_citizenship_value
        }
    }
}

impl IFilter for FreedomTaxCitizenshipiFilter {
    fn process(&self, input_records: &[Record]) -> Vec<Record> {
        input_records
            .iter()
            .filter(|r|
                r.freedom  >= self.freedom &&
                r.taxation >= self.tax     &&
                r.dual_citizenship  >= self.dual_citizenship
            )
            .cloned()
            .collect()
    }
}

pub struct DefaultFilter {}
impl DefaultFilter {
    pub fn new() -> Self {
        DefaultFilter {}
    }
}

impl IFilter for DefaultFilter {
    fn process(&self, input_records: &[Record]) -> Vec<Record> {
        input_records
            .iter()
            .filter(|r| 
                    r.dual_citizenship >= 30
                    && r.taxation >= 30
                    && r.freedom >= 30
                    && r.perception >= 30)
            .cloned()
            .collect()
    }
}
