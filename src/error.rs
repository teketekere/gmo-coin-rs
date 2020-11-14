//! ライブラリ内で異常が発生したときに投げるエラーを定義する。

use crate::response::ErrorResponse;
use thiserror::Error;

/// 異常が発生したときに投げるエラー。
#[derive(Debug, Error)]
pub enum Error {
    #[error("reqwest使ってるとこで何かあったわ")]
    ReqwestError(reqwest::Error),

    #[error("HTTPレスポンスのボディをserde_jsonで構造体にバインディングしてるとこで何かあったわ")]
    SerdeJsonError(serde_json::Error),

    #[error("URLうまく作れんかったわ")]
    UrlParseError(url::ParseError),

    #[error("何なんかよー分からんわ")]
    DeserializeError(),

    #[error("GMOコインのAPIからエラーが返って来とるで")]
    APIError(ErrorResponse),

    #[error("環境変数読み取れなんだわ")]
    EnvVarError(std::env::VarError),

    #[error("何もよー分かりまへん")]
    UnknownError,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJsonError(e)
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::UrlParseError(e)
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::EnvVarError(e)
    }
}
