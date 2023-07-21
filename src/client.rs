/// Dependencies
use reqwest::{ClientBuilder, header::{HeaderMap, HeaderValue}};
use crate::api_models::{APIResponse, StatusResponse, KeyDetailsResponse, KeyStatsResponse, CreatePayload, CreateResponse, EditPayload, BasicResponse, GetPayload, GetResponse, ResetHWIDPayload, LinkDiscordPayload};

/// Avoids boilerplate.
macro_rules! api_response {
    ($response:expr) => {{
        match $response.status().is_success() {
            true => APIResponse::Success($response.json().await?),
            false => APIResponse::Error($response.json().await?)
        }
    }};
}

/// Used to create requests to Luarmor.
pub struct Client {
    api_key: String,
    client: reqwest::Client
}
impl Client {
    /// Creates an instance.
    pub fn new(api_key: &str) -> Self {
        // Set default headers
        let mut default_headers = HeaderMap::new();
        default_headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        default_headers.insert("Authorization", HeaderValue::from_str(api_key).unwrap());

        // Return self
        Self {
            api_key: api_key.to_string(),
            client: ClientBuilder::new()
                .default_headers(default_headers)
                .build()
                .unwrap()
        }
    }

    /// This will return you the version information about the API.
    pub async fn status(&self) -> Result<APIResponse<StatusResponse>, reqwest::Error> {
        let response = self.client.get("https://api.luarmor.net/status").send().await?;
        Ok(api_response!(response))
    }
    
    /// You can get details of your API key. Project/Script IDs, execution amounts, script names etc...
    pub async fn key_details(&self) -> Result<APIResponse<KeyDetailsResponse>, reqwest::Error> {
        let response = self.client.get(format!("https://api.luarmor.net/v3/keys/{}/details", self.api_key)).send().await?;
        Ok(api_response!(response))
    }

    /// You can fetch the stats of your API key.
    /// This includes usage details, remaining obfuscations, max obfuscations, max users, execution amounts, monthly execution graph values etc...
    /// 
    /// If `no_users=true`, it will return the info about user limits. (e.g how many users there are, # of banned, # of whitelisted)
    /// This parameter is optional, you don't have to include it at all. If included, server might respond 0.001s faster
    pub async fn key_stats(&self, no_users: bool) -> Result<APIResponse<KeyStatsResponse>, reqwest::Error> {
        let response = self.client.get(format!("https://api.luarmor.net/v3/keys/{}/stats?noUsers={}", self.api_key, no_users)).send().await?;
        Ok(api_response!(response))
    }

    /// This endpoint will generate a key. 
    /// 
    /// If you don't specify the parameters, key will be 'unassigned' which means a user with the key can claim it and automatically assign their HWID / Discord ID to the key.
    /// 
    /// Users who have their HWIDs linked to the keys will be able to run the script.
    /// If they don't include `script_key` on top of their script, they will not be able to run the script as long as the FFA mode isn't on.
    pub async fn create_key(&self, project_id: &str, payload: &CreatePayload) -> Result<APIResponse<CreateResponse>, reqwest::Error> {
        let response = self.client.post(format!("https://api.luarmor.net/v3/projects/{}/users", project_id)).json(payload).send().await?;
        Ok(api_response!(response))
    }

    /// You can use this endpoint to edit an already existing user.
    /// 
    /// If you don't provide a specific field, API will assume that you don't want to change that property, so it's going to stay the same.
    pub async fn update_key(&self, project_id: &str, payload: &EditPayload) -> Result<APIResponse<BasicResponse>, reqwest::Error> {
        let response = self.client.patch(format!("https://api.luarmor.net/v3/projects/{}/users", project_id)).json(payload).send().await?;
        Ok(api_response!(response))
    }

    /// You can delete a key from your script, this will also remove the access of the user who has their hwid/discord id linked to that key.
    pub async fn delete_key(&self, project_id: &str, user_key: &str) -> Result<APIResponse<BasicResponse>, reqwest::Error> {
        let response = self.client.delete(format!("https://api.luarmor.net/v3/projects/{}/users?user_key={}", project_id, user_key)).send().await?;
        Ok(api_response!(response))
    }

    /// You can fetch all users from a script, and you can specify filters too. (such as `discord_id`, `identifier` etc.)
    /// 
    /// If you want to get someone's `user_key` from their `discord_id`, the key must have it linked first.
    /// 
    /// Note that response body will be an object array containing users. If you specified a filter value (e.g `discord_id=124345`) the array will contain one user so you just have to read `users[0]`;
    pub async fn get_keys(&self, project_id: &str, payload: &GetPayload) -> Result<APIResponse<GetResponse>, reqwest::Error> {
        let response = self.client.post(format!("https://api.luarmor.net/v3/projects/{}/users", project_id)).query(payload).send().await?;
        Ok(api_response!(response))
    }

    /// Resets the HWID for a `user_key`.
    pub async fn reset_hwid(&self, project_id: &str, payload: &ResetHWIDPayload) -> Result<APIResponse<BasicResponse>, reqwest::Error> {
        let response = self.client.post(format!("https://api.luarmor.net/v3/projects/{}/users/resethwid", project_id)).json(payload).send().await?;
        Ok(api_response!(response))
    }

    /// Links a discord ID to a `user_key`.
    pub async fn link_discord(&self, project_id: &str, payload: &LinkDiscordPayload) -> Result<APIResponse<BasicResponse>, reqwest::Error> {
        let response = self.client.post(format!("https://api.luarmor.net/v3/projects/{}/users/linkdiscord", project_id)).json(payload).send().await?;
        Ok(api_response!(response))
    }
}