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

        let parts: Vec<&str> = id.split('@').collect();
        assert_eq!(parts.len(), 2);
        let left = parts[0];
        let right = parts[1];

        assert_eq!(right, domain);

        let left_parts: Vec<&str> = left.split('.').collect();
        assert_eq!(left_parts[0], request_id);

        // timestamp 应该是数字
        assert!(left_parts[1].parse::<i64>().is_ok());
    }
    #[test]
    fn returns_current_timestamp() {
        let (id, time) = generate_message_id("req", "domain");

        assert!(!id.is_empty());

        let now = chrono::Local::now();

        // 时间差应该非常小
        assert!((now.timestamp() - time.timestamp()).abs() < 2);
    }
}
