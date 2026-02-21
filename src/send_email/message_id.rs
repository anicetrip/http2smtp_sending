use chrono::{DateTime, Local};

pub fn generate_message_id(
    request_id: &str,
    domain: &str,
) -> (String, DateTime<Local>) {
    let now = Local::now();

    let message_id = format!("{}.{}@{}", request_id, now.timestamp(), domain);

    (message_id, now)
}