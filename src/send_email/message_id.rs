use chrono::{DateTime, Local};

pub fn generate_message_id(request_id: &str, domain: &str) -> (String, DateTime<Local>) {
    let now = Local::now();

    let message_id = format!("{}.{}@{}", request_id, now.timestamp(), domain);

    (message_id, now)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_id_format_is_correct() {
        let request_id = "abc123";
        let domain = "mail.test.com";

        let (id, _) = generate_message_id(request_id, domain);

        assert!(id.contains(request_id));
        assert!(id.contains(domain));
        assert!(id.contains("@"));
    }
}
