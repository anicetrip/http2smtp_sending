use crate::{configuration::Settings, service::errors::EmailError};
use actix_web::HttpRequest;

pub fn extract_api_key(request: &HttpRequest, settings: &Settings) -> Result<String, EmailError> {
    let header_name = &settings.pass_header;

    let value = request
        .headers()
        .get(header_name)
        .ok_or(EmailError::MissingHeader)?
        .to_str()
        .map_err(|_| EmailError::InvalidHeader)?;

    Ok(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::{MailServer, Settings};
    use actix_web::http::header::HeaderName;
    use actix_web::test;

    fn test_settings() -> Settings {
        Settings {
            pass_header: "X-API-Key".into(),
            mail_server: MailServer {
                hostname: "localhost".into(),
                port: 25,
            },
        }
    }

    #[test]
    async fn extract_api_key_success() {
        let settings = test_settings();

        let req = test::TestRequest::default()
            .insert_header((HeaderName::from_static("x-api-key"), "secret"))
            .to_http_request();

        let key = extract_api_key(&req, &settings).unwrap();

        assert_eq!(key, "secret");
    }

    #[test]
    async fn extract_api_key_missing_header() {
        let settings = test_settings();

        let req = test::TestRequest::default().to_http_request();

        let result = extract_api_key(&req, &settings);

        assert!(matches!(result, Err(EmailError::MissingHeader)));
    }

    #[test]
    async fn extract_api_key_invalid_header() {
        let settings = test_settings();

        let req = test::TestRequest::default()
            .insert_header((HeaderName::from_static("x-api-key"), b"\xff\xff".as_slice()))
            .to_http_request();

        let result = extract_api_key(&req, &settings);

        assert!(matches!(result, Err(EmailError::InvalidHeader)));
    }
}
