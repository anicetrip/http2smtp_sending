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
