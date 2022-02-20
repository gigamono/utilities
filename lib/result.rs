// Copyright 2021 the Gigamono authors. All rights reserved. GPL-3.0 License.

pub use anyhow::{Context, Result}; // Re-export

pub type HandlerResult<T> = std::result::Result<T, super::errors::HandlerError>;
