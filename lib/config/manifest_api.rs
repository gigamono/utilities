// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use super::config::{Meta, Permissions};
use crate::nested_struct;
use crate::result::{Context, Result};
use serde::Deserialize;

nested_struct! {
    ApiManifest {
        meta (Meta),
        permissions (Permissions),
        middlewares (Vec<Middleware>),
        authentication (Authentication {
            enabled(bool)
        })
    }
}

nested_struct! {
    Middleware {
        script (String)
    }
}

impl ApiManifest {
    pub fn try_from(config_str: &str) -> Result<Self> {
        // TODO(appcypher): Default values. Validation (version, type, etc)
        serde_yaml::from_str(&config_str).context("deserializing api manifest")
    }
}
