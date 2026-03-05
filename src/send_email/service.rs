use crate::{
    configuration::Settings,
    email::{EmailContent, EmailReturnInfo},
};
use std::time::Instant;
use tracing::{debug, error, info, instrument};

use super::{message_id::generate_message_id, smtp::send_via_smtp};

#[instrument(
    name = "smtp_send_email",
    skip(email, settings),
    fields(
        to = %email.To,
        subject = %email.Subject
    )
)]
pub async fn send_email<'a>(
    email: EmailContent,
    settings: &'a Settings,
    request_id: &'a str,
) -> EmailReturnInfo {
    let start_time = Instant::now();

    let domain = &settings.mail_server.hostname;

    let (message_id, timestamp) = generate_message_id(request_id, domain);

    info!(
        message_id = %message_id,
        "generated message id"
    );

    let mut result = EmailReturnInfo {
        To: email.To.clone(),
        SubmittedAt: timestamp,
        MessageID: message_id.clone(),
        ErrorCode: 0,
        Message: String::new(),
    };

    debug!("building MIME message");

    let message = email.to_message(&message_id);

    info!(
        smtp_host = %settings.mail_server.hostname,
        smtp_port = settings.mail_server.port,
        "connecting to SMTP server"
    );

    match send_via_smtp(
        &settings.mail_server.hostname,
        settings.mail_server.port,
        &email,
        message,
    )
    .await
    {
        Ok(_) => {
            let duration_ms = start_time.elapsed().as_millis();

            info!(
                message_id = %message_id,
                duration_ms,
                "email sent successfully"
            );

            result.Message = "Successful sending".into();
        }
        Err(err) => {
            let duration_ms = start_time.elapsed().as_millis();
            error!(
                message_id = %message_id,
                duration_ms,
                error = %err,
                "smtp send failed"
            );

            result.ErrorCode = 1;
            result.Message = err;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::{MailServer, Settings};
    use secrecy::SecretString;

    fn test_settings() -> Settings {
        Settings {
            pass_header: "X-API-Key".into(),
            mail_server: MailServer {
                hostname: "localhost".into(),
                port: 25,
            },
            port: 3300,
        }
    }

    fn test_email() -> EmailContent {
        EmailContent {
            From: "sender@test.com".into(),
            password: SecretString::new("pwd".into()),
            To: "receiver@test.com".into(),
            Subject: "Hello".into(),
            TextBody: "text".into(),
            HtmlBody: "<b>html</b>".into(),
        }
    }

    #[tokio::test]
    async fn send_email_returns_result_structure() {
        let settings = test_settings();
        let email = test_email();

        // ⚠️ 使用不存在端口避免真实连接
        let result = send_email(email, &settings, "req-123").await;

        assert_eq!(result.To, "receiver@test.com");
        assert!(result.MessageID.contains("req-123"));
        assert!(!result.SubmittedAt.to_string().is_empty());
    }

    #[tokio::test]
    async fn send_email_sets_error_on_failure() {
        let settings = test_settings();
        let email = test_email();

        let result = send_email(email, &settings, "req-123").await;

        assert_eq!(result.ErrorCode, 1);
        assert!(!result.Message.is_empty());
    }
}
