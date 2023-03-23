use chrono::DateTime;
use chrono::{self, Local};
use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};
use secrecy::ExposeSecret;
use sysinfo::{System, SystemExt};
use uuid::Uuid;

use crate::email_content::EmailReturnInfo;
use crate::{configuration::get_configuration, email_content::EmailContent};

pub async fn send_email(email_content: EmailContent) -> EmailReturnInfo {
    // Build a simple multipart message
    let from_data = (
        email_content.From.split("@").collect::<Vec<&str>>()[0],
        email_content.From.as_str(),
    );
    let to_data: Vec<(&str, &str)> = email_content
        .To
        .split(",")
        .map(|email| (email.split("@").collect::<Vec<&str>>()[0], email))
        .collect();

    let mut return_info = get_messageid();

    let message_builder = MessageBuilder::new()
        .from(from_data)
        .to(to_data)
        .subject(email_content.Subject.as_str())
        .html_body(email_content.HtmlBody.as_str())
        .text_body(email_content.TextBody.as_str())
        .message_id(return_info.MessageID.clone());

    // Connect to the SMTP submissions port, upgrade to TLS and
    // authenticate using the provided credentials.
    let configuration = get_configuration().unwrap();
    let hostname = configuration.mail_server.hostname.as_str();
    let port = configuration.mail_server.port;
    let credential = (
        email_content.From.as_str(),
        email_content.password.expose_secret().as_str(),
    );

    let _return_message = match SmtpClientBuilder::new(hostname, port)
        .allow_invalid_certs()
        .implicit_tls(false)
        .credentials(credential)
        .connect()
        .await
    {
        Ok(mut a) => {
            match a.send(message_builder).await {
                Ok(_) => {
                    return_info.Message = "Successful sending".to_string();
                    return return_info;
                }
                Err(e) => {
                    return_info.Message = e.to_string();
                    return_info.ErrorCode = 1;
                    return return_info;
                }
            };
        }
        Err(e) => {
            return_info.Message = e.to_string();
            return_info.ErrorCode = 2;
            return return_info;
        }
    };
}

fn get_messageid() -> EmailReturnInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    let sysname = sys.host_name().unwrap_or_default();
    let id = Uuid::new_v4();
    let local: DateTime<Local> = Local::now();
    let timetstamp = local.timestamp().to_string();

    EmailReturnInfo {
        To: "".to_string(),
        SubmittedAt: local,
        MessageID: id.to_string() + "." + &timetstamp + "@" + &sysname,
        ErrorCode: 0,
        Message: "".to_string(),
    }
}
