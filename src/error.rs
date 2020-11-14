//! ライブラリ内で異常が発生したときに投げるエラーを定義する。

use crate::response::ErrorResponse;
use thiserror::Error;

/// 異常が発生したときに投げるエラー。
#[derive(Debug, Error)]
pub enum Error {
    #[error("reqwest error")]
    ReqwestError(reqwest::Error),

    #[error("serde_json error")]
    SerdeJsonError(serde_json::Error),

    #[error("url parse error")]
    UrlParseError(url::ParseError),

    #[error("deserialize error")]
    DeserializeError(),

    #[error("api error")]
    APIError(ErrorResponse),

    #[error("env var error")]
    EnvVarError(std::env::VarError),

    #[error("Unknown error")]
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
