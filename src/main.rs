use lettre_try::{setup::http_front, configuration};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("{}","start to work");
    println!("configuration: {:?}",configuration::get_configuration());
    http_front().await
}
