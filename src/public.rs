//! Public APIを実装する。

pub mod status;
use crate::error::Error;
use crate::http_client::HttpClient;
use crate::public::status::*;
use crate::response::RestResponse;
use async_trait::async_trait;

pub struct PublicAPI<T: HttpClient + std::marker::Sync + std::marker::Send> {
    pub http_client: T,
}

#[async_trait]
pub trait PublicAPITrait {
    async fn status(&self) -> Result<RestResponse<Status>, Error>;
}

#[async_trait]
impl<T: HttpClient + std::marker::Sync + std::marker::Send> PublicAPITrait for PublicAPI<T> {
    async fn status(&self) -> Result<RestResponse<Status>, Error> {
        let response = get_status(&self.http_client).await?;
        Ok(response)
    }
}
