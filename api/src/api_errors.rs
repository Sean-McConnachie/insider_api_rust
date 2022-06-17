use std::fmt;

use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use database_handler::database_errors::DbError;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(status_code: u16, message: String) -> ApiError {
        ApiError { status_code, message }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<DbError> for ApiError {
    fn from(error: DbError) -> ApiError {
        match error {
            DbError::DatabaseError(e) => ApiError::new(409, e),
            DbError::NotFound(_) => ApiError::new(404, "No matching records found".to_string()),
            DbError::ParseError(e) => ApiError::new(400, e),
            _ => ApiError::new(500, "Internal server error.".to_string()),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => {
                error!("{}", self.message);
                "Internal server error".to_string()
            }
        };

        HttpResponse::build(status_code).json(Self{status_code: status_code.as_u16(), message})
    }
}