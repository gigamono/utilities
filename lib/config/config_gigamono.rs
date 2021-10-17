use super::config::Meta;
use crate::messages::error::SystemError;
use crate::result::Result;
use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Deserialize)]
pub struct GigamonoConfig {
    pub meta: Meta,
    pub broker: Broker,
    pub engines: Engines,
    pub ui: UI,
    pub logs: Logs,
}

#[derive(Debug, Deserialize)]
pub struct Broker {
    pub url: String,
    pub subscriptions: Subscriptions,
}

#[derive(Debug, Deserialize)]
pub struct Subscriptions {
    pub workspaces: EnabledSubscriptions,
    pub logs: EnabledSubscriptions,
}

#[derive(Debug, Deserialize)]
pub struct EnabledSubscriptions {
    version: usize,
}

#[derive(Debug, Deserialize)]
pub struct Engines {
    pub api: API,
    pub backend: Backend,
    pub db: DB,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct API {
    pub port: u16,
    pub db_url: String,
    pub reply_timeout: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backend {
    pub root_path: String,
    pub subscriptions: BackendSubscriptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DB {
    pub db_url: String,
    pub subscriptions: DBSubscriptions,
}

#[derive(Debug, Deserialize)]
pub struct BackendSubscriptions {
    pub workspaces: BackendWorkspaces,
}

#[derive(Debug, Deserialize)]
pub struct DBSubscriptions {
    pub workspaces: DBWorkspaces,
}

#[derive(Debug, Deserialize)]
pub struct BackendWorkspaces {
    pub run: EngineSubscriptions,
}

#[derive(Debug, Deserialize)]
pub struct DBWorkspaces {
    pub query: EngineSubscriptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineSubscriptions {
    pub subscribed_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UI {
    pub dir: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logs {
    pub file: String,
    pub is_published: String,
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
            ctx: format!("fetching gigamono config env var, `{}`", env_var),
            src: err,
        })?;

        let file_content = fs::read_to_string(&path).map_err(|err| SystemError::Io {
            ctx: format!("reading gigamono config file, `{}`", &path),
            src: err,
        })?;

        Self::new(&file_content)
    }
}
