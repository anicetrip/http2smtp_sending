use actix_web::http::StatusCode;
use actix_web::ResponseError;
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
    fn status_code(&self) -> StatusCode {
        match self {
            Self::MissingHeader => StatusCode::UNAUTHORIZED,
            Self::InvalidHeader => StatusCode::BAD_REQUEST,
            Self::SendFailed => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::ResponseError;

    #[test]
    fn missing_header_returns_401() {
        let err = EmailError::MissingHeader;

        let resp = err.error_response();

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn invalid_header_returns_400() {
        let err = EmailError::InvalidHeader;

        let resp = err.error_response();

        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn send_failed_returns_500() {
        let err = EmailError::SendFailed;

        let resp = err.error_response();

        assert_eq!(
            resp.status(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
