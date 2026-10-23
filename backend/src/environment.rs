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

#[derive(Trl, Debug)]
#[getters]
pub struct Environment {
    database_connection: String,
    danbooru_id: u64,
    backend_url: String,
    frontend_url: String,
}

impl Environment {
    pub fn parse() -> Result<Environment, EnvError> {
        if !cfg!(test) {
            dotenv::dotenv().map_err(|_| EnvError::DotEnvError)?;
        }

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

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    fn clear_env() {
        unsafe {
            env::remove_var("AILY_DATABASE");
            env::remove_var("AILY_DANBOORU_ID");
            env::remove_var("AILY_BACKEND_URL");
            env::remove_var("AILY_FRONTEND_URL");
        }
    }

    #[test]
    fn test_parse_success() {
        let _guard = ENV_MUTEX.lock().unwrap();
        clear_env();
        unsafe {
            env::set_var("AILY_DATABASE", "sqlite::memory:");
            env::set_var("AILY_DANBOORU_ID", "42");
            env::set_var("AILY_BACKEND_URL", "http://localhost:8080");
            env::set_var("AILY_FRONTEND_URL", "http://localhost:3000");
        }

        let result = Environment::parse();
        assert!(result.is_ok());

        let env_config = result.unwrap();
        assert_eq!(env_config.database_connection(), "sqlite::memory:");
        assert_eq!(*env_config.danbooru_id(), 42);
        assert_eq!(env_config.backend_url(), "http://localhost:8080");
        assert_eq!(env_config.frontend_url(), "http://localhost:3000");
    }

    #[test]
    fn test_parse_variable_not_found() {
        let _guard = ENV_MUTEX.lock().unwrap();
        clear_env();
        unsafe {
            env::set_var("AILY_DANBOORU_ID", "42");
            env::set_var("AILY_BACKEND_URL", "http://localhost:8080");
            env::set_var("AILY_FRONTEND_URL", "http://localhost:3000");
        }

        let result = Environment::parse();

        assert!(matches!(result, Err(EnvError::VariableNotFound(var)) if var == "AILY_DATABASE"));
    }

    #[test]
    fn test_parse_incorrect_type() {
        let _guard = ENV_MUTEX.lock().unwrap();
        clear_env();
        unsafe {
            env::set_var("AILY_DATABASE", "sqlite::memory:");
            env::set_var("AILY_DANBOORU_ID", "not_a_number");
            env::set_var("AILY_BACKEND_URL", "http://localhost:8080");
            env::set_var("AILY_FRONTEND_URL", "http://localhost:3000");
        }

        let result = Environment::parse();
        assert!(matches!(result, Err(EnvError::IncorrectType(var)) if var == "AILY_DANBOORU_ID"));
    }
}
