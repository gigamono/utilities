// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use std::any::Any;

use anyhow::Context;
use hyper::{Body, Request, Response};
use log::error;

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

pub fn handle_panic_error_t<T>(err: Box<dyn Any + Send>) -> std::result::Result<Response<Body>, T> {
    // Guess error type.
    let error_str = if let Some(s) = err.downcast_ref::<&str>() {
        *s
    } else {
        "undetermined panic error type"
    };

    // Log error and return server error.
    error!("{:?}", error_str);
    Ok(HandlerError::Internal {
        src: errors::new_error(format!("{:?}", error_str)),
        ctx: errors::HandlerErrorMessage::InternalError,
    }
    .as_hyper_response())
}
