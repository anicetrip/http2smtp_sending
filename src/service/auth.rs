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
