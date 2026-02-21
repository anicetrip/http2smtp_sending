use lettre_try::{configuration, setup::http_front, telemetry::init::init_tracing};
use tracing::{error, info};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_tracing();

    info!("start to work");

    // ⭐ 加载配置
    let config = match configuration::get_configuration() {
        Ok(c) => {
            info!(?c, "configuration loaded");
            c
        }
        Err(e) => {
            error!(error = ?e, "failed to load configuration");
            return Err(std::io::Error::other(e));
        }
    };

    // ⭐ 把配置传入 http server
    http_front(config).await
}
