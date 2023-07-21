// Dependencies
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, TimestampSeconds, formats::Flexible};
use std::time::SystemTime;

/// An API response.
#[derive(Debug, Clone)]
pub enum APIResponse<T> {
    Error(BasicResponse),
    Success(T)
}

/// User status.
#[derive(Clone, Debug, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all="lowercase")]
pub enum UserStatus {
    /// User has linked their hwid to key and its active.
    Active,
    /// User has reset their hwid and it's waiting to be assigned upon first execution.
    Reset,
    /// User doesn't have a key linked (aka. unknown) and banned.
    Banned
}

/// Possible messages.
/// 
/// Federal said "messages can change anytime", "you must not rely on them".
/// If you find any missing, make a PR.
#[derive(Clone, Debug, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[non_exhaustive]
pub enum Messages {
    /// Success
    #[strum(serialize="API is up and working!")]
	#[serde(rename="API is up and working!")]
    APIWorking,
    /// Incorrect API key
    #[strum(serialize="Invalid API key! Visit https://luarmor.net/ to get access.")]
	#[serde(rename="Invalid API key! Visit https://luarmor.net/ to get access.")]
    IncorrectAPIKey,
    /// Invalid API key
    #[strum(serialize="Wrong API key")]
	#[serde(rename="Wrong API key")]
    InvalidAPIKey,
    /// Success
    #[strum(serialize="Success!")]
	#[serde(rename="Success!")]
    Success,
    /// Key has been deleted
    #[strum(serialize="User has been deleted!")]
	#[serde(rename="User has been deleted!")]
    UserDeleted,
    /// Wrong project id / user key
    #[strum(serialize="Key not found")]
	#[serde(rename="Key not found")]
    KeyNotFound,
    /// Project not found
    #[strum(serialize="Project not found!")]
	#[serde(rename="Project not found!")]
    ProjectNotFound,
    /// Success!
    #[strum(serialize="Successfully reset!")]
	#[serde(rename="Successfully reset!")]
    SuccessReset,
    //// Bad request
    /// 
    /// There are few reasons you might be getting 400 error.
    /// - User reset their hwid too frequently. You can use "force":true parameter to bypass this
    /// - Reset hwid is disabled for this project. You can force it
    /// - User is banned. In this case, you need to unban it first.
    #[strum(serialize="User is on cooldown.")]
	#[serde(rename="User is on cooldown.")]
    UserCooldown,
    /// Key or project not found
    #[strum(serialize="User key doesn't exist")]
	#[serde(rename="User key doesn't exist")]
    UserKeyNotFound,
    /// Bad request
    /// 
    /// There are few reasons you might be getting 400 error.
    /// - There's already a discord ID linked to this key. You can use "force":true parameter to bypass this
    /// - There's another key that's linked to same discord ID. In this case, you need to delete the other key.
    /// - Invalid discord ID.
    #[strum(serialize="This key already has a discord linked to it")]
	#[serde(rename="This key already has a discord linked to it")]
    DiscordAlreadyLinked,
    /// This is not found in the docs.
    NothingToSee,
    /// This is not found in the docs.
    /// 
    /// Happens when creating a key with an identifier that already exists.
    #[strum(serialize="Identifier already exists.")]
	#[serde(rename="Identifier already exists.")]
    IdentifierAlreadyExists,
    /// This is not found in the docs.
    /// 
    /// Happens when linking an invalid discord_id.
    #[strum(serialize="Invalid discord_id")]
	#[serde(rename="Invalid discord_id")]
    InvalidDiscordId,
    /// This is not found in the docs.
    #[strum(serialize="Discord ID successfully linked!")]
	#[serde(rename="Discord ID successfully linked!")]
    DiscordIdSuccess,
    /// This is not found in the docs.
    /// 
    /// Happens when you try to reset HWID without `force=true` and Reset Hwid is disabled for the script.
    #[strum(serialize="Reset Hwid is disabled for this script")]
	#[serde(rename="Reset Hwid is disabled for this script")]
    ResetHWIDDisabled,
    /// This is not found in the docs.
    /// 
    /// Happens when you successfully edit a user / key.
    #[strum(serialize="User has been edited successfully!")]
	#[serde(rename="User has been edited successfully!")]
    EditSuccess,
}

/// A basic response.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BasicResponse {
    pub success: bool,
    pub message: Messages
}

/// Response for `/status`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub version: String,
    pub active: bool,
    pub message: Messages,
    pub warning: bool,
    pub warning_message: Messages
}

/// Project settings.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub reset_hwid_cooldown: i32
}

/// A script.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Script {
    pub script_name: String,
    pub script_id: String,
    pub script_version: String,
    pub ffa: bool,
    pub silent: bool
}

/// A project.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub platform: String,
    pub id: String,
    pub name: String,
    pub settings: ProjectSettings,
    pub scripts: Vec<Script>
}

/// Response for `/details`.
#[serde_with::serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyDetailsResponse {
    pub success: bool,
    pub message: Messages,
    pub email: String,
    pub discord_id: String,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub expires_at: SystemTime,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub registered_at: SystemTime,
    pub plan: String,
    pub enabled: u8,
    pub projects: Vec<Project>
}

/// Used within [`KeyStatsResponse`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionData {
    pub frequency: u64,
    pub executions: Vec<u64>
}

/// Used within [`Stats`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefaultStats {
    pub scripts: u64,
    pub users: u64,
    pub obfuscations: u64
}

/// Used within [`KeyStatsResponse`].
#[serde_with::serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub obfuscations: u64,
    pub scripts: u64,
    /// Only defined if `no_users` was set to `false`.
    pub users: Option<u64>,
    /// Only defined if `no_users` was set to `false`.
    pub attacks_blocked: Option<u64>,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub reset_at: SystemTime,
    pub default: DefaultStats
}

/// Response for `/stats`.
#[serde_with::serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyStatsResponse {
    pub success: bool,
    pub message: Messages,
    pub execution_data: ExecutionData,
    pub stats: Stats
}

/// The payload for creating a key / user.
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreatePayload {
    /// Identifier of the user to whitelist.
    /// 
    /// Could be a HWID.
    pub identifier: Option<String>,
    /// Unix timestamp (seconds) of expiry date.
    /// 
    /// If you don't provide this field, it will never expire.
    #[serde_as(as = "Option<TimestampSeconds<String, Flexible>>")]
    pub auth_expire: Option<SystemTime>,
    /// Custom note for client.
    /// 
    /// This might be easier to identify the user.
    pub note: Option<String>,
    /// Discord ID of the user.
    /// 
    /// If not specified, user won't be able to resethwid on their own.
    /// They can still link their discord id to their key using /redeem command (if you configured the bot)
    pub discord_id: Option<String>
}

/// The response for creating a key / user
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub success: bool,
    pub message: Messages,
    pub user_key: String
}

/// The payload for editing an existing user.
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EditPayload {
    /// Unique user_key to edit.
    pub user_key: String,
    /// Identifier of the user to whitelist.
    /// 
    /// Could be a HWID.
    pub identifier: Option<String>,
    /// Unix timestamp (seconds) of expiry date.
    /// 
    /// If you don't want it to expire, use negative one (-1) as value.
    pub auth_expire: i32,
    /// Custom note for client.
    /// 
    /// This might be easier to identify the user.
    pub note: Option<String>,
    /// Discord ID of the user.
    /// 
    /// If not specified, user won't be able to resethwid on their own.
    /// They can still link their discord id to their key using /redeem command (if you configured the bot)
    pub discord_id: Option<String>
}

/// The payload for getting users.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPayload {
    /// Discord ID to get the connected user.
    pub discord_id: Option<String>,
    /// Key to get the connected user.
    pub user_key: Option<String>,
    /// HWID to get the connected user.
    pub identifier: Option<String>
}

/// A single user.
#[serde_with::serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub user_key: String,
    pub identifier: String,
    pub identifier_type: String,
    pub discord_id: String,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub last_reset: SystemTime,
    pub total_resets: u64,
    pub auth_expire: i32,
    /// Either `0` or `1`.
    pub banned: u8,
    pub ban_reason: String,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub ban_expire: SystemTime,
    pub unban_token: String,
    pub total_executions: u64,
    pub note: String,
    pub ban_ip: String
}

/// The response for getting users / keys.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetResponse {
    pub success: bool,
    pub message: Messages,
    pub users: Vec<User>
}

/// The payload for resetting the HWID of a key.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResetHWIDPayload {
    /// Key to reset the hwid of.
    pub user_key: String,
    /// Whether reset HWID is forced or not.
    /// 
    /// If `true`, it will ignore resethwid cooldown.
    pub force: Option<bool>
}

/// The payload for linking discord ID to a key.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkDiscordPayload {
    /// Key to link the discord ID.
    pub user_key: String,
    /// Discord Id (1234578635849).
    pub discord_id: String,
    /// If `true`, it will overwrite the discord ID if key already has one linked.
    pub force: Option<bool>
}