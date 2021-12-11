// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use anyhow::Context;
use hyper::{Body, Request};

use crate::{
    errors::{self, HandlerError, HandlerErrorMessage, SystemError},
    result::Result,
};

pub fn parse_url_path_number(path: &str) -> Result<usize> {
    // Trim one or more "/" from the prefix.
    let trimmed_path = path.trim_start_matches("/");

    // Get the index of the next "/".
    let index = trimmed_path.find("/").unwrap_or(0);

    // Get the substring up until "/".
    let sub = &trimmed_path[..index];

    // Parse substring.
    let num = sub.parse::<usize>()?;

    Ok(num)
}

pub fn get_header_value(request: &Request<Body>, key: impl AsRef<str>) -> Result<String> {
    let key = key.as_ref();

    match request.headers().get(key) {
        Some(value) => {
            let value = value
                .to_str()
                .context(format!(r#"converting header value of "{}" to string"#, key))?;

            Ok(value.to_owned())
        }
        None => errors::new_error_t(format!(r#"header key does not exist "{}""#, key)),
    }
}

pub fn internal_error(err: SystemError) -> HandlerError {
    HandlerError::Internal {
        ctx: HandlerErrorMessage::InternalError,
        src: err,
    }
}
