//! Public APIを実装する。

pub mod status;
pub mod ticker;

use crate::error::Error;
use crate::http_client::HttpClient;
use crate::public::status::{get_status, Status};
use crate::public::ticker::{get_ticker, Ticker};
use crate::response::RestResponse;
use async_trait::async_trait;

pub struct PublicAPI<T: HttpClient + std::marker::Sync + std::marker::Send> {
    pub http_client: T,
}

#[async_trait]
pub trait PublicAPITrait {
    async fn status(&self) -> Result<RestResponse<Status>, Error>;
    async fn ticker(&self, symbol: &str) -> Result<RestResponse<Ticker>, Error>;
}

#[async_trait]
impl<T: HttpClient + std::marker::Sync + std::marker::Send> PublicAPITrait for PublicAPI<T> {
    async fn status(&self) -> Result<RestResponse<Status>, Error> {
        let response = get_status(&self.http_client).await?;
        Ok(response)
    }

    async fn ticker(&self, symbol: &str) -> Result<RestResponse<Ticker>, Error> {
        let response = get_ticker(&self.http_client, &symbol).await?;
        Ok(response)
    }
}
