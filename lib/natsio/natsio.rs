use std::time::Duration;

use crate::messages::error::SystemError;
use crate::result::Result;
use nats::{Connection, Message, Subscription};

#[derive(Debug)]
pub struct Nats(Connection);

impl Nats {
    pub fn connect(nats_url: &str) -> Result<Self> {
        let conn = nats::connect(nats_url).map_err(|err| SystemError::Io {
            ctx: format!(r#"connecting to broker, "{}""#, nats_url),
            src: err,
        })?;
        Ok(Self(conn))
    }

    pub fn request_timeout(
        &self,
        subject: &str,
        msg: impl AsRef<[u8]>,
        timeout: Duration,
    ) -> Result<Message> {
        self.0
            .request_timeout(subject, msg, timeout)
            .map_err(|err| SystemError::Io {
                ctx: "requesting and waiting for response".to_string(),
                src: err,
            })
    }

    pub fn queue_subscribe(&self, subject: &str, queue: &str) -> Result<Subscription> {
        self.0
            .queue_subscribe(subject, queue)
            .map_err(|err| SystemError::Io {
                ctx: format!(r#"queue subscribing to subject, "{}""#, subject),
                src: err,
            })
    }
}
