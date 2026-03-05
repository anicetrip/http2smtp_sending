use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Content {
    pub From: String,
    pub To: String,
    pub Subject: String,
    pub TextBody: String,
    pub HtmlBody: String,
}
