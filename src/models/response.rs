use std::{convert::Infallible, str::FromStr};

use api_builder::APIClientError;
use serde::Deserialize;

/// Possible messages.
///
/// Messages can change anytime per the API docs; don't rely on them.
/// If you find any missing, please open a PR.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, thiserror::Error)]
pub enum LuarmorMessage {
    /// Success
    #[error("API is up and working")]
    APIWorking,
    /// Incorrect API key
    #[error("Invalid API key! Visit https://luarmor.net/ to get access")]
    IncorrectAPIKey,
    /// Invalid API key
    #[error("Wrong API key")]
    InvalidAPIKey,
    /// Success
    #[error("Success")]
    Success,
    /// Key has been deleted
    #[error("User has been deleted")]
    UserDeleted,
    /// Wrong project id / user key
    #[error("Key not found")]
    KeyNotFound,
    /// Project not found
    #[error("Project not found")]
    ProjectNotFound,
    /// Success!
    #[error("Successfully reset")]
    SuccessReset,
    //// Bad request
    ///
    /// There are few reasons you might be getting 400 error.
    /// - User reset their hwid too frequently. You can use "force":true parameter to bypass this
    /// - Reset hwid is disabled for this project. You can force it
    /// - User is banned. In this case, you need to unban it first.
    #[error("User is on cooldown")]
    UserCooldown,
    /// Key or project not found
    #[error("User key does not exist")]
    UserKeyNotFound,
    /// Bad request
    ///
    /// There are few reasons you might be getting 400 error.
    /// - There's already a discord ID linked to this key. You can use "force":true parameter to bypass this
    /// - There's another key that's linked to same discord ID. In this case, you need to delete the other key.
    /// - Invalid discord ID.
    #[error("The key already has a Discord linked to it")]
    DiscordAlreadyLinked,
    /// This is not found in the docs.
    #[error("nothing to see here")]
    NothingToSee,
    /// This is not found in the docs.
    ///
    /// Happens when creating a key with an identifier that already exists.
    #[error("Identifier already exists")]
    IdentifierAlreadyExists,
    /// This is not found in the docs.
    ///
    /// Happens when linking an invalid discord_id.
    #[error("Invalid Discord ID")]
    InvalidDiscordId,
    /// This is not found in the docs.
    #[error("Discord ID successfully linked")]
    DiscordIdSuccess,
    /// This is not found in the docs.
    ///
    /// Happens when you try to reset HWID without `force=true` and Reset Hwid is disabled for the script.
    #[error("Reset HWID is disabled for this script")]
    ResetHWIDDisabled,
    /// This is not found in the docs.
    ///
    /// Happens when you successfully edit a user / key.
    #[error("User has been edited successfully")]
    EditSuccess,
    /// This is not found in the docs.
    #[error("Discord ID already exists")]
    DiscordAlreadyExists,
    #[error("{0}")]
    Other(String),
}
impl APIClientError for LuarmorMessage {}
impl FromStr for LuarmorMessage {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "API is up and working!" => Self::APIWorking,
            "Invalid API key! Visit https://luarmor.net/ to get access." => Self::IncorrectAPIKey,
            "Wrong API key" => Self::InvalidAPIKey,
            "Success!" => Self::Success,
            "User has been deleted!" => Self::UserDeleted,
            "Key not found" => Self::KeyNotFound,
            "Project not found!" => Self::ProjectNotFound,
            "Successfully reset!" => Self::SuccessReset,
            "User is on cooldown." => Self::UserCooldown,
            "User key doesn't exist" => Self::UserKeyNotFound,
            "This key already has a discord linked to it" => Self::DiscordAlreadyLinked,
            "nothing to see here." => Self::NothingToSee,
            "Identifier already exists." => Self::IdentifierAlreadyExists,
            "Invalid discord_id" => Self::InvalidDiscordId,
            "Discord ID successfully linked!" => Self::DiscordIdSuccess,
            "Reset Hwid is disabled for this script" => Self::ResetHWIDDisabled,
            "User has been edited successfully!" => Self::EditSuccess,
            "Project not found" => Self::ProjectNotFound,
            "Discord ID already exists" => Self::DiscordAlreadyExists,
            "Discord ID already exists." => Self::DiscordAlreadyExists,
            "Discord ID already exist." => Self::DiscordAlreadyExists,
            s => Self::Other(s.to_string()),
        })
    }
}
impl<'de> Deserialize<'de> for LuarmorMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_str(s.as_str()).unwrap()) // Infallible
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
pub struct LuarmorResponse<T> {
    pub success: bool,
    pub message: LuarmorMessage,
    #[serde(flatten)]
    pub data: Option<T>,
}
