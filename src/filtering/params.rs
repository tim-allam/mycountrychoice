pub enum FilterParams {
    BetterTax(u8),
    DualCitizenshipTax(u8, u8),
    FreedomTaxCitizenship(u8, u8, u8)
}

impl From<u8> for FilterParams {
    fn from(tax_value: u8) -> Self {
        FilterParams::BetterTax(tax_value)
    }
}

impl From<(u8, u8)> for FilterParams {
    fn from((dual_citizenship_value, 
            tax_value): (u8, u8)) -> Self 
    {
        FilterParams::DualCitizenshipTax(
            dual_citizenship_value, tax_value
        )
    }
}

impl From<(u8, u8, u8)> for FilterParams {
    fn from((freedom_value,
            tax_value,
            citizenship_value): (u8, u8, u8)) -> Self 
    {
        FilterParams::FreedomTaxCitizenship(
            freedom_value,
            tax_value,
            citizenship_value
        )
    }
}
