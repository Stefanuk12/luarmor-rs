use api_builder::{Endpoint, api_endpoint};
use serde::Serialize;
use time::OffsetDateTime;
use typed_builder::TypedBuilder;

/// Update fields of an already existing user.
///
/// ⚠️ Authenticated ⚠️
///
/// [Reference](https://docs.luarmor.net/#updating-an-existing-user)
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, TypedBuilder,
)]
#[builder(field_defaults(setter(into)))]
pub struct UpdateUser<'a> {
    /// The associated project ID that contains the key.
    #[serde(skip)]
    pub project_id: &'a str,

    /// The user key to update.
    pub user_key: &'a str,
    /// A unique identifier for the key, usually their HWID.
    #[builder(default, setter(strip_option))]
    pub identifier: Option<&'a str>,
    /// The unix timestamp of when the key should expire.
    ///
    /// NOTE: if this is not defined, the key will never expire; read the struct comments for more information.
    #[serde(with = "time::serde::timestamp::option")]
    #[builder(default, setter(strip_option))]
    pub auth_expire: Option<OffsetDateTime>,
    /// A custom note for the key, which can be used to identify the user and key or provide additional information about them.
    #[builder(default, setter(strip_option))]
    pub note: Option<&'a str>,
    /// The Discord ID associated with the key.
    /// However if you set up the Discord bot, the user can manually redeem their key and link it via the "Redeem" button on the panel.
    ///
    /// NOTE: if a Discord ID is not associated with a key, the user cannot use the `/resethwid` discord command.
    #[builder(default, setter(strip_option))]
    pub discord_id: Option<&'a str>,
}
#[api_endpoint(method = PATCH, path = format!("/v3/projects/{}/users", self.project_id), self_as_body = "application/json")]
impl Endpoint for UpdateUser<'_> {}
