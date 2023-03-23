use lettre_try::setup::http_front;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    http_front().await
}
