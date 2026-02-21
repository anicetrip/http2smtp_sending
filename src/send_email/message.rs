use crate::email_content::EmailContent;
use mail_send::mail_builder::MessageBuilder;

pub fn build_message<'a>(email: &'a EmailContent, message_id: &'a str) -> MessageBuilder<'a> {
    let from_data = (
        email.From.split('@').next().unwrap_or(""),
        email.From.as_str(),
    );

    let to_data: Vec<(&str, &str)> = email
        .To
        .split(',')
        .map(|addr| (addr.split('@').next().unwrap_or(""), addr))
        .collect();

    MessageBuilder::new()
        .from(from_data)
        .to(to_data)
        .subject(email.Subject.as_str())
        .html_body(email.HtmlBody.as_str())
        .text_body(email.TextBody.as_str())
        .message_id(message_id)
}
