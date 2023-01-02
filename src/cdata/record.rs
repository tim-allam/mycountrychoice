use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    pub country: String,
    pub travel_power: u16,
    pub taxation: u8,
    pub perception: u8,
    pub dual_citizenship: u8,
    pub freedom: u8
}