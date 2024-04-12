use anyhow::Error;
use std::{fmt, str::FromStr};
use url::Url;

use dotenvy;

use serde::{Deserialize, Serialize};

const ENV_APP_ENV: &str = "APP_ENV";
const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_DATABASE_POOL_SIZE: &str = "DATABASE_POOL_SIZE";

const POSTGRES_SCHEME: &str = "postgres";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub env: Env,
    pub database: Database,
}

const APP_ENV_DEV: &str = "dev";
const APP_ENV_STAGING: &str = "staging";
const APP_ENV_PRODUCTION: &str = "production";

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Env {
    Dev,
    Staging,
    Production,
}

impl FromStr for Env {
    type Err = Error;

    fn from_str(s: &str) -> Result<Env, Error> {
        match s {
            APP_ENV_DEV => Ok(Env::Dev),
            APP_ENV_STAGING => Ok(Env::Staging),
            APP_ENV_PRODUCTION => Ok(Env::Production),
            _ => Err(Error::msg(format!(
                "config: {} is not a valid env. Valid values are [{}, {}, {}]",
                s,
                Env::Dev,
                Env::Staging,
                Env::Production,
            ))),
        }
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Env::Dev => write!(f, "{}", APP_ENV_DEV),
            Env::Staging => write!(f, "{}", APP_ENV_STAGING),
            Env::Production => write!(f, "{}", APP_ENV_PRODUCTION),
        }
    }
}

/// Database contains the data necessary to connect to a database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool_size: u32,
}

const DEFAULT_DATABASE_POOL_SIZE: u32 = 100;

impl Config {
    /// Load and validate the configuration from the environment.
    /// If an error is found while parsing the values, or validating the data, an error is returned.
    pub fn load() -> Result<Self, Error> {
        dotenvy::dotenv().ok();

        // app
        let env = std::env::var(ENV_APP_ENV)
            .map_err(|_| env_not_found(ENV_APP_ENV))?
            .parse::<Env>()?;

        // database
        let database_url =
            std::env::var(ENV_DATABASE_URL).map_err(|_| env_not_found(ENV_DATABASE_URL))?;
        let database_pool_size = std::env::var(ENV_DATABASE_POOL_SIZE)
            .ok()
            .map_or(Ok(DEFAULT_DATABASE_POOL_SIZE), |pool_size_str| {
                pool_size_str.parse::<u32>()
            })?;

        let database = Database {
            url: database_url,
            pool_size: database_pool_size,
        };

        let mut config = Self { env, database };

        config.clean_and_validate()?;

        Ok(config)
    }

    fn clean_and_validate(&mut self) -> Result<(), Error> {
        // Database
        let database_url = Url::parse(&self.database.url)?;
        if database_url.scheme() != POSTGRES_SCHEME {
            return Err(Error::msg(String::from(
                "config: database_url is not a valid postgres URL",
            )));
        }
        Ok(())
    }
}

fn env_not_found(var: &str) -> Error {
    Error::msg(format!("config: {} env var not found", var))
}
