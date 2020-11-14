//! Private APIを実装する。

pub mod active_orders;
pub mod assets;
pub mod margin;
pub mod orders;

use crate::error::Error;
use crate::http_client::HttpClient;
use crate::private::active_orders::{
    get_active_orders, get_active_orders_with_options, ActiveOrders,
};
use crate::private::assets::{get_assets, Assets};
use crate::private::margin::{get_margin, Margin};
use crate::private::orders::{get_orders, Orders};
use crate::response::RestResponse;
use crate::symbol::Symbol;

/// Private API。
pub struct PrivateAPI<T: HttpClient + std::marker::Sync + std::marker::Send> {
    pub http_client: T,
}

impl<T: HttpClient + std::marker::Sync + std::marker::Send> PrivateAPI<T> {
    /// 余力情報APIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `http_client` - Http client
    /// * `api_key` - GMOコインのAPIキー。
    /// * `secret_key` - GMOコインのAPIシークレット。
    ///
    pub async fn margin(
        &self,
        api_key: &str,
        secret_key: &str,
    ) -> Result<RestResponse<Margin>, Error> {
        let response = get_margin(&self.http_client, &api_key, &secret_key).await?;
        Ok(response)
    }

    /// 資産残高APIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `http_client` - Http client
    /// * `api_key` - GMOコインのAPIキー。
    /// * `secret_key` - GMOコインのAPIシークレット。
    ///
    pub async fn assets(
        &self,
        api_key: &str,
        secret_key: &str,
    ) -> Result<RestResponse<Assets>, Error> {
        let response = get_assets(&self.http_client, &api_key, &secret_key).await?;
        Ok(response)
    }

    /// 注文情報取得APIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `http_client` - Http client
    /// * `api_key` - GMOコインのAPIキー。
    /// * `secret_key` - GMOコインのAPIシークレット。
    /// * `order_ids` - 取得する注文の注文ID。最大10件まで指定できる。
    ///
    pub async fn orders(
        &self,
        api_key: &str,
        secret_key: &str,
        order_ids: &[&str],
    ) -> Result<RestResponse<Orders>, Error> {
        let response = get_orders(&self.http_client, &api_key, &secret_key, &order_ids).await?;
        Ok(response)
    }

    /// 有効注文一覧APIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `http_client` - Http client
    /// * `api_key` - GMOコインのAPIキー。
    /// * `secret_key` - GMOコインのAPIシークレット。
    /// * `symbol` - 有効注文を取得する銘柄。
    ///
    pub async fn active_orders(
        &self,
        api_key: &str,
        secret_key: &str,
        symbol: &Symbol,
    ) -> Result<RestResponse<ActiveOrders>, Error> {
        let response = get_active_orders(&self.http_client, &api_key, &secret_key, &symbol).await?;
        Ok(response)
    }

    /// 有効注文一覧APIをオプション引数付きで呼び出す。
    ///
    /// # Arguments
    ///
    /// * `http_client` - Http client
    /// * `api_key` - GMOコインのAPIキー。
    /// * `secret_key` - GMOコインのAPIシークレット。
    /// * `symbol` - 有効注文を取得する銘柄。
    /// * `page` - 取得対象ページ。
    /// * `count` - 1ページ当たりの取得件数。
    ///
    pub async fn active_orders_with_options(
        &self,
        api_key: &str,
        secret_key: &str,
        symbol: &Symbol,
        page: i32,
        count: i32,
    ) -> Result<RestResponse<ActiveOrders>, Error> {
        let response = get_active_orders_with_options(
            &self.http_client,
            &api_key,
            &secret_key,
            &symbol,
            page,
            count,
        )
        .await?;
        Ok(response)
    }
}
