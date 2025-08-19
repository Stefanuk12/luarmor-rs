use api_builder::{Endpoint, api_endpoint};
use serde::Deserialize;

/// This will return you the version information about the API.
///
/// [Reference](https://docs.luarmor.net/#getting-api-status)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ApiStatus;
#[api_endpoint(method = GET, path = "\"/status\"")]
impl Endpoint for ApiStatus {}

/// [Reference](https://docs.luarmor.net/#tab-id-200-ok)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
pub struct ApiStatusResponse {
    pub version: String,
    pub active: bool,
    pub message: String,
    pub warning: bool,
    pub warning_message: String,
}
