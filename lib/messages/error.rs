use std::fmt::Display;

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
}

#[derive(Debug)]
pub enum HandlerError {
    Internal {
        ctx: HandlerErrorMessage,
        src: SystemError,
    },
    Client {
        ctx: HandlerErrorMessage,
    },
}

#[derive(Debug)]
pub enum HandlerErrorMessage {
    Example { code: usize, msg: String },
}

impl Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::System { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::Io { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::Yaml { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::EnvVar { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::Conn { ctx, src } => write!(f, "{}: {}", ctx, src),
        }
    }
}

impl Display for HandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Internal { ctx, src } => write!(f, "{}: {}", ctx, src),
            Self::Client { ctx } => write!(f, "{}", ctx),
        }
    }
}

impl Display for HandlerErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
