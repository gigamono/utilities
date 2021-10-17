use super::config::Meta;
use crate::messages::error::SystemError;
use crate::result::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurlManifest {
    pub meta: Meta,
    pub middleware_scripts: Vec<String>,
}

impl SurlManifest {
    pub fn new(config_str: &String) -> Result<Self> {
        serde_yaml::from_str(&config_str).map_err(|err| SystemError::Yaml {
            ctx: "deserializing surl manifest".to_string(),
            src: err,
        })
    }

    pub fn load_config() -> Self {
        todo!()
    }
}
