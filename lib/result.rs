// Copyright 2021 the Gigamono authors. All rights reserved. GPL-3.0 License.

pub use anyhow::{Context, Result}; // Re-export
use std::result;

pub type HandlerResult<T> = result::Result<T, super::errors::HandlerError>;
