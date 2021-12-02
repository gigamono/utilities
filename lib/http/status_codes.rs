// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy)]
pub enum StatusCode {
    Ok = 200,
    InternalServerError = 500,
    BadRequest = 400,
    NotFound = 404,
    Unauthorized = 401,
}

impl StatusCode {
    pub fn to_u16(&self) -> u16 {
        *self as u16
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        Self::Ok
    }
}
