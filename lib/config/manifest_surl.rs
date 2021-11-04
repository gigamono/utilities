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
        // TODO(appcypher): Default values. Validation (version, type, etc)
        serde_yaml::from_str(&config_str).context("deserializing surl manifest")
    }
}
