//! ライブラリ内でエラーが発生したときにResultのNGの方で返す値。
//!
//! 正直エラーハンドリング周りが全然わかっていない。
//! サードパーティライブラリ(reqwestとか)でエラーがでたときどうすればいいんだ？

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("reqwest error")]
    ReqwestError,

    #[error("serde_json error")]
    SerdeJsonError(serde_json::error::Category),

    #[error("url parse error")]
    UrlParseError(url::ParseError),

    #[error("Unknown error")]
    UnknownError,
}

impl From<reqwest::Error> for Error {
    fn from(_e: reqwest::Error) -> Self {
        Error::ReqwestError
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJsonError(e.classify())
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::UrlParseError(e)
    }
}
