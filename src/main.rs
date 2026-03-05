use lettre_try::configuration::load_configuration;
use lettre_try::{setup::http_front, telemetry::init::init_tracing};
use tracing::info;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_tracing();

    info!("start to work");

    let config = load_configuration()?;

    http_front(config).await
}
