use std::collections::HashMap;

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

use super::status::ResponseStatus;

#[derive(Serialize, ts_rs::TS)]
#[ts(export)]
#[ts(export_to = "error.ts")]
#[serde(tag = "type", content = "error")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SatelliteError {
    ClientError(String),
}

impl From<reqwest::Error> for SatelliteError {
    fn from(err: reqwest::Error) -> Self {
        SatelliteError::ClientError(err.to_string())
    }
}

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
#[ts(export_to = "response.ts")]
pub struct SatelliteResponse {
    pub code: u16,
    pub status: ResponseStatus,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
}

impl SatelliteResponse {
    pub async fn from_response(
        value: reqwest::Response,
    ) -> std::result::Result<Self, SatelliteError> {
        let headers = value
            .headers()
            .iter()
            .map(|(&ref name, &ref value)| {
                (
                    name.to_string(),
                    value.to_str().unwrap_or_default().to_string(),
                )
            })
            .collect();

        let code = value.status().as_u16();
        let body = match value.json().await {
            Ok(body) => Some(body),
            Err(_) => None,
        };

        Ok(Self {
            code,
            status: ResponseStatus::from(code),
            body,
            headers,
        })
    }
}

impl serde::ser::Serialize for SatelliteResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut response = serializer.serialize_struct("SatelliteResponse", 4)?;
        response.serialize_field("code", &self.code)?;
        response.serialize_field("status", &self.status)?;
        response.serialize_field("headers", &self.headers)?;
        response.serialize_field("body", &self.body)?;
        response.end()
    }
}

pub type Result<SatelliteResponse> = std::result::Result<SatelliteResponse, SatelliteError>;
