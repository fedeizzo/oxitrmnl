use std::{env, fmt::Display};

const DATABASE_URL: &str = "DATABASE_URL";

const LOG_LEVEL: &str = "LOG_LEVEL";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub database_url: String,
    pub log_level_var: String,
}

impl Config {
    pub fn from_env() -> Result<Config, ConfigError> {
        let database_url = env::var(DATABASE_URL)?;
        Ok(Config {
            database_url,
            log_level_var: LOG_LEVEL.to_string(),
        })
    }

    pub fn get_db_type(&self) -> DBType {
        // TODO add logic to recognize db based on url
        return DBType::SQLite;
    }
}

pub enum DBType {
    SQLite,
}

#[derive(Debug)]
pub struct ConfigError(String);

impl From<env::VarError> for ConfigError {
    fn from(value: env::VarError) -> Self {
        match value {
            env::VarError::NotPresent => {
                ConfigError(format!("env variable not present: {}", value))
            }
            env::VarError::NotUnicode(_) => ConfigError("not unicode char".to_string()),
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
