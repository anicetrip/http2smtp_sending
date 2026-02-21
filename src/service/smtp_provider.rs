use tracing::{error, info, instrument};

use crate::{
    configuration::Settings,
    email::{EmailContent, EmailReturnInfo},
    send_email::service::send_email,
    service::{email_provider::EmailProvider, errors::EmailError},
};

pub struct SmtpProvider<'a> {
    settings: &'a Settings,
}

impl<'a> SmtpProvider<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self { settings }
    }
}

impl<'a> EmailProvider for SmtpProvider<'a> {
    #[instrument(
        name = "smtp_send",
        skip(self, email, request_id),
        fields(
            to = %email.To,
            subject = %email.Subject,
            request_id = %request_id
        )
    )]
    async fn send(
        &self,
        email: EmailContent,
        request_id: &str,
    ) -> Result<EmailReturnInfo, EmailError> {
        info!("starting SMTP send");

        // ⭐ 传入 request_id
        let result = send_email(email, self.settings, request_id).await;

        if result.ErrorCode != 0 {
            error!(
                error_code = result.ErrorCode,
                message = %result.Message,
                request_id = %request_id,
                "smtp send failed"
            );
            return Err(EmailError::SendFailed);
        }

        info!(
            message_id = %result.MessageID,
            request_id = %request_id,
            "smtp send success"
        );

        Ok(result)
    }
}
