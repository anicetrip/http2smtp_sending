use actix_web::{post, web, HttpMessage, HttpRequest};
use tracing::{info, instrument};

use crate::{
    configuration::Settings,
    email_content::{Content, EmailContent, EmailReturnInfo},
    service::{EmailProvider, auth::extract_api_key, errors::EmailError, smtp_provider::SmtpProvider},
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

    // ⭐ 创建 provider
    let provider = SmtpProvider::new(settings.get_ref());

    // ⭐ 调用 trait 方法
    let result = provider.send(email, &request_id).await?;

    Ok(web::Json(result))
}
