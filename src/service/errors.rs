use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Missing authentication header")]
    MissingHeader,

    #[error("Invalid header format")]
    InvalidHeader,

    #[error("Email sending failed")]
    SendFailed,
}

impl ResponseError for EmailError {
    fn error_response(&self) -> HttpResponse {
        match self {
            EmailError::MissingHeader => HttpResponse::Unauthorized().body(self.to_string()),
            EmailError::InvalidHeader => HttpResponse::BadRequest().body(self.to_string()),
            EmailError::SendFailed => HttpResponse::InternalServerError().body(self.to_string()),
        }
    }
}
