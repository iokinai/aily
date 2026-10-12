use serde::Serialize;

use crate::errors::ErrorType;

#[derive(Serialize)]
pub struct HttpErrorBody {
    #[serde(rename = "error_type")]
    e_type: ErrorType,
    #[serde(rename = "inner")]
    inner_json_string: serde_json::Value,
}

impl HttpErrorBody {
    pub fn new(e_type: ErrorType, inner_json_string: serde_json::Value) -> HttpErrorBody {
        HttpErrorBody {
            e_type,
            inner_json_string,
        }
    }
}
