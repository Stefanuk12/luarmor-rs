use api_builder::{Endpoint, api_endpoint};
use typed_builder::TypedBuilder;

/// This will unblacklist a key.
/// It does not need strict API key authentication, only the `unban_token` which is unique and changes each time a key is blacklisted.
///
/// [Reference](https://docs.luarmor.net/#unblacklisting-a-key)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct UnblacklistUser<'a> {
    /// The associated project ID that contains the blacklisted key.
    pub project_id: &'a str,
    /// The unban token, a 32 character random string automatically generated when you blacklist someone.
    /// You can get the blacklist token from [crate::models::v3::projects::users::GetUsers].
    pub unban_token: &'a str,
}
#[api_endpoint(method = GET, path = format!("/v3/projects/{}/users/unban?unban_token={}", self.project_id, self.unban_token))]
impl Endpoint for UnblacklistUser<'_> {}
