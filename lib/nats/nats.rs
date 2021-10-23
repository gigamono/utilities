use crate::messages::error::SystemError;
use crate::result::Result;
use nats_lib::Connection;

#[derive(Debug)]
pub struct Nats {
    pub conn: Connection,
}

impl Nats {
    pub fn connect(nats_url: &str) -> Result<Self> {
        let conn = nats_lib::connect(nats_url).map_err(|err| SystemError::Io {
            ctx: format!(r#"connecting to broker, "{}""#, nats_url),
            src: err,
        })?;
        Ok(Self { conn })
    }
}
