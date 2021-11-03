use crate::{
    http::HttpRequest,
    result::{Context, Result},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub workspace_id: String,
    pub request: HttpRequest,
}

impl Payload {
    pub fn new(workspace_id: String, request: HttpRequest) -> Self {
        Self {
            workspace_id,
            request,
        }
    }
}

pub fn serialize(payload: &Payload) -> Result<Vec<u8>> {
    bincode::serialize(payload).context("unable to serialize payload")
}

pub fn deserialize(bytes: impl AsRef<[u8]>) -> Result<Payload> {
    bincode::deserialize(bytes.as_ref()).context("unable to deserialize bytes to payload")
}
