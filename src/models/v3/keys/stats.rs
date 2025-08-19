use api_builder::{Endpoint, api_endpoint};
use serde::Deserialize;
use time::OffsetDateTime;
use typed_builder::TypedBuilder;

/// You can fetch the stats of your API key.
///
/// This includes
/// - usage details
/// - remaining obfuscations
/// - max obfuscations
/// - max users
/// - execution amounts
/// - monthly executions
/// - graph values
/// - etc..
///
/// ⚠️ Authenticated ⚠️
///
/// View full response at [ApiKeyStatsResponse].
///
/// [Reference](https://docs.luarmor.net/#getting-api-key-stats)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ApiKeyStats<'a> {
    pub api_key: &'a str,

    /// If `true`, it will return the info about user limits. (e.g. how many users there are, # of banned, # of whitelisted)
    #[builder(default = false)]
    pub no_users: bool,
}
#[api_endpoint(method = GET, path = format!("/v3/keys/{}/stats?noUsers={}", self.api_key, self.no_users))]
impl Endpoint for ApiKeyStats<'_> {}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
pub struct ExecutionData {
    pub frequency: u32,
    pub executions: Vec<u32>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
pub struct ScriptDefaultStats {
    pub scripts: u8,
    pub users: u32,
    pub obfuscations: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
pub struct ScriptStats {
    pub obfuscations: u32,
    pub scripts: u8,
    pub users: u32,
    pub attacks_blocked: u32,
    pub default: ScriptDefaultStats,
    #[serde(with = "time::serde::timestamp")]
    pub reset_at: OffsetDateTime,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
pub struct ScriptStatsNoUsers {
    pub obfuscations: u32,
    pub scripts: u8,
    pub attacks_blocked: u32,
    pub default: ScriptDefaultStats,
    #[serde(with = "time::serde::timestamp")]
    pub reset_at: OffsetDateTime,
}

/// [Reference](https://docs.luarmor.net/#tab-id-200-ok)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
pub struct ApiKeyStatsResponse {
    pub execution_data: ExecutionData,
    pub stats: ScriptStats,
}

/// [Reference](https://docs.luarmor.net/#tab-id-200-ok)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
pub struct ApiKeyStatsNoUsersResponse {
    pub execution_data: ExecutionData,
    pub stats: ScriptStatsNoUsers,
}
