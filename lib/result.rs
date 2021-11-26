// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

pub use anyhow::{Context, Result}; // Re-export
use std::result;

pub type HandlerResult<T> = result::Result<T, super::errors::HandlerError>;
