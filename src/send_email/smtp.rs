use crate::email::EmailContent;
use mail_send::SmtpClientBuilder;
use secrecy::ExposeSecret;
use tracing::{debug, instrument, trace};

#[instrument(
    name = "smtp_transport",
    skip(email, message),
    fields(
        smtp_host = hostname,
        smtp_port = port,
        recipient = %email.To
    )
)]
pub async fn send_via_smtp(
    hostname: &str,
    port: u16,
    email: &EmailContent,
    message: mail_send::mail_builder::MessageBuilder<'_>,
) -> Result<(), String> {
    debug!("establishing SMTP connection (TLS negotiation)");

    let credential = (email.From.as_str(), email.password.expose_secret());

    let mut client = SmtpClientBuilder::new(hostname, port)
        .allow_invalid_certs()
        .implicit_tls(false)
        .credentials(credential)
        .connect()
        .await
        .map_err(|e| {
            debug!(error = %e, "SMTP connection/authentication failed");
            e.to_string()
        })?;

    trace!("SMTP authentication successful");

    debug!("starting SMTP transaction");

    client.send(message).await.map_err(|e| {
        debug!(error = %e, "SMTP transaction failed");
        e.to_string()
    })?;

    debug!("SMTP transaction completed successfully");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::email::EmailContent;
    use mail_send::mail_builder::MessageBuilder;
    use secrecy::SecretString;

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
    async fn smtp_connect_failure_returns_error() {
        let email = test_email();

        // 构造一个最简单 message
        let message = MessageBuilder::new();

        // 使用一个几乎不可能成功的端口
        let result = send_via_smtp("127.0.0.1", 1, &email, message).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn smtp_invalid_host_fails() {
        let email = test_email();
        let message = MessageBuilder::new();

        let result = send_via_smtp("invalid.host.test", 25, &email, message).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn smtp_send_failure_returns_error() {
        let email = test_email();
        let message = MessageBuilder::new();

        let result = send_via_smtp("127.0.0.1", 1, &email, message).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn smtp_send_failure_path() {
        let email = test_email();

        let message = MessageBuilder::new();

        // 故意给一个不可解析的 hostname
        let result = send_via_smtp("invalid.host.local", 25, &email, message).await;

        assert!(result.is_err());
    }
}
