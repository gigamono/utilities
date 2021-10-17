use crate::messages::error::{Result, SystemError};
use nats::Connection;

#[derive(Debug)]
pub struct BrokerClient {
    conn: Connection,
}

impl BrokerClient {
    pub fn connect(nats_url: &str) -> Result<Self> {
        let conn = nats::connect(nats_url).map_err(|err| SystemError::Io {
            ctx: format!("connecting to broker, `{}`", nats_url),
            src: err,
        })?;
        Ok(Self { conn })
    }
}
