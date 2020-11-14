//! Private APIを実装する。

pub mod assets;
pub mod margin;

use crate::error::Error;
use crate::http_client::HttpClient;
use crate::private::assets::{get_assets, Assets};
use crate::private::margin::{get_margin, Margin};
use crate::response::RestResponse;

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
}
