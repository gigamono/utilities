use super::config::{Meta, Permissions};
use crate::nested_struct;
use crate::result::{Context, Result};
use serde::Deserialize;

nested_struct! {
    SurlManifest {
        meta (Meta),
        permissions (Permissions),
        middlewares (Vec<String>),
    }
}

impl SurlManifest {
    pub fn new(config_str: &String) -> Result<Self> {
        serde_yaml::from_str(&config_str).context("deserializing surl manifest")
    }

    pub fn load_config() -> Self {
        todo!()
    }
}
