use secrecy::SecretString;

#[derive(Clone)]
#[allow(non_snake_case)]
pub struct EmailContent {
    pub From: String,
    pub password: SecretString,
    pub To: String,
    pub Subject: String,
    pub TextBody: String,
    pub HtmlBody: String,
}



use super::api_models::Content;

impl From<(Content, String)> for EmailContent {
    fn from((content, api_key): (Content, String)) -> Self {
        Self {
            From: content.From,
            password: SecretString::new(api_key.into()),
            To: content.To,
            Subject: content.Subject,
            TextBody: content.TextBody,
            HtmlBody: content.HtmlBody,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::email::api_models::Content;

    #[test]
    fn convert_content_into_email_content() {
        let content = Content {
            From: "sender@test.com".into(),
            To: "receiver@test.com".into(),
            Subject: "Hello".into(),
            TextBody: "text".into(),
            HtmlBody: "<b>html</b>".into(),
        };

        let api_key = "secret_key".to_string();

        let email: EmailContent = (content.clone(), api_key.clone()).into();

        assert_eq!(email.From, content.From);
        assert_eq!(email.To, content.To);
        assert_eq!(email.Subject, content.Subject);
        assert_eq!(email.TextBody, content.TextBody);
        assert_eq!(email.HtmlBody, content.HtmlBody);
    }
}