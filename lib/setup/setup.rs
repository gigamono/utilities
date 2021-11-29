// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use parking_lot::Mutex;

use crate::config::GigamonoConfig;
use crate::database::DB;
use crate::result::{Context, Result};

use diesel::pg::PgConnection;

pub struct SharedSetup {
    pub nats: async_nats::Connection,
    pub config: GigamonoConfig,
}

pub struct RouterSetup {
    pub common: SharedSetup,
    pub db: Mutex<DB<PgConnection>>, // SQliteConnection contains Cell/RefCell
}

impl SharedSetup {
    pub async fn new() -> Result<Self> {
        let config = GigamonoConfig::load()?;

        let broker_url = &config.broker.url;
        let nats = async_nats::connect(broker_url)
            .await
            .context(format!(r#"connecting to broker, "{}""#, broker_url))?;

        Ok(Self { nats, config })
    }
}

impl RouterSetup {
    pub async fn new() -> Result<Self> {
        let common = SharedSetup::new().await?;
        let db = Mutex::new(DB::connect(&common.config.engines.api.db_url)?);
        Ok(Self { common, db })
    }
}
