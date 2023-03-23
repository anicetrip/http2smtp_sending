use serde_aux::prelude::deserialize_number_from_string;
#[derive(serde::Deserialize)]
pub struct Settings {
    pub mail_server: MailServer,
    pub pass_header: String,
}

#[derive(serde::Deserialize)]
pub struct MailServer {
    pub hostname: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}
