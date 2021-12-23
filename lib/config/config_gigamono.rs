// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

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

        engines (Engines {
            #[serde(default)]
            proxy (
                Proxy {
                    socket_address (String),
                }
            ),

            #[serde(default)]
            workspace (
                Workspace {
                    socket_address (String),
                }
            ),

            backend (
                Backend {
                    #[serde(default = "Backend::default_sock_addr")]
                    socket_address (String),

                    root_path (String),

                    #[serde(default)]
                    runtime (
                        #[derive(Default)]
                        Runtime {
                            enable_snapshot (bool)
                        }
                    ),
                }
            ),
        }),

        web_ui (WebUI {
            dir (String),
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

impl Backend {
    pub fn default_sock_addr() -> String {
        "127.0.0.1:5052".into()
    }
}

impl Default for Broker {
    fn default() -> Self {
        Self {
            socket_address: "nats://127.0.0.1:4222".into(),
        }
    }
}

impl Default for Proxy {
    fn default() -> Self {
        Self {
            socket_address: "127.0.0.1:5050".into(),
        }
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self {
            socket_address: "127.0.0.1:5051".into(),
        }
    }
}
