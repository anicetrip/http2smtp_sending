use crate::email_content::EmailContent;
use mail_send::SmtpClientBuilder;
use secrecy::ExposeSecret;

pub async fn send_via_smtp(
    hostname: &str,
    port: u16,
    email: &EmailContent,
    message: mail_send::mail_builder::MessageBuilder<'_>,
) -> Result<(), String> {
    let credential = (email.From.as_str(), email.password.expose_secret());

    let mut client = SmtpClientBuilder::new(hostname, port)
        .allow_invalid_certs()
        .implicit_tls(false)
        .credentials(credential)
        .connect()
        .await
        .map_err(|e| e.to_string())?;

    client.send(message).await.map_err(|e| e.to_string())?;

    Ok(())
}
