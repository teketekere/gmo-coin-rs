//! Public APIを実装する。

pub mod orderbooks;
pub mod status;
pub mod ticker;
pub mod trades;

use crate::error::Error;
use crate::http_client::HttpClient;
use crate::public::orderbooks::{get_orderbooks, Orderbooks};
use crate::public::status::{get_status, Status};
use crate::public::ticker::{get_ticker, Ticker};
use crate::public::trades::{get_trades, get_trades_with_options, Trades};
use crate::response::RestResponse;

pub struct PublicAPI<T: HttpClient + std::marker::Sync + std::marker::Send> {
    pub http_client: T,
}

impl<T: HttpClient + std::marker::Sync + std::marker::Send> PublicAPI<T> {
    pub async fn status(&self) -> Result<RestResponse<Status>, Error> {
        let response = get_status(&self.http_client).await?;
        Ok(response)
    }

    pub async fn ticker(&self, symbol: &str) -> Result<RestResponse<Ticker>, Error> {
        let response = get_ticker(&self.http_client, &symbol).await?;
        Ok(response)
    }

    pub async fn orderbooks(&self, symbol: &str) -> Result<RestResponse<Orderbooks>, Error> {
        let response = get_orderbooks(&self.http_client, &symbol).await?;
        Ok(response)
    }

    pub async fn trades(&self, symbol: &str) -> Result<RestResponse<Trades>, Error> {
        let response = get_trades(&self.http_client, &symbol).await?;
        Ok(response)
    }

    pub async fn trades_with_options(
        &self,
        symbol: &str,
        page: i32,
        count: i32,
    ) -> Result<RestResponse<Trades>, Error> {
        let response = get_trades_with_options(&self.http_client, &symbol, page, count).await?;
        Ok(response)
    }
}
