use chrono::{DateTime, Local};
use secrecy::Secret;
///collect data from restful api
#[derive(serde::Deserialize,Clone)]
#[allow(non_snake_case)]
pub struct Content{
    pub From: String,
    pub To:String,
    pub Subject:String,
    pub TextBody:String,
    pub HtmlBody: String,
}

/// connect with email server
#[derive(serde::Deserialize,Clone)]
#[allow(non_snake_case)]
pub struct EmailContent{
    pub From: String,
    pub password: Secret<String>,
    pub To:String,
    pub Subject:String,
    pub TextBody:String,
    pub HtmlBody: String,
}

/// Return infos
#[derive(serde::Deserialize,serde::Serialize,Clone)]
#[allow(non_snake_case)]
pub struct EmailReturnInfo{
    pub To:String,
    pub SubmittedAt:DateTime<Local>,
    pub MessageID:String,
    pub ErrorCode:u16,
    pub Message: String,
}

