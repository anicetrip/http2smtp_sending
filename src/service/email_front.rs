use actix_web::{post, web, HttpRequest};
use tracing::{info, instrument};

use crate::{
    configuration::Settings,
    email_content::{Content, EmailReturnInfo},
    service::{
        auth::extract_api_key, email_mapper::to_email_content, email_service::send,
        errors::EmailError,
    },
};

#[post("/")]
#[instrument(skip(request, content, settings))]
pub async fn email_api(
    request: HttpRequest,
    content: web::Json<Content>,
    settings: web::Data<Settings>,
) -> Result<web::Json<EmailReturnInfo>, EmailError> {
    info!("request received");

    let api_key = extract_api_key(&request, &settings)?;

    let email = to_email_content(content.into_inner(), api_key);

    let result = send(email, &settings).await?;

    Ok(web::Json(result))
}
