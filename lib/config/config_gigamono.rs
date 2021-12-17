// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use super::config::Meta;
use crate::nested_struct;
use crate::result::{Context, Result};
use serde::Deserialize;
use std::{env, fs};

nested_struct! {
    GigamonoConfig {
        meta (Meta),
        broker (Broker {
            url (String),
            subscriptions (Subscriptions {
                workspaces (EnabledSubscriptions),
                logs (EnabledSubscriptions),
            }),
        }),
        engines (Engines {
            proxy (Proxy {
                socket_address (String),
                workspaces_db_url (String),
            }),
            workspace (Workspace {
                socket_address (String),
                workspaces_db_url (String),
            }),
            backend (Backend {
                socket_address (String),
                root_path (String),
                subscriptions (BackendSubscriptions {
                    workspaces (BackendWorkspaces {
                        run_surl (Vec<String>),
                    }),
                }),
            }),
            db (DB {
                db_url (String),
                subscriptions (DBSubscriptions {
                    workspaces (DBWorkspaces {
                        query (Vec<String>),
                    }),
                }),
            }),
        }),
        web_ui (WebUI {
            dir (String),
        }),
        logs (Logs {
            file (String),
            is_published (String),
        }),
        domain (String),
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
        // TODO(appcypher): Default values. Validation (version, type, etc)
        serde_yaml::from_str(&config_str).context("deserializing gigamono config")
    }
}
