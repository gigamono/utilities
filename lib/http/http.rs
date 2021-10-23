use actix_web::guard::{self, AnyGuard};
use hashbrown::HashMap;

pub struct Http1Request {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, Vec<String>>,
    pub data: Vec<u8>,
}

pub struct Http1Response {
    pub code: u16,
    pub version: String,
    pub data: Vec<u8>,
}

pub enum HttpMethod {
    Post,
    Put,
    Patch,
    Delete,
    Options,
    Head,
    Connect,
    Trace,
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
