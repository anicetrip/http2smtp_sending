use serde_aux::prelude::deserialize_number_from_string;
use tracing::{error, info};

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub mail_server: MailServer,
    pub pass_header: String,
    pub port: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct MailServer {
    pub hostname: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}
impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \
Use either `local` or `production`.",
                other
            )),
        }
    }
}

pub fn load_configuration() -> std::io::Result<Settings> {
    match get_configuration() {
        Ok(c) => {
            info!(?c, "configuration loaded");
            Ok(c)
        }
        Err(e) => {
            error!(error = ?e, "failed to load configuration");
            Err(std::io::Error::other(e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_parses_local() {
        let env: Environment = "local".to_string().try_into().unwrap();
        assert_eq!(env.as_str(), "local");
    }

    #[test]
    fn environment_parses_production() {
        let env: Environment = "production".to_string().try_into().unwrap();
        assert_eq!(env.as_str(), "production");
    }

    #[test]
    fn environment_rejects_invalid_value() {
        let result: Result<Environment, _> = "invalid".to_string().try_into();
        assert!(result.is_err());
    }

    #[test]
    fn load_configuration_returns_settings() {
        let result = load_configuration();

        assert!(result.is_ok());

        let settings = result.unwrap();

        assert!(!settings.mail_server.hostname.is_empty());
        assert!(settings.mail_server.port > 0);
        assert!(settings.port > 0);
    }

    #[test]
    fn get_configuration_reads_yaml() {
        std::env::set_var("APP_ENVIRONMENT", "local");

        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        std::env::set_current_dir(root).unwrap();

        let config = get_configuration();

        println!("config result: {:?}", config);

        assert!(config.is_ok());
    }
}
