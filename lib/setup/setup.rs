use crate::broker::BrokerClient;
use crate::config::GigamonoConfig;
use crate::database::DB;
use crate::result::Result;

use diesel::sqlite::SqliteConnection;

pub struct SharedSetup {
    pub broker: BrokerClient,
    pub config: GigamonoConfig,
}

pub struct APISetup {
    pub common: SharedSetup,
    pub db: DB<SqliteConnection>,
}

impl SharedSetup {
    pub fn new() -> Result<Self> {
        let config = GigamonoConfig::load()?;
        let broker = BrokerClient::connect(&config.broker.url)?;
        Ok(Self { broker, config })
    }
}

impl APISetup {
    pub fn new() -> Result<Self> {
        let common = SharedSetup::new()?;
        let db = DB::connect(&common.config.engines.api.db_url)?;
        Ok(Self { common, db })
    }
}
