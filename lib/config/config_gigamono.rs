// Copyright 2021 the Gigamono authors. All rights reserved. GPL-3.0 License.

use crate::nested_struct;
use crate::result::{Context, Result};
use serde::Deserialize;
use std::{env, fs};

nested_struct! {
    GigamonoConfig {
        meta (Meta {
            kind (String),
            version (String),
        }),

        #[serde(default)]
        broker (
            Broker {
                socket_address (String),
            }
        ),

        #[serde(default)]
        engines (Engines {
            #[serde(default)]
            proxy (
                ProxyEngine {
                    socket_address (String),
                }
            ),

            #[serde(default)]
            workspace (
                WorkspaceEngine {
                    socket_address (String),
                }
            ),

            #[serde(default)]
            runtime (
                RuntimeEngine {
                    socket_address (String),
                }
            ),
        }),

        #[serde(default)]
        js_runtime (
            #[derive(Default)]
            JsRuntime {
                enable_snapshot (bool)
            }
        ),

        // TODO(appcypher): Support default.
        db (
            Database {
                user (String),
                host (String),
                port (String),
                multi_workspace (bool),
            }
        ),

        // TODO(appcypher): Support default.
        volume (
            Volume {
                root (String),
                multi_workspace (bool),
            }
        ),

        ui (UI {
            root (String),
        }),
    }
}

#[derive(Debug, Deserialize)]
pub struct EnabledSubscriptions {
    pub version: u16,
}

impl GigamonoConfig {
    pub fn load() -> Result<Self> {
        let env_var = super::constants::GIGAMONO_CONFIG_PATH_ENV_VAR;

        let path = env::var(env_var).context(format!(
            r#"fetching gigamono config env var, "{}""#,
            env_var
        ))?;

        let file_content = fs::read_to_string(&path)
            .context(format!(r#"reading gigamono config file, "{}""#, &path))?;

        Self::try_from(&file_content)
    }

    pub fn try_from(config_str: &str) -> Result<Self> {
        serde_yaml::from_str(&config_str).context("deserializing gigamono config")
    }
}

impl Default for Broker {
    fn default() -> Self {
        Self {
            socket_address: "nats://127.0.0.1:4222".into(),
        }
    }
}

impl Default for Engines {
    fn default() -> Self {
        Self {
            proxy: Default::default(),
            workspace: Default::default(),
            runtime: Default::default(),
        }
    }
}

impl Default for ProxyEngine {
    fn default() -> Self {
        Self {
            socket_address: "127.0.0.1:5050".into(),
        }
    }
}

impl Default for WorkspaceEngine {
    fn default() -> Self {
        Self {
            socket_address: "127.0.0.1:5051".into(),
        }
    }
}

impl Default for RuntimeEngine {
    fn default() -> Self {
        Self {
            socket_address: "127.0.0.1:5052".into(),
        }
    }
}
