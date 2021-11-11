use strum_macros::Display;

#[derive(Debug, Display, Clone, Copy)]
pub enum StatusCode {
    InternalServerError = 500,
    BadRequest = 400,
    NotFound = 404,
    Unauthorized = 401,
}

impl StatusCode {
    pub fn to_u16(&self) -> u16 {
        *self as u16
    }
}
