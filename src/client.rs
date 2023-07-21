/// Dependencies
use reqwest::{ClientBuilder, header::{HeaderMap, HeaderValue}};
use crate::api_models::StatusResponse;

/// Used to create requests to Luarmor.
pub struct Client {
    api_key: String,
    client: reqwest::Client
}
impl Client {
    /// Creates an instance.
    pub fn new(api_key: &str) -> Self {
        // Set default headers
        let mut default_headers = HeaderMap::new();
        default_headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        default_headers.insert("Authorization", HeaderValue::from_str(api_key).unwrap());

        // Return self
        Self {
            api_key: api_key.to_string(),
            client: ClientBuilder::new()
                .default_headers(default_headers)
                .build()
                .unwrap()
        }
    }

    /// This will return you the version information about the API.
    pub async fn status(&self) -> Result<StatusResponse, reqwest::Error> {
        Ok(self.client.get("status").send().await?.json().await?)
    }

    /// This will return you the version information about the API..
    pub async fn key_details(&self) {

    }
}