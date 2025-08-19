use api_builder::{Endpoint, QueryParamPair, QueryParamPairs, api_endpoint};
use serde::{Deserialize, Deserializer, Serialize};
use time::OffsetDateTime;
use typed_builder::TypedBuilder;

/// This will fetch **all** filtered users within a project.
///
/// NOTE: if you want to get someone's `user_key` from their `discord_id`, they must have linked their key first.
///
/// View full response at [GetUsersResponse].
///
/// ⚠️ Authenticated ⚠️
///
/// [Reference](https://docs.luarmor.net/#getting-users)
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, TypedBuilder,
)]
#[builder(field_defaults(setter(into)))]
pub struct GetUsers<'a> {
    /// The associated project ID that contains the key.
    #[serde(skip)]
    pub project_id: &'a str,

    /// Filter for a specific user, based upon their key.
    #[builder(default, setter(strip_option))]
    pub user_key: Option<&'a str>,
    /// Filter for a specific user, based upon their Discord ID.
    #[builder(default, setter(strip_option))]
    pub discord_id: Option<&'a str>,
    /// Filter for a specific user, based upon their identifier (HWID).
    #[builder(default, setter(strip_option))]
    pub identifier: Option<&'a str>,

    /// Filter based upon a search string that can be found anywhere, i.e. `identifier`, `user_key`, `discord_id`, `note`.
    #[builder(default, setter(strip_option))]
    pub search: Option<&'a str>,

    /// Pagination start index, not page.
    #[builder(default, setter(strip_option))]
    pub from: Option<u32>,
    /// Pagination end index, not page.
    #[builder(default, setter(strip_option))]
    pub until: Option<u32>,
}
#[api_endpoint(method = GET, path = format!("/v3/projects/{}/users", self.project_id), self_as_body = "application/json")]
impl Endpoint for GetUsers<'_> {
    fn query_params(&self) -> Option<QueryParamPairs> {
        let mut params = QueryParamPairs::default();

        if let Some(x) = self.user_key {
            params.push(QueryParamPair::new("user_key", x.to_string()));
        }
        if let Some(x) = self.discord_id {
            params.push(QueryParamPair::new("discord_id", x.to_string()));
        }
        if let Some(x) = self.identifier {
            params.push(QueryParamPair::new("identifier", x.to_string()));
        }
        if let Some(x) = self.search {
            params.push(QueryParamPair::new("search", x.to_string()));
        }
        if let Some(x) = self.from {
            params.push(QueryParamPair::new("from", x.to_string()));
        }
        if let Some(x) = self.until {
            params.push(QueryParamPair::new("until", x.to_string()));
        }

        Some(params)
    }
}

fn deserialize_empty_string<'a, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'a>,
{
    let x = String::deserialize(deserializer)?;
    match x.as_str() {
        "" => Ok(None),
        _ => Ok(Some(x)),
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum UserIdentifierType {
    HWID,
    #[default]
    None,
}
impl<'de> Deserialize<'de> for UserIdentifierType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "HWID" => Ok(Self::HWID),
            _ => Ok(Self::None),
        }
    }
}

/// Each key will have a `status` field.
///
/// - [UserStatus::Active]
///     The user has linked their HWID to the key and it is active.
/// - [UserStatus::Reset]
///     The user has reset their HWID and it's waiting to be assigned, upontheir first execution.
/// - [UserStatus::Banned]
///     The user does not have a key linked and banned.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Reset,
    Banned,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum UserExpiration {
    Never,
    Specified(OffsetDateTime),
}
impl<'de> Deserialize<'de> for UserExpiration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
pub struct User {
    #[serde(deserialize_with = "deserialize_empty_string")]
    pub user_key: Option<String>,
    #[serde(deserialize_with = "deserialize_empty_string")]
    pub identifier: Option<String>,
    pub identifier_type: UserIdentifierType,
    #[serde(deserialize_with = "deserialize_empty_string")]
    pub discord_id: Option<String>,
    pub status: UserStatus,
    #[serde(with = "time::serde::timestamp")]
    pub last_reset: OffsetDateTime,
    pub total_resets: u32,
    pub auth_expire: UserExpiration,
    #[serde(deserialize_with = "crate::models::v3::deserialize_number_as_bool")]
    pub banned: bool,
    #[serde(deserialize_with = "deserialize_empty_string")]
    pub ban_reason: Option<String>,
    #[serde(with = "time::serde::timestamp")]
    pub ban_expire: OffsetDateTime,
    #[serde(deserialize_with = "deserialize_empty_string")]
    pub unban_token: Option<String>,
    pub total_executions: u64,
    #[serde(deserialize_with = "deserialize_empty_string")]
    pub note: Option<String>,
    #[serde(deserialize_with = "deserialize_empty_string")]
    pub ban_ip: Option<String>,
}

/// [Reference](https://docs.luarmor.net/#tab-id-200-ok-success-2)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
pub struct GetUsersResponse {
    pub users: Vec<User>,
}
