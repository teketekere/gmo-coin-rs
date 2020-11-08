/// Public APIのエンドポイント。
pub const PUBLIC_ENDPOINT: &str = "https://api.coin.z.com/public";

/// Public WebSocket APIのエンドポイント。
pub const PUBLIC_WEB_SOCKET_ENDPOINT: &str = "wss://api.coin.z.com/ws/public";

/// Private APIのエンドポイント。
pub const PRIVATE_ENDPOINT: &str = "https://api.coin.z.com/private";

/// Public WebSocket APIのエンドポイント。
pub const PRIVATE_WEB_SOCKET_ENDPOINT: &str = "wss://api.coin.z.com/ws/private";

/// EndpointとPathからAPIのURLを作る。
///
/// # Arguments
///
/// * `endpoint` - APIのエンドポイントを表す文字列。
///
/// * `path` - 各APIのPathを表す文字列。
///
pub fn to_url(endpoint: &str, path: &str) -> String {
    format!("{}{}", endpoint, path)
}
