use std::ops::Deref;

use api_builder::{
    AsyncClient, AsyncQuery, Bytes, Client, Endpoint, Query, error::APIError, impl_query,
    impl_query_async,
};
use http::Response;
use serde::de::DeserializeOwned;

use crate::models::{LuarmorMessage, LuarmorResponse};

pub struct Luarmor<E>(pub E);
impl<E> Deref for Luarmor<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<E, T, C> Query<T, C> for Luarmor<E>
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client<Error = LuarmorMessage>,
{
    impl_query!("request");
    impl_query!("send");
    impl_query!("query");

    fn finalise(&self, response: Response<Bytes>) -> Result<T, APIError<C::Error>> {
        if response.body().is_empty() && !response.status().is_success() && !self.0.ignore_errors()
        {
            return Err(response)?;
        }

        let lrm_response = serde_json::from_slice::<LuarmorResponse<T>>(response.body())?;
        if !lrm_response.success {
            Err(lrm_response.message)?
        } else if let Some(data) = lrm_response.data {
            Ok(data)
        } else {
            Err(response)?
        }
    }
}
impl<E, T, C> AsyncQuery<T, C> for Luarmor<E>
where
    E: Endpoint + Sync,
    T: DeserializeOwned,
    C: AsyncClient<Error = LuarmorMessage> + Sync,
{
    impl_query_async!("request");
    impl_query_async!("send");
    impl_query_async!("query");

    async fn finalise_async(&self, response: Response<Bytes>) -> Result<T, APIError<C::Error>> {
        if response.body().is_empty() && !response.status().is_success() && !self.0.ignore_errors()
        {
            return Err(response)?;
        }

        let lrm_response = serde_json::from_slice::<LuarmorResponse<T>>(response.body())?;
        if !lrm_response.success {
            Err(lrm_response.message)?
        } else if let Some(data) = lrm_response.data {
            Ok(data)
        } else {
            Err(response)?
        }
    }
}
