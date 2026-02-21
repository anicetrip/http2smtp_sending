use chrono::{DateTime, Local};
use sysinfo::System;
use uuid::Uuid;

pub fn generate_message_id() -> (String, DateTime<Local>) {
    let sysname = System::host_name().unwrap_or_default();
    let id = Uuid::new_v4();
    let now = Local::now();

    let message_id = format!("{}.{}@{}", id, now.timestamp(), sysname);

    (message_id, now)
}
