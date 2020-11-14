//! 各種DTOを定義する。
//! ここで定義したDTOはGMOコインからのレスポンスを構造体にバインディングするのに用いる。

use crate::json::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 注文情報を格納する構造体。
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Order {
    /// 親注文ID。
    #[serde(deserialize_with = "orderid_to_str")]
    pub rootOrderId: String,

    /// 注文ID。
    #[serde(deserialize_with = "orderid_to_str")]
    pub orderId: String,

    /// 銘柄名。
    pub symbol: String,

    /// 売買区分。"BUY" or "SELL"。
    pub side: String,

    /// 取引区分。"NORMAL" or "LOSSCUT"。
    pub orderType: String,

    /// 注文タイプ。"MARKET", "LIMIT", or "STOP"。
    pub executionType: String,

    /// 決済区分。"OPEN" or "CLOSE"。
    pub settleType: String,

    /// 発注数量。
    #[serde(deserialize_with = "str_to_f64")]
    pub size: f64,

    /// 約定数量。
    #[serde(deserialize_with = "str_to_f64")]
    pub executedSize: f64,

    /// 注文価格。MARKET注文の場合は"0"。
    #[serde(deserialize_with = "str_to_i64")]
    pub price: i64,

    /// ロスカットレート。現物取引や未設定の場合は"0"。
    #[serde(deserialize_with = "str_to_i64")]
    pub losscutPrice: i64,

    /// 注文ステータス。"WAITING", "ORDERED", "MODIFYING", "CANCELLING", "CANCELED", "EXECUTED", or "EXPIRED"
    pub status: String,

    /// 取消区分。"NONE", "USER", "POSITION_LOSSCUT", "INSUFFICIENT_BALANCE", "INSUFFICIENT_MARGIN", "ACCOUNT_LOSSCUT", "EXPIRED_FAK", "EXPIRED_FOK", or "EXPIRED_SOK"。
    /// GMOコインではstatusが "CANCELLING", "CANCELED" または "EXPIRED" の場合のみ返ってくるが、実装として難しいので無い場合は"NONE"という値を持つ。
    #[serde(default = "get_string_default_value")]
    pub cancelType: String,

    /// 執行数量条件。"FAK", "FAS", or "FOK"。 Post-only の場合は "SOK"。
    pub timeInForce: String,

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

/// 取得ページに関する情報(current_page, count)を格納する構造体。
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Pagination {
    /// 取得した取引履歴のページ番号。
    #[serde(deserialize_with = "str_to_i64")]
    pub currentPage: i64,

    /// 何件取引履歴を取得したか。
    #[serde(deserialize_with = "str_to_i64")]
    pub count: i64,
}
