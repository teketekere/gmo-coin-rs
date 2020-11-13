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
    /// 取引所ステータスAPIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `http_client` - Http client
    ///
    pub async fn status(&self) -> Result<RestResponse<Status>, Error> {
        let response = get_status(&self.http_client).await?;
        Ok(response)
    }

    /// 最新レートAPIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `http_client` - Http client
    /// * `symbol` - 銘柄
    ///
    pub async fn ticker(&self, symbol: &str) -> Result<RestResponse<Ticker>, Error> {
        let response = get_ticker(&self.http_client, &symbol).await?;
        Ok(response)
    }

    /// 板情報APIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `http_client` - Http client
    /// * `symbol` - 銘柄
    ///
    pub async fn orderbooks(&self, symbol: &str) -> Result<RestResponse<Orderbooks>, Error> {
        let response = get_orderbooks(&self.http_client, &symbol).await?;
        Ok(response)
    }

    /// 取引履歴APIを呼び出す。取得ページは1、取得件数は100(最大値)となる。
    ///
    /// # Arguments
    ///
    /// * `http_client` - Http client
    /// * `symbol` - 銘柄
    ///
    pub async fn trades(&self, symbol: &str) -> Result<RestResponse<Trades>, Error> {
        let response = get_trades(&self.http_client, &symbol).await?;
        Ok(response)
    }

    /// 取引履歴APIを呼び出す。引数で取得対象ページと1ページ当たりの取得件数を指定する。
    ///
    /// # Arguments
    ///
    /// * `http_client` - Http client
    /// * `symbol` - 銘柄
    /// * `page` - 取得対象ページ
    /// * `count` - 取得件数
    ///
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
