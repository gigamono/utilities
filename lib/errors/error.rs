// Copyright 2021 the Gigamono authors. All rights reserved. GPL-3.0 License.

use hyper::{Body, Response};
use std::{borrow::Cow, error::Error, fmt::Display};
use strum::EnumMessage;
use strum_macros::EnumMessage;

use crate::hyper::StatusCode;

/// Represents any kind of error that can occur system-wide.
/// It wraps every other error.
pub type SystemError = anyhow::Error;

#[derive(Debug)]
pub enum CustomError {
    Any(Cow<'static, str>),
    Permission(Cow<'static, str>),
    Type(Cow<'static, str>),
    Missing(Cow<'static, str>),
    LimitExceeded(Cow<'static, str>),
}

// TODO(appcypher): Move to error/http.rs
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

// TODO(appcypher): Move to error/http.rs
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
    #[strum(message = "expected a GET method")]
    GetMethodExpected,
    #[strum(message = "invalid file path")]
    InvalidFilePath,
}

impl Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl HandlerError {
    pub fn status_code(&self) -> u16 {
        match &self {
            Self::Client { code, .. } => code.as_u16(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }

    pub fn error_json(&self) -> String {
        let ctx = match self {
            Self::Client { ctx, .. } => ctx,
            Self::Internal { ctx, .. } => ctx,
        };

        format!(r#"{{ "errors": [ {{ "message": "{}" }} ] }}"#, ctx)
    }

    pub fn as_hyper_response(&self) -> Response<Body> {
        Response::builder()
            .header("Content-Type", "application/json")
            .status(self.status_code())
            .body(Body::from(self.error_json()))
            .unwrap()
    }

    pub fn system_error(&self) -> &SystemError {
        match &self {
            Self::Client { src, .. } => src,
            Self::Internal { src, .. } => src,
        }
    }
}

impl Display for HandlerErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Returning only the message because that is what the client needs to see.
        write!(f, "{}", self.get_message().unwrap_or(""))
    }
}
