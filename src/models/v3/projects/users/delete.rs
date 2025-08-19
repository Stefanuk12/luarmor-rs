use api_builder::{Endpoint, api_endpoint};
use typed_builder::TypedBuilder;

/// This will delete an existing key, effectively removing the access of the user who has their HWID and Discord ID linked to that key.
///
/// ⚠️ Authenticated ⚠️
///
/// [Reference](https://docs.luarmor.net/#deleting-the-key)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DeleteUser<'a> {
    /// The associated project ID that contains the key.
    pub project_id: &'a str,

    /// The user key to delete.
    pub user_key: &'a str,
}
#[api_endpoint(method = DELETE, path = format!("/v3/projects/{}/users?user_key={}", self.project_id, self.user_key))]
impl Endpoint for DeleteUser<'_> {}
