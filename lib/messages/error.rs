use actix_web::{
    dev::Body,
    error::ResponseError,
    http::{header, StatusCode},
    web::{BytesMut, HttpResponse},
};
use std::fmt::{Display, Write};
use strum::EnumMessage;
use strum_macros::EnumMessage;

#[derive(Debug)]
pub enum SystemError {
    System {
        ctx: String,
        src: Box<SystemError>,
    },
    Io {
        ctx: String,
        src: std::io::Error,
    },
    Yaml {
        ctx: String,
        src: serde_yaml::Error,
    },
    EnvVar {
        ctx: String,
        src: std::env::VarError,
    },
    Conn {
        ctx: String,
        src: diesel::result::ConnectionError,
    },
    Join {
        ctx: String,
        src: tokio::task::JoinError,
    },
    Generic {
        ctx: String,
    },
    ToStr {
        ctx: String,
        src: actix_web::http::header::ToStrError,
    },
    Diesel {
        ctx: String,
        src: diesel::result::Error,
    },
    ActixWeb {
        ctx: String,
        src: String, // Can't use `actix_web::Error` because it is !Send and gives tokio problem https://gist.github.com/appcypher/09eada7309a0be6835778049099401f4
    },
    Uuid {
        ctx: String,
        src: uuid::Error,
    },
    DenoAny {
        ctx: String,
        src: deno_core::error::AnyError,
    },
    Poison {
        ctx: String,
    },
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

impl Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::System { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::Io { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::Yaml { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::EnvVar { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::Conn { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::Join { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::ToStr { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::Diesel { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::ActixWeb { ctx, src } => write!(f, "{}: {:?}", ctx, src),
            Self::Uuid { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::DenoAny { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::Poison { ctx } => write!(f, "{}", ctx),
            Self::Generic { ctx } => write!(f, "{}", ctx),
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
