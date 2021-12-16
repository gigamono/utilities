// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use std::{borrow::Cow, error::Error};

use super::{CustomError, SystemError};

/// Takes a generic error and wraps its in SystemError.
pub fn wrap_error<T: Error + Send + Sync + 'static>(
    msg: impl Into<Cow<'static, str>>,
    err: T,
) -> SystemError {
    SystemError::new(err).context(msg.into())
}

pub fn new_error(msg: impl Into<Cow<'static, str>>) -> SystemError {
    CustomError::Any(msg.into()).into()
}

pub fn new_error_t<T>(msg: impl Into<Cow<'static, str>>) -> Result<T, SystemError> {
    Err(CustomError::Any(msg.into()).into())
}

pub fn permission_error_t<T>(msg: impl Into<Cow<'static, str>>) -> Result<T, SystemError> {
    Err(CustomError::Permission(msg.into()).into())
}

pub fn type_error_t<T>(msg: impl Into<Cow<'static, str>>) -> Result<T, SystemError> {
    Err(CustomError::Type(msg.into()).into())
}

pub fn missing_error_t<T>(msg: impl Into<Cow<'static, str>>) -> Result<T, SystemError> {
    Err(CustomError::Missing(msg.into()).into())
}

pub fn limit_exceeded_error_t<T>(msg: impl Into<Cow<'static, str>>) -> Result<T, SystemError> {
    Err(CustomError::LimitExceeded(msg.into()).into())
}
