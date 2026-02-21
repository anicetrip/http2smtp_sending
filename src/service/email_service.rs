use crate::{
    configuration::Settings,
    email_content::{EmailContent, EmailReturnInfo},
    send_email::send_email,
    service::errors::EmailError,
};
use tracing::{error, info};

pub async fn send(email: EmailContent, settings: &Settings) -> Result<EmailReturnInfo, EmailError> {
    info!("sending email via SMTP");

    let result = send_email(email, settings).await;

    if result.ErrorCode != 0 {
        error!(
            error_code = result.ErrorCode,
            message = %result.Message,
            "email send failed"
        );
        return Err(EmailError::SendFailed);
    }

    info!(
        message_id = %result.MessageID,
        "email sent successfully"
    );

    Ok(result)
}
