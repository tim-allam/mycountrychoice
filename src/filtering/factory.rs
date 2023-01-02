use crate::filtering::interface::IFilter;
use crate::filtering::params::FilterParams;

use crate::filtering::collection::{
    BetterTaxFilter,
    DefaultFilter,
    DualCitizenshipTaxFilter,
    FreedomTaxCitizenshipiFilter
};

pub struct FilterFactory {
}

impl FilterFactory {
    pub fn generate_filter(filter_params: &Option<FilterParams>) -> Box<dyn IFilter> {
        match filter_params {
            Some(fp) => {
                match *fp {
                    FilterParams::BetterTax(tax) => {
                        Box::new(BetterTaxFilter::new(tax))
                    },
                    FilterParams::DualCitizenshipTax(citizensip, tax) => {
                        Box::new(DualCitizenshipTaxFilter::new(citizensip, tax))
                    },
                    FilterParams::FreedomTaxCitizenship(freedom_value, tax_value, citizenship_value) => {
                        Box::new(
                            FreedomTaxCitizenshipiFilter::new(
                                freedom_value, tax_value, citizenship_value)
                        )
                    }
                } 
            },
            None => {
                Box::new(DefaultFilter::new())
            }

        }
    }
}
