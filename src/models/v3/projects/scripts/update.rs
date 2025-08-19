use api_builder::{Endpoint, api_endpoint};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Programmatically edit a script.
///
/// ⚠️ Authenticated ⚠️
///
/// View full response at [UpdateScriptResponse].
///
/// [Reference](https://docs.luarmor.net/#updating-a-script)
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, TypedBuilder,
)]
#[builder(field_defaults(setter(into)))]
pub struct UpdateScript<'a> {
    /// The associated project ID that contains the script.
    pub project_id: &'a str,
    /// The id of the script to update.
    pub script_id: &'a str,
    /// The raw Lua source code to upload.
    pub script: &'a str,

    /// Disables Luarmor console outputs.
    /// This is not recommended as it's useful for debugging.
    /// However, it may be necessary to bypass anti cheats that detect console outputs.
    #[builder(default, setter(strip_option))]
    pub silent: Option<bool>,
    /// Free-For-All (FFA) mode allows anyone to execute the script, without a script key.
    /// Use with caution, you may not want this behaviour.
    #[builder(default, setter(strip_option))]
    pub ffa: Option<bool>,
    /// Ensures clients are connected via a heartbeat.
    /// This is recommended as it improves security.
    /// However, in some cases, it may be mandatory if you require instance limits or tracking.
    #[builder(default = Some(true), setter(strip_option))]
    pub heartbeat: Option<bool>,
    /// Removes some inline security checks to make your script run faster.
    /// This is not recommended as it decreases the security a little, and [optimising your script](https://luarmor.mintlify.app/scripting/optimisation) is preferred.
    #[builder(default, setter(strip_option))]
    pub lightning: Option<bool>,
}
#[api_endpoint(method = PUT, path = format!("/v3/projects/{}/scripts/{}", self.project_id, self.script_id), self_as_body = "application/json")]
impl Endpoint for UpdateScript<'_> {}
