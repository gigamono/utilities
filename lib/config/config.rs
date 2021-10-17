use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub kind: String,
    pub version: String,
}
