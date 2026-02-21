use mail_send::mail_builder::MessageBuilder;
use crate::email::domain::EmailContent;

impl EmailContent {
    pub fn to_message<'a>(&'a self, message_id: &'a str) -> MessageBuilder<'a> {
        let from_data = (
            self.From.split('@').next().unwrap_or(""),
            self.From.as_str(),
        );

        let to_data: Vec<(&str, &str)> = self
            .To
            .split(',')
            .map(|addr| addr.trim())
            .map(|addr| (addr.split('@').next().unwrap_or(""), addr))
            .collect();

        MessageBuilder::new()
            .from(from_data)
            .to(to_data)
            .subject(self.Subject.as_str())
            .html_body(self.HtmlBody.as_str())
            .text_body(self.TextBody.as_str())
            .message_id(message_id)
    }
}


#[cfg(test)]
mod tests {
    use crate::email::domain::EmailContent;
    use secrecy::SecretString;

    #[test]
    fn build_message_builder_successfully() {
        let email = EmailContent {
            From: "sender@test.com".into(),
            password: SecretString::new("pwd".into()),
            To: "receiver@test.com".into(),
            Subject: "Hello".into(),
            TextBody: "plain".into(),
            HtmlBody: "<b>html</b>".into(),
        };

        let message = email.to_message("test-id@test.com");

        // 只验证 builder 创建成功（Happy Path）
        let _ = message;

        // 如果能执行到这里，说明 builder 构造成功
        assert!(true);
    }
}