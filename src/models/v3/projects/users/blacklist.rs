use api_builder::{Endpoint, api_endpoint};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// This will blacklist an existing key, and the HWID linked to it (if any).
///
/// ⚠️ Authenticated ⚠️
///
/// [Reference](https://docs.luarmor.net/#blacklisting-a-key)
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, TypedBuilder,
)]
#[builder(field_defaults(setter(into)))]
pub struct BlacklistUser<'a> {
    /// The associated project ID that contains the key.
    #[serde(skip)]
    pub project_id: &'a str,

    /// The user key to blacklist.
    pub user_key: &'a str,
    /// The reason of the blacklist.
    /// This will be shown to the user when they execute.
    #[builder(default, setter(strip_option))]
    pub ban_reason: Option<&'a str>,
    /// The **exact** unix timestamp of the blacklist expiry date.
    ///
    /// NOTE: a value of less than 0 will result in an infinite ban
    #[builder(default, setter(strip_option))]
    pub ban_expire: Option<i32>,
}
#[api_endpoint(method = POST, path = format!("/v3/projects/{}/users/blacklist", self.project_id), self_as_body = "application/json")]
impl Endpoint for BlacklistUser<'_> {}
