use actix_web::guard::{self, AnyGuard};

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
