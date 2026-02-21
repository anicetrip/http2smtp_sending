use crate::{
    email_content::{EmailContent, EmailReturnInfo},
    service::errors::EmailError,
};
#[allow(async_fn_in_trait)]
pub trait EmailProvider: Send + Sync {
    async fn send(
        &self,
        email: EmailContent,
        request_id: &str,
    ) -> Result<EmailReturnInfo, EmailError>;
}
