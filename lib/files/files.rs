// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use crate::result::Result;
use std::fs::{File, OpenOptions};

pub fn open_or_create(path: &str, append: bool) -> Result<File> {
    Ok(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .append(append)
        .open(path)?)
}
