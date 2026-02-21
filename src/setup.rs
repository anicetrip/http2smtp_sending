use crate::configuration::Settings;
use crate::service::email_front::email_api;
use crate::telemetry::span::CustomRootSpanBuilder;
use actix_web::{web::Data, App, HttpServer};
use tracing_actix_web::TracingLogger;

pub async fn http_front(settings: Settings) -> std::io::Result<()> {
    let settings = Data::new(settings);

    HttpServer::new(move || {
        App::new()
            .app_data(settings.clone())
            .wrap(TracingLogger::<CustomRootSpanBuilder>::new())
            .service(email_api)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
