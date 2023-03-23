use actix_web::{HttpServer, App};

use crate::service::email_front::email_api;

pub async fn http_front() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(email_api))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}