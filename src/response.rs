//! APIのレスポンスを定義する。

/// HTTPクライアントから返ってくるそのままのレスポンスを持つ構造体。
pub struct RawResponse {
    pub http_status_code: u16,
    pub body_text: String,
}

/// Public API, Private APIの結果として返す構造体。
pub struct RestResponse<T> {
    pub http_status_code: u16,
    pub body: T,
}
