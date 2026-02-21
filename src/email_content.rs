use chrono::{DateTime, Local};
use secrecy::SecretString;

/// collect data from REST API
#[derive(serde::Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Content {
    pub From: String,
    pub To: String,
    pub Subject: String,
    pub TextBody: String,
    pub HtmlBody: String,
}

/// connect with email server
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

impl From<(Content, String)> for EmailContent {
    fn from((content, api_key): (Content, String)) -> Self {
        let password = SecretString::new(api_key.into());
        Self {
            From: content.From,
            password,
            To: content.To,
            Subject: content.Subject,
            TextBody: content.TextBody,
            HtmlBody: content.HtmlBody,
        }
    }
}

/// Return infos
#[derive(serde::Serialize, Clone)]
#[allow(non_snake_case)]
pub struct EmailReturnInfo {
    pub To: String,
    pub SubmittedAt: DateTime<Local>,
    pub MessageID: String,
    pub ErrorCode: u16,
    pub Message: String,
}
