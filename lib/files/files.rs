use crate::result::{Context, Result};
use std::fs::{File, OpenOptions};

pub fn open_or_create(path: &str, append: bool) -> Result<File> {
    OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .append(append)
        .open(path)
        .context(format!(
            r#"opening file with the possibility of creating it if it doesn't exist, "{}""#,
            path
        ))
}
