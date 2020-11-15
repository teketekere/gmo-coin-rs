//! HTTPクライアントを定義する。

use crate::error::*;
use crate::headers::Headers;
use crate::response::*;
use async_trait::async_trait;
use serde_json::Value;

/// HTTPクライアントのtrait。GET, POSTとか。
#[async_trait]
pub trait HttpClient {
    async fn get(&self, url: String, headers: &Headers) -> Result<RawResponse, Error>;
    async fn post(
        &self,
        url: String,
        headers: &Headers,
        parameters: &Value,
    ) -> Result<RawResponse, Error>;
}

/// ネットワークアクセス時に用いるHttpクライアント。
/// Rustではreqwestがデファクトっぽいのでネットワークアクセスするときはreqwestを使う。
pub struct Reqwest;

#[async_trait]
impl HttpClient for Reqwest {
    async fn get(&self, url: String, headers: &Headers) -> Result<RawResponse, Error> {
        let mut request_builder = reqwest::Client::new().get(reqwest::Url::parse(&url)?);
        for (key, value) in headers {
            request_builder = request_builder.header(key, value);
        }

        let response = request_builder.send().await?;
        Ok(RawResponse {
            http_status_code: (response.status().as_u16()),
            body_text: (response.text().await?),
        })
    }

    async fn post(
        &self,
        url: String,
        headers: &Headers,
        parameters: &Value,
    ) -> Result<RawResponse, Error> {
        let mut request_builder = reqwest::Client::new().post(reqwest::Url::parse(&url)?);
        for (key, value) in headers {
            request_builder = request_builder.header(key, value);
        }

        let request_builder = request_builder.json(&parameters);
        let response = request_builder.send().await?;
        Ok(RawResponse {
            http_status_code: (response.status().as_u16()),
            body_text: (response.text().await?),
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    /// 単体テスト用のHttpクライアント。
    pub struct InmemClient {
        pub http_status_code: u16,
        pub body_text: String,
        pub return_error: bool,
    }

    impl InmemClient {
        async fn return_result(&self) -> Result<RawResponse, Error> {
            if self.return_error {
                return Err(Error::UnknownError {});
            }

            Ok(RawResponse {
                http_status_code: (self.http_status_code),
                body_text: (self.body_text.clone()),
            })
        }
    }

    #[async_trait]
    impl HttpClient for InmemClient {
        async fn get(&self, _url: String, _headers: &Headers) -> Result<RawResponse, Error> {
            self.return_result().await
        }

        async fn post(
            &self,
            _url: String,
            _headers: &Headers,
            _parameters: &Value,
        ) -> Result<RawResponse, Error> {
            self.return_result().await
        }
    }
}
