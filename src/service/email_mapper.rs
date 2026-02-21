use crate::email_content::{Content, EmailContent};
use secrecy::SecretString;

pub fn to_email_content(content: Content, api_key: String) -> EmailContent {
    EmailContent {
        From: content.From,
        password: SecretString::new(api_key.into()),
        Subject: content.Subject,
        To: content.To,
        TextBody: content.TextBody,
        HtmlBody: content.HtmlBody,
    }
}
