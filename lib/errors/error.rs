// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use std::{borrow::Cow, error::Error, fmt::Display};
use strum::EnumMessage;
use strum_macros::EnumMessage;

use crate::http::StatusCode;

/// Represents any kind of error that can occur system-wide.
/// It wraps every other error.
pub type SystemError = anyhow::Error;

#[derive(Debug)]
pub enum CustomError {
    Any(Cow<'static, str>),
    Permissions(Cow<'static, str>),
}

#[derive(Debug)]
pub enum HandlerError {
    Internal {
        ctx: HandlerErrorMessage,
        src: SystemError,
    },
    Client {
        ctx: HandlerErrorMessage,
        code: StatusCode, // For HTTP handlers.
        src: SystemError,
    },
}

#[derive(Debug, EnumMessage)]
pub enum HandlerErrorMessage {
    #[strum(message = "internal server error")]
    InternalError,
    #[strum(message = "invalid or missing workspace id")]
    InvalidWorkspaceID,
    #[strum(message = "invalid workspace name")]
    InvalidWorkspaceName,
    #[strum(message = "bad request")]
    BadRequest,
    #[strum(message = "one of authorisation or middleware failed")]
    AuthMiddleware,
    #[strum(message = "resource not found")]
    NotFound,
}

impl Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Any(msg) => f.debug_tuple("CustomError::Any").field(msg).finish(),
            Self::Permissions(msg) => f
                .debug_tuple("CustomError::Permissions")
                .field(msg)
                .finish(),
        }
    }
}

impl HandlerError {
    pub fn status_code(&self) -> u16 {
        match &self {
            Self::Client { code, .. } => code.to_u16(),
            _ => StatusCode::InternalServerError.to_u16(),
        }
    }

    pub fn error_json(&self) -> String {
        let ctx = match self {
            Self::Client { ctx, .. } => ctx,
            Self::Internal { ctx, .. } => ctx,
        };

        format!(r#"{{ "errors": [ {{ "message": "{}" }} ] }}"#, ctx)
    }
}

impl Display for HandlerErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Returning only the message because that is what the client needs to see.
        write!(f, "{}", self.get_message().unwrap_or(""))
    }
}