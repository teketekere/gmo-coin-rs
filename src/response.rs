//! APIのレスポンスを定義する。

use serde::Deserialize;

/// HTTPクライアントから返ってくるそのままのレスポンスを持つ構造体。
pub struct RawResponse {
    pub http_status_code: u16,
    pub body_text: String,
}

/// Public API, Private APIの結果として返す構造体。
pub struct RestResponse<T> {
    /// HTTPステータスコード。
    pub http_status_code: u16,

    /// GMOコインからのレスポンスのボディ部分。
    pub body: T,
}

/// APIの呼び出しが不正なときにGMOコインから返ってくるレスポンスのうちメッセージの部分。
#[derive(Deserialize, Debug)]
pub struct ErrorMessage {
    pub message_code: String,
    pub message_string: String,
}

/// APIの呼び出しが不正なときにGMOコインから返ってくるレスポンス。
#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: i16,
    pub messages: Vec<ErrorMessage>,
}
