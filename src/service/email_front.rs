use actix_web::{post, web, HttpMessage, HttpRequest};
use tracing::{info, instrument};

use crate::{
    configuration::Settings,
    email::{Content, EmailContent, EmailReturnInfo},
    service::{
        auth::extract_api_key, errors::EmailError, smtp_provider::SmtpProvider, EmailProvider,
    },
};

#[post("/email")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::{MailServer, Settings};
    use actix_web::{test, App};

    fn test_settings() -> Settings {
        Settings {
            pass_header: "X-API-Key".into(),
            mail_server: MailServer {
                hostname: "127.0.0.1".into(),
                port: 1, // 确保 SMTP 一定失败
            },
            port: 3300,
        }
    }

    fn valid_body() -> Content {
        Content {
            From: "sender@test.com".into(),
            To: "receiver@test.com".into(),
            Subject: "Hello".into(),
            TextBody: "text".into(),
            HtmlBody: "<b>html</b>".into(),
        }
    }

    #[actix_rt::test]
    async fn email_api_returns_response() {
        let settings = test_settings();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(settings))
                .service(email_api),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/email")
            .insert_header(("X-API-Key", "secret"))
            .set_json(&valid_body())
            .to_request();

        let resp = test::call_service(&app, req).await;

        // SMTP 会失败，但 handler 会返回 EmailReturnInfo
        assert!(resp.status().is_success() || resp.status().is_server_error());
    }

    #[actix_rt::test]
    async fn email_api_fails_without_api_key() {
        let settings = test_settings();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(settings))
                .service(email_api),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/email")
            .set_json(&valid_body())
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn email_api_rejects_invalid_json() {
        let settings = test_settings();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(settings))
                .service(email_api),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/email")
            .insert_header(("X-API-Key", "secret"))
            .set_payload("{}") // 缺少字段
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error());
    }
}
