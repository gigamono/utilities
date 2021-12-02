// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use std::sync::Arc;

use crate::result::{Context, Result};
use async_nats::Message;

pub fn get_first_from_headers(msg: &Arc<Message>, key: impl AsRef<str>) -> Result<String> {
    // Get the hashset of values from header using provided key.
    let headers = msg
        .headers
        .as_ref()
        .context("getting message header value")?
        .get(key.as_ref());

    // Unwrap optional.
    let values = headers.context("getting message header value")?;

    // Get the first value.
    let value = values
        .iter()
        .next()
        .context("getting message header value")?;

    Ok(value.clone())
}

pub fn has_header_key(msg: &Message, key: impl AsRef<str>) -> bool {
    match msg.headers.as_ref() {
        Some(headers) => headers.contains_key(key.as_ref()),
        None => false,
    }
}
