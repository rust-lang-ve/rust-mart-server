use actix_web::error::Error as ActixError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Deserialize;
use serde_json::json;
use std::fmt;
use http_auth_basic::AuthBasicError;

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(status_code: u16, message: String) -> Self {
        Self {
            status_code,
            message,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
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
                "Internal Server Error".to_string()
            }
        };

        HttpResponse::build(status_code).json(json!({ "message": message }))
    }
}

impl From<ActixError> for ApiError {
    fn from(error: ActixError) -> Self {
        let message = error.to_string();

        error!("An actix error ocurred. {}", message);
        Self::new(500, message)
    }
}

impl From<AuthBasicError> for ApiError {
    fn from(auth_basic_error: AuthBasicError) -> Self {
        let message = auth_basic_error.to_string();

        error!("An error ocurred with the Authorization process");
        Self::new(400, message)
    }
}
