use actix_web::error;
use actix_web::http::StatusCode;
use serde::Serialize;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct OSBError {
    error: ErrorCode,
    description: String,
    instance_usable: bool,
    update_repeatable: bool,
}

#[allow(dead_code)]
#[derive(Serialize, Debug)]
pub enum ErrorCode {
    AsyncRequired,
    ConcurrencyError,
    RequiresApp,
    MaintenanceInfoConflict,
    Custom(String),
}

impl OSBError {
    fn new(error: ErrorCode, desc: String) -> Self {
        OSBError {
            error,
            description: desc,
            instance_usable: true,
            update_repeatable: true,
        }
    }
}

impl fmt::Display for OSBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::ResponseError for OSBError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
