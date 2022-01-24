// Copyright 2021 the Gigamono authors. All rights reserved. GPL-3.0 License.

use crate::nested_struct;
use crate::result::{Context, Result};
use serde::Deserialize;

nested_struct! {
    ThemeConfig {
        meta (Meta {
            kind (String),
            version (String),
        }),

    }
}

impl ThemeConfig {
    pub fn try_from(config_str: &str) -> Result<Self> {
        serde_yaml::from_str(&config_str).context("deserializing theme config")
    }
}
