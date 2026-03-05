use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Serialize, Clone)]
#[allow(non_snake_case)]
pub struct EmailReturnInfo {
    pub To: String,
    pub SubmittedAt: DateTime<Local>,
    pub MessageID: String,
    pub ErrorCode: u16,
    pub Message: String,
}
