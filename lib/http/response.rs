// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use serde::{Deserialize, Serialize};
use crate::result::{Context, Result};

#[derive(Serialize, Deserialize)]
pub struct Response {
    // pub parts: Parts,
    pub body: Vec<u8>,
}
