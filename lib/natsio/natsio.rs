use std::time::Duration;

use crate::result::{Context, Result};
use nats::{Connection, Message, Subscription};

#[derive(Debug)]
pub struct Nats(Connection);

impl Nats {
    pub fn connect(nats_url: &str) -> Result<Self> {
        let conn =
            nats::connect(nats_url).context(format!(r#"connecting to broker, "{}""#, nats_url))?;

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
            .context("requesting and waiting for response")
    }

    pub fn queue_subscribe(&self, subject: &str, queue: &str) -> Result<Subscription> {
        self.0
            .queue_subscribe(subject, queue)
            .context(format!(r#"queue subscribing to subject, "{}""#, subject))
    }
}
