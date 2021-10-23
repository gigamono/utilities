use diesel::connection::Connection;

use crate::messages::error::SystemError;
use crate::result::Result;

pub struct DB<T>
where
    T: Connection,
{
    pub conn: T,
}

impl<T> DB<T>
where
    T: Connection,
{
    pub fn connect(conn_str: &str) -> Result<Self> {
        let conn = T::establish(conn_str).map_err(|err| SystemError::Conn {
            ctx: format!(r#"connecting to db, "{}""#, conn_str),
            src: err,
        })?;
        Ok(Self { conn })
    }
}
