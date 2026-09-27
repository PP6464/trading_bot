use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Instrument {
    pub ticker: String,
    #[serde(rename = "type")]
    pub instrument_type: String,
}