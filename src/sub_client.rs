use api_builder::{AsyncClient, AsyncQuery, Client, Query, RestClient, error::APIError};
use http::{header::AUTHORIZATION, HeaderValue};

use crate::{
    Luarmor,
    models::{
        LuarmorMessage,
        status::{ApiStatus, ApiStatusResponse},
        v3::{
            keys::{ApiKeyDetails, ApiKeyDetailsResponse, ApiKeyStats, ApiKeyStatsResponse},
            projects::{
                scripts::UpdateScript,
                users::{
                    BlacklistUser, CreateUser, CreateUserResponse, DeleteUser, GetUsers,
                    GetUsersResponse, LinkDiscordId, ResetHwid, UnblacklistUser, UpdateUser, User,
                },
            },
        },
    },
};

/// Used to create requests to Luarmor.
#[derive(Clone, Debug)]
pub struct LuarmorClient<C> {
    api_key: String,
    client: C,
}
impl<C> LuarmorClient<C> {
    /// Creates an instance.
    pub fn new(api_key: String, client: C) -> Self {
        Self { api_key, client }
    }
}
impl<C> LuarmorClient<C>
where
    C: Client<Error = LuarmorMessage>,
{
    pub fn details(&self) -> Result<ApiKeyDetailsResponse, APIError<C::Error>> {
        Luarmor(
            ApiKeyDetails::builder()
                .api_key(self.api_key.as_str())
                .build(),
        )
        .query(&self.client)
    }

    pub fn status(&self) -> Result<ApiStatusResponse, APIError<C::Error>> {
        ApiStatus.query(&self.client)
    }

    pub fn stats(&self, no_users: bool) -> Result<ApiKeyStatsResponse, APIError<C::Error>> {
        Luarmor(
            ApiKeyStats::builder()
                .api_key(self.api_key.as_str())
                .no_users(no_users)
                .build(),
        )
        .query(&self.client)
    }

    pub fn update_script(&self, payload: UpdateScript<'_>) -> Result<(), APIError<C::Error>> {
        Luarmor(payload).query(self)
    }

    pub fn blacklist(&self, payload: BlacklistUser<'_>) -> Result<(), APIError<C::Error>> {
        Luarmor(payload).query(self)
    }

    pub fn create_user(
        &self,
        payload: CreateUser<'_>,
    ) -> Result<String, APIError<C::Error>> {
        let x: Result<CreateUserResponse, APIError<C::Error>> = Luarmor(payload).query(self);
        x.map(|x| x.user_key)
    }

    pub fn delete_user(&self, project_id: &str, user_key: &str) -> Result<(), APIError<C::Error>> {
        Luarmor(
            DeleteUser::builder()
                .project_id(project_id)
                .user_key(user_key)
                .build(),
        )
        .query(self)
    }

    pub fn users(&self, payload: GetUsers<'_>) -> Result<Vec<User>, APIError<C::Error>> {
        let x: Result<GetUsersResponse, APIError<LuarmorMessage>> = Luarmor(payload).query(self);
        x.map(|x| x.users)
    }

    pub fn link_discord(&self, payload: LinkDiscordId<'_>) -> Result<(), APIError<C::Error>> {
        Luarmor(payload).query(self)
    }

    pub fn reset_hwid(&self, payload: ResetHwid<'_>) -> Result<(), APIError<C::Error>> {
        Luarmor(payload).query(self)
    }

    pub fn unblacklist(
        &self,
        project_id: &str,
        unban_token: &str,
    ) -> Result<(), APIError<C::Error>> {
        Luarmor(
            UnblacklistUser::builder()
                .project_id(project_id)
                .unban_token(unban_token)
                .build(),
        )
        .query(&self.client)
    }

    pub fn update_user(&self, payload: UpdateUser<'_>) -> Result<(), APIError<C::Error>> {
        Luarmor(payload).query(self)
    }
}
impl<C> LuarmorClient<C>
where
    C: AsyncClient<Error = LuarmorMessage> + Sync,
{
    pub async fn details_async(&self) -> Result<ApiKeyDetailsResponse, APIError<C::Error>> {
        Luarmor(
            ApiKeyDetails::builder()
                .api_key(self.api_key.as_str())
                .build(),
        )
        .query_async(&self.client)
        .await
    }

    pub async fn status_async(&self) -> Result<ApiStatusResponse, APIError<C::Error>> {
        ApiStatus.query_async(&self.client).await
    }

    pub async fn stats_async(
        &self,
        no_users: bool,
    ) -> Result<ApiKeyStatsResponse, APIError<C::Error>> {
        Luarmor(
            ApiKeyStats::builder()
                .api_key(self.api_key.as_str())
                .no_users(no_users)
                .build(),
        )
        .query_async(&self.client)
        .await
    }

    pub async fn update_script_async(
        &self,
        payload: UpdateScript<'_>,
    ) -> Result<(), APIError<C::Error>> {
        Luarmor(payload).query_async(self).await
    }

    pub async fn blacklist_async(
        &self,
        payload: BlacklistUser<'_>,
    ) -> Result<(), APIError<C::Error>> {
        Luarmor(payload).query_async(self).await
    }

    pub async fn create_user_async(
        &self,
        payload: CreateUser<'_>,
    ) -> Result<CreateUserResponse, APIError<C::Error>> {
        Luarmor(payload).query_async(self).await
    }

    pub async fn delete_user_async(
        &self,
        project_id: &str,
        user_key: &str,
    ) -> Result<(), APIError<C::Error>> {
        Luarmor(
            DeleteUser::builder()
                .project_id(project_id)
                .user_key(user_key)
                .build(),
        )
        .query_async(self)
        .await
    }

    pub async fn users_async(
        &self,
        payload: GetUsers<'_>,
    ) -> Result<Vec<User>, APIError<C::Error>> {
        let x: Result<GetUsersResponse, APIError<LuarmorMessage>> =
            Luarmor(payload).query_async(self).await;
        x.map(|x| x.users)
    }

    pub async fn link_discord_async(
        &self,
        payload: LinkDiscordId<'_>,
    ) -> Result<(), APIError<C::Error>> {
        Luarmor(payload).query_async(self).await
    }

    pub async fn reset_hwid_async(&self, payload: ResetHwid<'_>) -> Result<(), APIError<C::Error>> {
        Luarmor(payload).query_async(self).await
    }

    pub async fn unblacklist_async(
        &self,
        project_id: &str,
        unban_token: &str,
    ) -> Result<(), APIError<C::Error>> {
        Luarmor(
            UnblacklistUser::builder()
                .project_id(project_id)
                .unban_token(unban_token)
                .build(),
        )
        .query_async(&self.client)
        .await
    }

    pub async fn update_user_async(
        &self,
        payload: UpdateUser<'_>,
    ) -> Result<(), APIError<C::Error>> {
        Luarmor(payload).query_async(self).await
    }
}

// Adds `api_key` Authorization header
impl<C> RestClient for LuarmorClient<C>
where
    C: RestClient,
{
    type Error = C::Error;

    fn rest_endpoint(&self, path: &str) -> Result<api_builder::Url, APIError<Self::Error>> {
        self.client.rest_endpoint(path)
    }
}
impl<C> Client for LuarmorClient<C>
where
    C: Client,
{
    fn rest(
        &self,
        mut request: http::Request<Vec<u8>>,
    ) -> Result<http::Response<api_builder::Bytes>, APIError<Self::Error>> {
        request.headers_mut().append(AUTHORIZATION, HeaderValue::from_str(self.api_key.as_str())?);
        self.client.rest(request)
    }
}
impl<C> AsyncClient for LuarmorClient<C>
where
    C: AsyncClient,
{
    fn rest_async(
        &self,
        request: http::Request<Vec<u8>>,
    ) -> impl std::future::Future<
        Output = Result<http::Response<api_builder::Bytes>, APIError<Self::Error>>,
    > + Send {
        self.client.rest_async(request)
    }
}
