use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    year: u32,
    month: u8,
    region: String,
    #[serde(rename = "co2, tons")]
    co2: f64,
    #[serde(rename = "trees, pcs")]
    trees: u64,
}