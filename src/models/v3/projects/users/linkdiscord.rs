use api_builder::{Endpoint, api_endpoint};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// This will link a Discord ID to an existing key, and optionally override the current linked ID.
///
/// ⚠️ Authenticated ⚠️
///
/// [Reference](https://docs.luarmor.net/#linking-discord-id-to-a-key)
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, TypedBuilder,
)]
#[builder(field_defaults(setter(into)))]
pub struct LinkDiscordId<'a> {
    /// The associated project ID that contains the key.
    #[serde(skip)]
    pub project_id: &'a str,

    /// The user key to link to.
    pub user_key: &'a str,
    /// The Discord ID you want to associate with the key.
    #[builder(default, setter(strip_option))]
    pub discord_id: Option<&'a str>,
    /// If `true`, it will overwrite any current linked Discord ID.
    #[builder(default, setter(strip_option))]
    pub force: Option<bool>,
}
#[api_endpoint(method = POST, path = format!("/v3/projects/{}/users/linkdiscord", self.project_id), self_as_body = "application/json")]
impl Endpoint for LinkDiscordId<'_> {}
