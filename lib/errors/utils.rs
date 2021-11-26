// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use std::{borrow::Cow, error::Error};

use super::{CustomError, SystemError};

/// Takes a generic error and wraps its in SystemError.
pub fn wrap_error<T: Error + Send + Sync + 'static>(
    msg: impl Into<Cow<'static, str>>,
    err: T,
) -> Result<(), SystemError> {
    Err(SystemError::new(err).context(msg.into()))
}

pub fn any_error(msg: impl Into<Cow<'static, str>>) -> Result<(), SystemError> {
    Err(CustomError::Any(msg.into()).into())
}

pub fn permission_error(msg: impl Into<Cow<'static, str>>) -> Result<(), SystemError> {
    Err(CustomError::Permissions(msg.into()).into())
}
