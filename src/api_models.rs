// Dependencies
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, TimestampSeconds, formats::Flexible};
use std::time::SystemTime;

/// An API response.
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
#[derive(Clone, Debug, Serialize, Deserialize, strum::Display, strum::EnumString)]
pub enum Messages {
    /// Success
    #[strum(serialize="API is up and working!")]
    APIWorking,
    /// Incorrect API key
    #[strum(serialize="Invalid API key! Visit https://luarmor.net/ to get access.")]
    IncorrectAPIKey,
    /// Invalid API key
    #[strum(serialize="Wrong API key")]
    InvalidAPIKey,
    /// Success
    #[strum(serialize="Success!")]
    Success,
    /// Key has been deleted
    #[strum(serialize="User has been deleted!")]
    UserDeleted,
    /// Wrong project id / user key
    #[strum(serialize="Key not found")]
    KeyNotFound,
    /// Project not found
    #[strum(serialize="Project not found!")]
    ProjectNotFound,
    /// Success!
    #[strum(serialize="Sucessfully reset!")]
    SuccessReset,
    //// Bad request
    /// 
    /// There are few reasons you might be getting 400 error.
    /// - User reset their hwid too frequently. You can use "force":true parameter to bypass this
    /// - Reset hwid is disabled for this project. You can force it
    /// - User is banned. In this case, you need to unban it first.
    #[strum(serialize="User is on cooldown.")]
    UserCooldown,
    /// Key or project not found
    #[strum(serialize="User key doesn't exist")]
    UserKeyNotFound,
    /// Bad request
    /// 
    /// There are few reasons you might be getting 400 error.
    /// - There's already a discord ID linked to this key. You can use "force":true parameter to bypass this
    /// - There's another key that's linked to same discord ID. In this case, you need to delete the other key.
    /// - Invalid discord ID.
    #[strum(serialize="This key already has a discord linked to it")]
    DiscordAlreadyLinked
}

/// A basic response.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BasicResponse {
    success: bool,
    message: String
}

/// Response for `/status`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    version: String,
    active: bool,
    message: String,
    warning: bool,
    warning_message: String
}

/// Project settings.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectSettings {
    reset_hwid_cooldown: i32
}

/// A script.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Script {
    script_name: String,
    script_id: String,
    script_version: String,
    ffa: bool,
    silent: bool
}

/// A project.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    platform: String,
    id: String,
    name: String,
    settings: ProjectSettings,
    scripts: Vec<Script>
}

/// Response for `/details`.
#[serde_with::serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyDetailsResponse {
    success: bool,
    message: String,
    email: String,
    discord_id: String,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    expires_at: SystemTime,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    registered_at: SystemTime,
    plan: String,
    enabled: u8,
    projects: Vec<Project>
}

/// Used within [`KeyStatsResponse`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionData {
    frequency: u64,
    executions: Vec<u64>
}

/// Used within [`Stats`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefaultStats {
    scripts: u64,
    users: u64,
    obfuscations: u64
}

/// Used within [`KeyStatsResponse`].
#[serde_with::serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    obfuscations: u64,
    scripts: u64,
    /// Only defined if `no_users` was set to `false`.
    users: Option<u64>,
    /// Only defined if `no_users` was set to `false`.
    attacks_blocked: Option<u64>,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    reset_at: SystemTime,
    default: DefaultStats
}

/// Response for `/stats`.
#[serde_with::serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyStatsResponse {
    success: bool,
    message: String,
    execution_data: ExecutionData,
    stats: Stats
}

/// The payload for creating a key / user.
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreatePayload {
    /// Identifier of the user to whitelist.
    /// 
    /// Could be a HWID.
    identifier: Option<String>,
    /// Unix timestamp (seconds) of expiry date.
    /// 
    /// If you don't provide this field, it will never expire.
    #[serde_as(as = "Option<TimestampSeconds<String, Flexible>>")]
    auth_expire: Option<SystemTime>,
    /// Custom note for client.
    /// 
    /// This might be easier to identify the user.
    note: Option<String>,
    /// Discord ID of the user.
    /// 
    /// If not specified, user won't be able to resethwid on their own.
    /// They can still link their discord id to their key using /redeem command (if you configured the bot)
    discord_id: Option<String>
}

/// The response for creating a key / user
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    success: bool,
    message: String,
    user_key: String
}

/// The payload for editing an existing user.
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EditPayload {
    /// Unique user_key to edit.
    user_key: String,
    /// Identifier of the user to whitelist.
    /// 
    /// Could be a HWID.
    identifier: Option<String>,
    /// Unix timestamp (seconds) of expiry date.
    /// 
    /// If you don't want it to expire, use negative one (-1) as value.
    auth_expire: i32,
    /// Custom note for client.
    /// 
    /// This might be easier to identify the user.
    note: Option<String>,
    /// Discord ID of the user.
    /// 
    /// If not specified, user won't be able to resethwid on their own.
    /// They can still link their discord id to their key using /redeem command (if you configured the bot)
    discord_id: Option<String>
}

/// The payload for getting users.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPayload {
    /// Discord ID to get the connected user.
    discord_id: Option<String>,
    /// Key to get the connected user.
    user_key: Option<String>,
    /// HWID to get the connected user.
    identifier: Option<String>
}

/// A single user.
#[serde_with::serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    user_key: String,
    identifier: String,
    identifier_type: String,
    discord_id: String,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    last_reset: SystemTime,
    total_resets: u64,
    auth_expire: i32,
    /// Either `0` or `1`.
    banned: u8,
    ban_reason: String,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    ban_expire: SystemTime,
    unban_token: String,
    total_executions: u64,
    note: String,
    ban_ip: String
}

/// The response for getting users / keys.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetResponse {
    success: bool,
    message: String,
    users: Vec<User>
}

/// The payload for resetting the HWID of a key.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResetHWIDPayload {
    /// Key to reset the hwid of.
    user_key: String,
    /// Whether reset HWID is forced or not.
    /// 
    /// If `true`, it will ignore resethwid cooldown.
    force: Option<bool>
}

/// The payload for linking discord ID to a key.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkDiscordPayload {
    /// Key to link the discord ID.
    user_key: String,
    /// Discord Id (1234578635849).
    discord_id: String,
    /// If `true`, it will overwrite the discord ID if key already has one linked.
    force: Option<bool>
}