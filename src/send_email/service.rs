use crate::{
    configuration::Settings,
    email_content::{EmailContent, EmailReturnInfo},
};
use tracing::{error, info};

use super::{message::build_message, message_id::generate_message_id, smtp::send_via_smtp};

pub async fn send_email(email: EmailContent, settings: &Settings) -> EmailReturnInfo {
    let (message_id, timestamp) = generate_message_id();

    let mut result = EmailReturnInfo {
        To: email.To.clone(),
        SubmittedAt: timestamp,
        MessageID: message_id.clone(),
        ErrorCode: 0,
        Message: String::new(),
    };

    let message = build_message(&email, message_id);

    info!("connecting to smtp server");

    match send_via_smtp(
        &settings.mail_server.hostname,
        settings.mail_server.port,
        &email,
        message,
    )
    .await
    {
        Ok(_) => {
            info!("email sent successfully");
            result.Message = "Successful sending".into();
        }
        Err(err) => {
            error!(error = %err, "smtp send failed");
            result.ErrorCode = 1;
            result.Message = err;
        }
    }

    result
}
