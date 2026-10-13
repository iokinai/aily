use std::env;

use thiserror::Error;
use trl::Trl;

#[derive(Error, Debug)]
pub enum EnvError {
    #[error("Could not initialize dotenv")]
    DotEnvError,
    #[error("Could not load env variable: {0}")]
    VariableNotFound(String),
    #[error("Varriable has incorrent type: {0}")]
    IncorrectType(String),
}

#[derive(Trl)]
#[getters]
pub struct Environment {
    database_connection: String,
    danbooru_id: u64,
    backend_url: String,
    frontend_url: String,
}

impl Environment {
    pub fn parse() -> Result<Environment, EnvError> {
        dotenv::dotenv().map_err(|_| EnvError::DotEnvError)?;

        Ok(Environment {
            database_connection: env::var("AILY_DATABASE")
                .map_err(|_| EnvError::VariableNotFound("AILY_DATABASE".to_string()))?,
            danbooru_id: env::var("AILY_DANBOORU_ID")
                .map_err(|_| EnvError::VariableNotFound("AILY_DANBOORU_ID".to_string()))?
                .parse::<u64>()
                .map_err(|_| EnvError::IncorrectType("AILY_DANBOORU_ID".to_string()))?,
            backend_url: env::var("AILY_BACKEND_URL")
                .map_err(|_| EnvError::VariableNotFound("AILY_BACKEND_URL".to_string()))?,
            frontend_url: env::var("AILY_FRONTEND_URL")
                .map_err(|_| EnvError::VariableNotFound("AILY_FRONTEND_URL".to_string()))?,
        })
    }
}
