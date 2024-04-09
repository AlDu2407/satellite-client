use std::collections::HashMap;

use serde::Deserialize;
use serde::ser::SerializeStruct;

use super::{rest::SatelliteError, status::ResponseStatus};

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
#[ts(export_to = "response.ts")]
pub struct SatelliteResponse {
    pub code: u16,
    pub status: ResponseStatus,
    pub headers: HashMap<String, String>,
    pub body: serde_json::Value,
}

impl SatelliteResponse {
    pub async fn from_response(value: reqwest::Response) -> Result<Self, SatelliteError> {
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

        Ok(Self {
            code: value.status().as_u16(),
            status: ResponseStatus::from(value.status().as_u16()),
            body: value.json().await?,
            headers,
        })
    }
}

impl serde::ser::Serialize for SatelliteResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
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
