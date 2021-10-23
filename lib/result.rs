use std::result;

pub type Result<T> = result::Result<T, super::messages::error::SystemError>;
pub type HandlerResult<T> = result::Result<T, super::messages::error::HandlerError>;
