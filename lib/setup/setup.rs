use std::sync::Mutex;

use crate::config::GigamonoConfig;
use crate::database::DB;
use crate::natsio::Nats;
use crate::result::Result;

use diesel::pg::PgConnection;

pub struct SharedSetup {
    pub nats: Nats,
    pub config: GigamonoConfig,
}

pub struct APISetup {
    pub common: SharedSetup,
    pub db: Mutex<DB<PgConnection>>, // SQliteConnection contains Cell/RefCell
}

impl SharedSetup {
    pub fn new() -> Result<Self> {
        let config = GigamonoConfig::load()?;
        let nats = Nats::connect(&config.broker.url)?;
        Ok(Self { nats, config })
    }
}

impl APISetup {
    pub fn new() -> Result<Self> {
        let common = SharedSetup::new()?;
        let db = Mutex::new(DB::connect(&common.config.engines.api.db_url)?);
        Ok(Self { common, db })
    }
}
