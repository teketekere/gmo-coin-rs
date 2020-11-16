//! ライブラリ内で異常が発生したときに投げるエラーを定義する。

use crate::response::ErrorResponse;
use thiserror::Error;

/// 異常が発生したときに投げるエラー。
#[derive(Debug, Error)]
pub enum Error {
    #[error("reqwestがエラーを投げた")]
    ReqwestError(reqwest::Error),

    #[error("HTTPレスポンスのボディをserde_jsonで構造体にバインディングしてるとこで異常があった")]
    SerdeJsonError(serde_json::Error),

    #[error("URLを作るとこで異常が起きた")]
    UrlParseError(url::ParseError),

    #[error("空のレスポンスが返ってこないはずの箇所で空のレスポンスが返ってきた")]
    EmptyResponseError(),

    #[error("GMOコインのAPIからエラーレスポンスが返ってきた")]
    APIError(ErrorResponse),

    #[error("環境変数を読み取れなかった")]
    EnvVarError(std::env::VarError),

    #[error("IDを文字列から数値に変換できなかった")]
    IdToNumberError(String),

    #[error("デバッグ用")]
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
