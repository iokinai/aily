use std::fmt;

use reqwest::StatusCode;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpStatusCode(pub u16);

impl fmt::Display for HttpStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MaybeStatusCode(pub Option<HttpStatusCode>);

impl MaybeStatusCode {
    pub fn from_option(o: Option<StatusCode>) -> MaybeStatusCode {
        if let Some(s) = o {
            MaybeStatusCode(Some(HttpStatusCode(s.as_u16())))
        } else {
            MaybeStatusCode(None)
        }
    }
}

impl fmt::Display for MaybeStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref s) = self.0 {
            return s.fmt(f);
        }

        write!(f, "<no status>")
    }
}

#[derive(Error, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyError {
    #[error("Could not send request to a remote server, status: {status}")]
    CouldNotSendRequest { status: MaybeStatusCode },
    #[error("Request failed with status: {status}")]
    RequestFailed { status: HttpStatusCode },
    #[error("Unable to read server response")]
    UnableToReadResponse,
}

impl ProxyError {
    fn status_code_from_u16(s: u16) -> axum::http::StatusCode {
        if let Ok(s) = axum::http::StatusCode::from_u16(s) {
            s
        } else {
            axum::http::StatusCode::BAD_GATEWAY
        }
    }

    pub fn get_status_code(&self) -> axum::http::StatusCode {
        match self {
            ProxyError::CouldNotSendRequest { status } => {
                if let Some(ref sc) = status.0 {
                    Self::status_code_from_u16(sc.0)
                } else {
                    axum::http::StatusCode::BAD_GATEWAY
                }
            }
            ProxyError::RequestFailed { status } => Self::status_code_from_u16(status.0),
            ProxyError::UnableToReadResponse => axum::http::StatusCode::BAD_GATEWAY,
        }
    }
}
