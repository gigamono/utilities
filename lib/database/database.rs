// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use diesel::connection::Connection;

use crate::result::{Context, Result};

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
        let conn =
            T::establish(conn_str).context(format!(r#"connecting to db, "{}""#, conn_str))?;

        Ok(Self { conn })
    }
}
