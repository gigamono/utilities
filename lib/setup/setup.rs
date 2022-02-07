// Copyright 2021 the Gigamono authors. All rights reserved. GPL-3.0 License.

use crate::config::GigamonoConfig;
use crate::result::{Context, Result};

pub struct CommonSetup {
    pub nats: async_nats::Connection,
    pub config: GigamonoConfig,
}

impl CommonSetup {
    pub async fn new() -> Result<Self> {
        // Load gigamono config file.
        let config = GigamonoConfig::load()?;

        // Connect to NATS.
        let broker_socket_address = &config.broker.socket_address;
        let nats = async_nats::connect(broker_socket_address)
            .await
            .context(format!(
                r#"connecting to broker, "{}""#,
                broker_socket_address
            ))?;

        Ok(Self { nats, config })
    }
}
