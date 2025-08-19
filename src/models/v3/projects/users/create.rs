use api_builder::{Endpoint, api_endpoint};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use typed_builder::TypedBuilder;

/// This will generate a new key.
///
/// If the parameters aren't specified, the key will be 'unassigned'.
/// An unassigned key means a user with the key can claim it and automatically assign their HWID and Discord ID to the key.
///
/// Users who have their HWIDs linked to the keys will be able to run the script.
/// If they don't include `script_key` on top of their script and Free-For-All (FFA) is disabled, they will not be able to run the script.
///
/// View full response at [CreateUserResponse].
///
/// ## `auth_expire` vs `key_days`
///
/// `key_days` specifies the number of days a key will have once it's been redeemed or executed for the very first time.
/// This allows you to generate unused time-locked keys without their timer starting right away so you can stock them.
///
/// On the other hand, `auth_expire` is the actual timestamp of the expiry date.
/// If you generate keys via `key_days`, once that key has been activated, it will automatically adjust `auth_expire` according to current time + (`key_days` * 86400).
///
/// If you don't provide `key_days` and provide `auth_expire` directly, you must include one of `identifier` or `discord_id` parameters to tell the server that it is a claimed key, so it will start counting towards their remaining days instantly.
/// If you don't provide `identifier` or `discord_id` fields, it will automatically convert the offset between current time and `auth_expire` to `key_days`.
///
/// TODO: consider making a separate struct to ensure the above case doesn't happen...
///
/// ⚠️ Authenticated ⚠️
///
/// [Reference](https://docs.luarmor.net/#creating-a-key-user)
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, TypedBuilder,
)]
#[builder(field_defaults(setter(into)))]
pub struct CreateUser<'a> {
    /// The associated project ID that you want to add the user key to.
    #[serde(skip)]
    pub project_id: &'a str,

    /// A unique identifier for the key, usually their HWID.
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
    /// NOTE: if a Discord ID is not associated with a key, the user cannot use the /resethwid discord command.
    #[builder(default, setter(strip_option))]
    pub discord_id: Option<&'a str>,
    /// The number of days the key will have **after** it has been activated by a user.
    ///
    /// For more information, read the struct comments.
    #[builder(default, setter(strip_option))]
    pub key_days: Option<u32>,
}
#[api_endpoint(method = POST, path = format!("/v3/projects/{}/users", self.project_id), self_as_body = "application/json")]
impl Endpoint for CreateUser<'_> {}

/// [Reference](https://docs.luarmor.net/#tab-id-200-ok-key-has-been-added-successfully)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
pub struct CreateUserResponse {
    pub user_key: String,
}
