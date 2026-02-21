use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use tracing::{field, info_span, Span};
use tracing_actix_web::RootSpanBuilder;
use uuid::Uuid;

pub struct CustomRootSpanBuilder;

impl RootSpanBuilder for CustomRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        let request_id = Uuid::new_v4();

        info_span!(
            "http_request",
            request_id = %request_id,
            method = %request.method(),
            path = %request.path(),
            client_ip = ?request.connection_info().realip_remote_addr(),
            user_agent = ?request.headers().get("user-agent"),

            status_code = field::Empty,
            error = field::Empty,
        )
    }

    fn on_request_end<B: MessageBody>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        match outcome {
            Ok(response) => {
                span.record("status_code", field::display(response.status()));
            }
            Err(e) => {
                span.record("error", field::display(e));
            }
        }
    }
}
