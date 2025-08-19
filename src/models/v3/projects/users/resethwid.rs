use api_builder::{Endpoint, api_endpoint};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// This will reset the HWID of a key.
///
/// ⚠️ Authenticated ⚠️
///
/// [Reference](https://docs.luarmor.net/#resetting-the-hwid-of-a-key)
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, TypedBuilder,
)]
#[builder(field_defaults(setter(into)))]
pub struct ResetHwid<'a> {
    /// The associated project ID that contains the key.
    #[serde(skip)]
    pub project_id: &'a str,

    /// The user key to reset the HWID of.
    pub user_key: &'a str,
    /// If `true`, it will ignore the reset HWID cooldown.
    #[builder(default, setter(strip_option))]
    pub force: Option<bool>,
}
#[api_endpoint(method = POST, path = format!("/v3/projects/{}/users/resethwid", self.project_id), self_as_body = "application/json")]
impl Endpoint for ResetHwid<'_> {}
