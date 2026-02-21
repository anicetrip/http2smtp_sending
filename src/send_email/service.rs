use crate::{
    configuration::Settings,
    email_content::{EmailContent, EmailReturnInfo},
};
use std::time::Instant;
use tracing::{debug, error, info, instrument};

use super::{message::build_message, message_id::generate_message_id, smtp::send_via_smtp};

#[instrument(
    name = "smtp_send_email",
    skip(email, settings),
    fields(
        to = %email.To,
        subject = %email.Subject
    )
)]
pub async fn send_email(
    email: EmailContent,
    settings: &Settings,
    request_id: &str,
) -> EmailReturnInfo {
    let start_time = Instant::now();

    // ⭐ 使用配置中的域名
    let domain = &settings.mail_server.hostname;

    let (message_id, timestamp) = generate_message_id(&request_id, domain);

    info!(
        message_id = %message_id,
        request_id = %request_id,
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

    let message = build_message(&email, &message_id);

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
            let elapsed = start_time.elapsed();

            info!(
                message_id = %message_id,
                request_id = %request_id,
                duration_ms = elapsed.as_millis(),
                "email sent successfully"
            );

            result.Message = "Successful sending".into();
        }
        Err(err) => {
            let elapsed = start_time.elapsed();

            error!(
                message_id = %message_id,
                request_id = %request_id,
                duration_ms = elapsed.as_millis(),
                error = %err,
                "smtp send failed"
            );

            result.ErrorCode = 1;
            result.Message = err;
        }
    }

    result
}
