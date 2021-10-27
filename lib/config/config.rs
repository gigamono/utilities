use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub kind: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct Permissions {
    pub fs: FSPermissions,
}

#[derive(Debug, Deserialize)]
pub struct FSPermissions {
    pub dirs: Vec<String>,
}
