// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use super::permissions::Permissions;
use crate::nested_struct;
use crate::result::{Context, Result};
use serde::Deserialize;

nested_struct! {
    ApiManifest {
        meta (Meta {
            kind (String),
            version (String),
            runtime_version (String),
        }),

        #[serde(default)]
        permissions (Option<Permissions>),

        #[serde(default)]
        middlewares (Vec<Middleware>),

        #[serde(default)]
        authentication (
            Authentication {
                enabled (bool)
            }
        )
    }
}

nested_struct! {
    Middleware {
        script (String)
    }
}

impl ApiManifest {
    pub fn try_from(config_str: &str) -> Result<Self> {
        serde_yaml::from_str(&config_str).context("deserializing api manifest")
    }
}

impl Default for Authentication {
    // SEC: Must default to true.
    fn default() -> Self {
        Self { enabled: true }
    }
}
