use super::config::{Meta, Permissions};
use crate::messages::error::SystemError;
use crate::nested_struct;
use crate::result::Result;
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
        serde_yaml::from_str(&config_str).map_err(|err| SystemError::Yaml {
            ctx: "deserializing surl manifest".to_string(),
            src: err,
        })
    }

    pub fn load_config() -> Self {
        todo!()
    }
}
