use super::config::Meta;
use crate::messages::error::SystemError;
use crate::result::Result;
use crate::nested_struct;
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
            api (API {
                port (u16),
                db_url (String),
                reply_timeout (u64),
            }),
            backend (Backend {
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
        ui (UI {
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
    pub fn new(config_str: &str) -> Result<Self> {
        serde_yaml::from_str(&config_str).map_err(|err| SystemError::Yaml {
            ctx: "deserializing gigamono config".to_string(),
            src: err,
        })
        // TODO: Default values.
    }

    pub fn load() -> Result<Self> {
        let env_var = super::constants::GIGAMONO_CONFIG_PATH_ENV_VAR;

        let path = env::var(env_var).map_err(|err| SystemError::EnvVar {
            ctx: format!(r#"fetching gigamono config env var, "{}""#, env_var),
            src: err,
        })?;

        let file_content = fs::read_to_string(&path).map_err(|err| SystemError::Io {
            ctx: format!(r#"reading gigamono config file, "{}""#, &path),
            src: err,
        })?;

        Self::new(&file_content)
    }
}
