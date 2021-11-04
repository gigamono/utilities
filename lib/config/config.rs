use serde::Deserialize;

use crate::nested_struct;

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub kind: String,
    pub version: String,
}

nested_struct! {
    Permissions {
        fs (FSPermissions {
            open (Vec<String>),
            write (Vec<String>),
        }),
    }
}
