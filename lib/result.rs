pub use anyhow::{Context, Result}; // Re-export
use std::result;

pub type HandlerResult<T> = result::Result<T, super::errors::HandlerError>;
