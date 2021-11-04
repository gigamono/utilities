use actix_web::{
    dev::Body,
    error::ResponseError,
    http::{header, StatusCode},
    web::{BytesMut, HttpResponse},
};
use std::{
    borrow::Cow,
    error::Error,
    fmt::{Display, Write},
};
use strum::EnumMessage;
use strum_macros::EnumMessage;

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
        code: StatusCode,
        src: SystemError,
    },
    /// For cases where the handler can't even send a response back but we have to return a HandlerError anyway.
    Critical { src: SystemError },
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

impl Display for HandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Internal { ctx, .. } => write!(f, "{}", ctx),
            Self::Client { ctx, .. } => write!(f, "{}", ctx),
            Self::Critical { src, .. } => write!(f, "{}", src),
        }
    }
}

impl ResponseError for HandlerError {
    fn status_code(&self) -> StatusCode {
        match &self {
            Self::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Client { code, .. } => *code,
            Self::Critical { .. } => unreachable!(),
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut resp = HttpResponse::new(self.status_code());
        let mut buf = BytesMut::new();
        let _ = write!(
            &mut buf,
            r#"{{ "errors": [ {{ "message": "{}" }} ] }}"#,
            self
        );
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json; charset=utf-8"),
        );
        resp.set_body(Body::from(buf))
    }
}

impl Display for HandlerErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_message().unwrap_or(""))
    }
}

pub fn any_error(msg: impl Into<Cow<'static, str>>) -> Result<(), SystemError> {
    Err(CustomError::Any(msg.into()).into())
}

pub fn permission_error(msg: impl Into<Cow<'static, str>>) -> Result<(), SystemError> {
    Err(CustomError::Permissions(msg.into()).into())
}
