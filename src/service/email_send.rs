use mail_send::{mail_builder::{MessageBuilder, headers::message_id::MessageId}, SmtpClientBuilder};
use secrecy::ExposeSecret;

use crate::{configuration::get_configuration, email_content::EmailContent};

pub async fn send_email(email_content: EmailContent) {
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


    // MessageId::




    let message_builder = MessageBuilder::new()
        .from(from_data)
        .to(to_data)
        .subject(email_content.Subject.as_str())
        .html_body(email_content.HtmlBody.as_str())
        .text_body(email_content.TextBody.as_str())

        ;



    // Connect to the SMTP submissions port, upgrade to TLS and
    // authenticate using the provided credentials.
    let configuration = get_configuration().unwrap();
    let hostname = configuration.mail_server.hostname.as_str();
    let port = configuration.mail_server.port;
    let credential = (
        email_content.From.as_str(),
        email_content.password.expose_secret().as_str(),
    );

    SmtpClientBuilder::new(hostname, port)
        .allow_invalid_certs()
        .implicit_tls(false)
        .credentials(credential)
        .connect()
        .await
        .unwrap()
        .send(message_builder)
        .await
        .unwrap();


}

