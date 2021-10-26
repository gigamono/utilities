use actix_web::{
    guard::{self, AnyGuard},
    web::Bytes,
    HttpRequest as HRequest,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub uri: String,
    pub version: String,
    pub headers: Vec<(String, Vec<u8>)>,
    pub data: Vec<u8>,
}

pub struct HttpResponse {
    pub code: u16,
    pub version: String,
    pub headers: Vec<(String, Vec<u8>)>,
    pub data: Vec<u8>,
}

pub fn any_method_guard() -> AnyGuard {
    guard::Any(guard::Get())
        .or(guard::Post())
        .or(guard::Put())
        .or(guard::Patch())
        .or(guard::Delete())
        .or(guard::Options())
        .or(guard::Head())
        .or(guard::Connect())
        .or(guard::Trace())
}

impl From<(&HRequest, &Bytes)> for HttpRequest {
    fn from(req: (&HRequest, &Bytes)) -> Self {
        // Get headers.
        let mut headers: Vec<(String, Vec<u8>)> = vec![];
        for (k, v) in req.0.headers() {
            headers.push((k.to_string(), v.as_bytes().to_owned()));
        }

        Self {
            headers,
            method: req.0.method().to_string(),
            path: req.0.path().to_string(),
            uri: req.0.uri().to_string(),
            version: format!("{:?}", req.0.version()),
            data: req.1.to_vec(),
        }
    }
}
