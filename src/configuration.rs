use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::convert::{TryFrom, TryInto};

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub application: ApplicationSetting,
    pub database: DatabaseSetting,
}

impl Settings {
    pub fn application_address(&self) -> String {
        format!("{}:{}", self.application.host, self.application.port)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct ApplicationSetting {
    pub host: String,
    pub port: u16,
    pub upload_path: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSetting {
    pub username: String,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub password: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSetting {
    pub fn connection_without_db(&self) -> PgConnectOptions {
        let ssl_mode = match self.require_ssl {
            true => PgSslMode::Require,
            false => PgSslMode::Prefer,
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn connection_with_db(&self) -> PgConnectOptions {
        self.connection_without_db().database(&self.database_name)
    }
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_string(&self) -> String {
        match self {
            Self::Local => "local".to_string(),
            Self::Production => "production".to_string(),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "production" => Ok(Self::Production),
            "local" => Ok(Self::Local),
            _ => Err(format!(
                "{} is an invalid environment, please use 'production' or 'local'",
                s
            )),
        }
    }
}

pub fn get_conf() -> Result<Settings, config::ConfigError> {
    // set up config flow
    let base_path = std::env::current_dir().expect("failed to get cur dur");
    let conf_path = base_path.join("conf");
    let mut settings = config::Config::new();
    // get app deployment environment, defaults to local
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to get app env");
    // set which conf file to load after base based on deployment environment
    let env_conf_path = conf_path.join(environment.as_string());

    // load in the configurations
    // starting with base
    settings.merge(config::File::from(conf_path.join("base")).required(true))?;
    // environment specific conf file
    settings.merge(config::File::from(env_conf_path).required(true))?;
    // deployment environment variables prefixed with APP__
    settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    settings.try_into()
}
