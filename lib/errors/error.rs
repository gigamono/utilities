// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use hyper::{Body, Response};
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
    Permission(Cow<'static, str>),
    Type(Cow<'static, str>),
    Missing(Cow<'static, str>),
    LimitExceeded(Cow<'static, str>),
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
            Self::Type(msg) => f.debug_tuple("CustomError::Type").field(msg).finish(),
            Self::Missing(msg) => f.debug_tuple("CustomError::Missing").field(msg).finish(),
            Self::Permission(msg) => f.debug_tuple("CustomError::Permission").field(msg).finish(),
            Self::LimitExceeded(msg) => f
                .debug_tuple("CustomError::LimitExceeded")
                .field(msg)
                .finish(),
        }
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
