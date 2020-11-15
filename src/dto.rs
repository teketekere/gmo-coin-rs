//! 各種DTOを定義する。
//! ここで定義したDTOはGMOコインからのレスポンスを構造体にバインディングするのに用いる。

use crate::json::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 注文情報を格納する構造体。
#[derive(Deserialize)]
pub struct Order {
    /// 親注文ID。
    #[serde(deserialize_with = "id_to_str", rename = "rootOrderId")]
    pub root_order_id: String,

    /// 注文ID。
    #[serde(deserialize_with = "id_to_str", rename = "orderId")]
    pub order_id: String,

    /// 銘柄名。
    pub symbol: String,

    /// 売買区分。"BUY" or "SELL"。
    pub side: String,

    /// 取引区分。"NORMAL" or "LOSSCUT"。
    #[serde(rename = "orderType")]
    pub order_type: String,

    /// 注文タイプ。"MARKET", "LIMIT", or "STOP"。
    #[serde(rename = "executionType")]
    pub execution_type: String,

    /// 決済区分。"OPEN" or "CLOSE"。
    #[serde(rename = "settleType")]
    pub settle_type: String,

    /// 発注数量。
    #[serde(deserialize_with = "str_to_f64")]
    pub size: f64,

    /// 約定数量。
    #[serde(deserialize_with = "str_to_f64", rename = "executedSize")]
    pub executed_size: f64,

    /// 注文価格。MARKET注文の場合は"0"。
    #[serde(deserialize_with = "str_to_i64")]
    pub price: i64,

    /// ロスカットレート。現物取引や未設定の場合は"0"。
    #[serde(deserialize_with = "str_to_i64", rename = "losscutPrice")]
    pub losscut_price: i64,

    /// 注文ステータス。"WAITING", "ORDERED", "MODIFYING", "CANCELLING", "CANCELED", "EXECUTED", or "EXPIRED"
    pub status: String,

    /// 取消区分。"NONE", "USER", "POSITION_LOSSCUT", "INSUFFICIENT_BALANCE", "INSUFFICIENT_MARGIN", "ACCOUNT_LOSSCUT", "EXPIRED_FAK", "EXPIRED_FOK", or "EXPIRED_SOK"。
    /// GMOコインではstatusが "CANCELLING", "CANCELED" または "EXPIRED" の場合のみ返ってくるが、実装として難しいので無い場合は"NONE"という値を持つ。
    #[serde(default = "get_string_default_value", rename = "cancelType")]
    pub cancel_type: String,

    /// 執行数量条件。"FAK", "FAS", or "FOK"。 Post-only の場合は "SOK"。
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,

    /// 注文日時。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub timestamp: DateTime<Utc>,
}

/// 取引データ(price, side, size, timestamp)を格納する構造体。
#[derive(Deserialize)]
pub struct Trade {
    /// 約定価格。
    #[serde(deserialize_with = "str_to_i64")]
    pub price: i64,

    /// 売買区分。"BUY" or "SELL"。
    pub side: String,

    /// 約定数量。
    #[serde(deserialize_with = "str_to_f64")]
    pub size: f64,

    /// 約定日時。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub timestamp: DateTime<Utc>,
}

/// 約定情報を格納する構造体。
#[derive(Deserialize)]
pub struct Execution {
    /// 約定ID。
    #[serde(deserialize_with = "id_to_str", rename = "executionId")]
    pub execution_id: String,

    /// 注文ID。
    #[serde(deserialize_with = "id_to_str", rename = "orderId")]
    pub order_id: String,

    /// 銘柄名。
    pub symbol: String,

    /// 売買区分。"BUY" or "SELL"。
    pub side: String,

    /// 決済区分。"OPEN" or "CLOSE"。
    #[serde(rename = "settleType")]
    pub settle_type: String,

    /// 約定数量。
    #[serde(deserialize_with = "str_to_f64")]
    pub size: f64,

    /// 約定レート。
    #[serde(deserialize_with = "str_to_i64")]
    pub price: i64,

    /// 決済損益。
    #[serde(deserialize_with = "str_to_i64", rename = "lossGain")]
    pub loss_gain: i64,

    /// 取引手数料。
    #[serde(deserialize_with = "str_to_i64")]
    pub fee: i64,

    /// 注文日時。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub timestamp: DateTime<Utc>,
}

/// 取得ページに関する情報(current_page, count)を格納する構造体。
#[derive(Deserialize)]
pub struct Pagination {
    /// 取得した取引履歴のページ番号。
    #[serde(deserialize_with = "str_to_i64", rename = "currentPage")]
    pub current_page: i64,

    /// 何件取引履歴を取得したか。
    #[serde(deserialize_with = "str_to_i64")]
    pub count: i64,
}

/// 取得対象ページのデフォルト値。APIを呼び出すとき、数値が指定されない場合はこの値を用いる。
pub const DEFAULT_PAGE: i32 = 1;

/// 1ページ当たりの取得件数のデフォルト値。APIを呼び出すとき、数値が指定されない場合はこの値を用いる。
pub const DEFAULT_COUNT: i32 = 100;

/// GMOコインからのレスポンスが省略されている場合の文字列型フィールドのデフォルト値。
pub fn get_string_default_value() -> String {
    "NONE".to_string()
}

/// GMOコインからのレスポンスが省略されている場合のVec型フィールドのデフォルト値。
pub fn get_vector_default_value<T>() -> Vec<T> {
    Vec::<T>::new()
}

/// GMOコインからのレスポンスが省略されている場合のPagination型フィールドのデフォルト値。
pub fn get_pagination_default_value() -> Pagination {
    Pagination {
        current_page: 0,
        count: 0,
    }
}
