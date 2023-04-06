use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::service::email_front::email_api;

pub async fn http_front() -> std::io::Result<()> {
    HttpServer::new(|| App::new().wrap(TracingLogger::default()).service(email_api))
        .bind(("0.0.0.1", 8080))?
        .run()
        .await
}
