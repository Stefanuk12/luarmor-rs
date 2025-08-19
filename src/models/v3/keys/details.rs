use api_builder::{Endpoint, api_endpoint};
use serde::Deserialize;
use time::OffsetDateTime;
use typed_builder::TypedBuilder;

/// You can get details of your API key.
///
/// This includes:
/// - Project/Script IDs
/// - execution amounts
/// - script names
/// - etc..
///
/// ⚠️ Authenticated ⚠️
///
/// View full response at [ApiKeyDetailsResponse].
///
/// [Reference](https://docs.luarmor.net/#getting-api-key-details)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ApiKeyDetails<'a> {
    pub api_key: &'a str,
}
#[api_endpoint(method = GET, path = format!("/v3/keys/{}/details", self.api_key))]
impl Endpoint for ApiKeyDetails<'_> {}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum KeyPlan {
    Basic,
    Premium,
    Pro,
    Other(char),
}
impl<'de> Deserialize<'de> for KeyPlan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match char::deserialize(deserializer)? {
            'b' => Ok(Self::Basic),
            'p' => Ok(Self::Premium),
            'r' => Ok(Self::Pro),
            x => Ok(Self::Other(x)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectPlatform {
    #[default]
    Roblox,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ResetHwidCooldown {
    Never,
    Specified(OffsetDateTime),
}
impl<'de> Deserialize<'de> for ResetHwidCooldown {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match i64::deserialize(deserializer)? {
            x if x < 0 => Ok(Self::Never),
            x => Ok(Self::Specified(
                OffsetDateTime::from_unix_timestamp(x)
                    .map_err(|err| serde::de::Error::custom(err))?,
            )),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
pub struct ProjectSettings {
    pub reset_hwid_cooldown: ResetHwidCooldown,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
pub struct Script {
    pub script_name: String,
    pub script_id: String,
    pub script_version: String,
    pub ffa: bool,
    pub silent: bool,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
pub struct Project {
    pub platform: ProjectPlatform,
    pub id: String,
    pub name: String,
    pub settings: ProjectSettings,
    pub scripts: Vec<Script>,
}

/// [Reference](https://docs.luarmor.net/#tab-id-200-ok)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
pub struct ApiKeyDetailsResponse {
    pub email: String,
    pub discord_id: String,
    #[serde(with = "time::serde::timestamp")]
    pub expires_at: OffsetDateTime,
    #[serde(with = "time::serde::timestamp")]
    pub registered_at: OffsetDateTime,
    #[serde(deserialize_with = "crate::models::v3::deserialize_number_as_bool")]
    pub enabled: bool,
    pub plan: KeyPlan,
    pub projects: Vec<Project>,
}
