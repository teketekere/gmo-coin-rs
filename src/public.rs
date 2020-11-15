//! Public APIを実装する。

pub mod orderbooks;
pub mod status;
pub mod ticker;
pub mod trades;

use crate::error::Error;
use crate::http_client::HttpClient;
use crate::public::orderbooks::{request_orderbooks, Orderbooks};
use crate::public::status::{request_status, Status};
use crate::public::ticker::{request_ticker, Ticker};
use crate::public::trades::{request_trades, request_trades_with_options, Trades};
use crate::response::RestResponse;
use crate::symbol::Symbol;

pub struct PublicAPI<T: HttpClient + std::marker::Sync + std::marker::Send> {
    pub http_client: T,
}

impl<T: HttpClient + std::marker::Sync + std::marker::Send> PublicAPI<T> {
    /// 取引所ステータスAPIを呼び出す。
    ///
    /// # Arguments
    ///
    ///
    pub async fn status(&self) -> Result<RestResponse<Status>, Error> {
        let response = request_status(&self.http_client).await?;
        Ok(response)
    }

    /// 最新レートAPIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄
    ///
    pub async fn ticker(&self, symbol: &Symbol) -> Result<RestResponse<Ticker>, Error> {
        let response = request_ticker(&self.http_client, &symbol).await?;
        Ok(response)
    }

    /// 板情報APIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄
    ///
    pub async fn orderbooks(&self, symbol: &Symbol) -> Result<RestResponse<Orderbooks>, Error> {
        let response = request_orderbooks(&self.http_client, &symbol).await?;
        Ok(response)
    }

    /// 取引履歴APIを呼び出す。取得ページは1、取得件数は100(最大値)となる。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄
    ///
    pub async fn trades(&self, symbol: &Symbol) -> Result<RestResponse<Trades>, Error> {
        let response = request_trades(&self.http_client, &symbol).await?;
        Ok(response)
    }

    /// 取引履歴APIを呼び出す。引数で取得対象ページと1ページ当たりの取得件数を指定する。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄
    /// * `page` - 取得対象ページ
    /// * `count` - 取得件数
    ///
    pub async fn trades_with_options(
        &self,
        symbol: &Symbol,
        page: i32,
        count: i32,
    ) -> Result<RestResponse<Trades>, Error> {
        let response = request_trades_with_options(&self.http_client, &symbol, page, count).await?;
        Ok(response)
    }
}
