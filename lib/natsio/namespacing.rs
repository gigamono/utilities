// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

// TODO(appcypher): Support group index. e.g. "v1.run_api.workspaces.1.*"

use std::fmt::Display;

use strum::EnumMessage;
use strum_macros::EnumMessage;

#[derive(Debug, EnumMessage)]
pub enum WorkspacesAction {
    #[strum(message = "run_api")]
    RunApi,
}

impl Display for WorkspacesAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_message().unwrap_or("[unknown]"))
    }
}

