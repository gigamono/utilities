use std::fmt::Display;

use crate::config::GigamonoConfig;
use strum::EnumMessage;
use strum_macros::EnumMessage;

#[derive(Debug, EnumMessage)]
pub enum WorkspacesAction {
    #[strum(message = "run_surl")]
    RunSurl,
}

impl Display for WorkspacesAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_message().unwrap_or("[unknown]"))
    }
}

pub fn get_workpace_subject<'a>(
    config: &GigamonoConfig,
    action: WorkspacesAction,
    id: Option<&str>,
) -> String {
    let version = config.broker.subscriptions.workspaces.version;
    let tail = if let Some(id) = id {
        format!(".{}", id)
    } else {
        String::new()
    };
    format!("v{}.{}.workspaces{}", version, action, tail)
}

pub fn get_backend_first_sub_target<'a>(
    config: &'a GigamonoConfig,
    action: WorkspacesAction,
) -> Option<&'a str> {
    match action {
        WorkspacesAction::RunSurl => {
            let ids = &config.engines.backend.subscriptions.workspaces.run_surl;
            if !ids.is_empty() {
                Some(&ids[0])
            } else {
                None
            }
        }
    }
}