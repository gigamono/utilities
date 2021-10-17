use diesel::connection::Connection;

use crate::messages::error::Result;
use crate::messages::error::SystemError;

pub struct DB<T>
where
    T: Connection,
{
    conn: T,
}

impl<T> DB<T>
where
    T: Connection,
{
    pub fn connect(conn_str: &str) -> Result<Self> {
        let conn = T::establish(conn_str).map_err(|err| SystemError::Conn {
            ctx: format!("connecting to db, `{}`", conn_str),
            src: err,
        })?;
        Ok(Self { conn })
    }
}
