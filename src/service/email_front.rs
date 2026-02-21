use actix_web::{post, web, HttpMessage, HttpRequest};
use tracing::{info, instrument};

use crate::{
    configuration::Settings,
    email::{Content, EmailContent, EmailReturnInfo},
    service::{
        auth::extract_api_key, errors::EmailError, smtp_provider::SmtpProvider, EmailProvider,
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
    let request_id = request
        .extensions()
        .get::<String>()
        .cloned()
        .unwrap_or_else(|| "unknown".into());

    let api_key = extract_api_key(&request, &settings)?;

    let email: EmailContent = (content.into_inner(), api_key).into();

    let provider = SmtpProvider::new(settings.get_ref());

    let result = provider.send(email, &request_id).await?;

    Ok(web::Json(result))
}
    