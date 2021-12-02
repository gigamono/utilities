// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use crate::result::Result;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{StatusCode, Uri, Version};

#[derive(Serialize, Deserialize, Default)]
pub struct Response {
    pub parts: ResponseParts,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ResponseParts {
    pub method: String,
    pub status_code: StatusCode,
    pub uri: Uri,
    pub version: Version,
    pub headers: HashMap<String, Vec<u8>>,
}

impl Response {
    pub fn to_hyper_response(response: Response) -> Result<hyper::Response<hyper::Body>> {
        todo!()
    }
}
