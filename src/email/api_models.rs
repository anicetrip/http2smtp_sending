use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Content {
    pub From: String,
    pub To: String,
    pub Subject: String,
    pub TextBody: String,
    pub HtmlBody: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_valid_content() {
        let json = r#"
        {
            "From": "sender@test.com",
            "To": "receiver@test.com",
            "Subject": "Hello",
            "TextBody": "text",
            "HtmlBody": "<b>html</b>"
        }
        "#;

        let content: Content = serde_json::from_str(json).unwrap();

        assert_eq!(content.From, "sender@test.com");
        assert_eq!(content.To, "receiver@test.com");
        assert_eq!(content.Subject, "Hello");
        assert_eq!(content.TextBody, "text");
        assert_eq!(content.HtmlBody, "<b>html</b>");
    }

    #[test]
    fn deserialize_fails_when_field_missing() {
        let json = r#"
        {
            "From": "sender@test.com",
            "To": "receiver@test.com",
            "Subject": "Hello"
        }
        "#;

        let result: Result<Content, _> = serde_json::from_str(json);

        assert!(result.is_err());
    }
    #[test]
    fn deserialize_fails_when_type_invalid() {
        let json = r#"
        {
            "From": 123,
            "To": "receiver@test.com",
            "Subject": "Hello",
            "TextBody": "text",
            "HtmlBody": "<b>html</b>"
        }
        "#;

        let result: Result<Content, _> = serde_json::from_str(json);

        assert!(result.is_err());
    }
}
