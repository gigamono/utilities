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
            Self::Any(msg) => write!(f, "{}", msg),
            Self::Permissions(msg) => write!(f, "{}", msg),
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
        format!(r#"{{ "errors": [ {{ "message": "{}" }} ] }}"#, self)
    }
}

impl Display for HandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Client { ctx, .. } => write!(f, "{}", ctx),
            Self::Internal { ctx, .. } => write!(f, "{}", ctx),
        }
    }
}

impl Display for HandlerErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_message().unwrap_or(""))
    }
}
