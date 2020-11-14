//! 注文情報取得APIを実装する。

use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 注文情報取得APIのパス。
const ORDERS_API_PATH: &str = "/v1/orders";

/// 注文情報取得APIを呼び出すときのメソッド。
const ORDERS_API_METHOD: &str = "GET";

/// 注文情報取得APIから返ってくるレスポンスのうち`data`の部分を格納する構造体。
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

/// 注文情報取得APIから返ってくるレスポンスのうち`list`の部分を格納数する構造体。
#[derive(Deserialize)]
pub struct Data {
    /// 注文情報の配列。
    pub list: Vec<Order>,
}

/// 注文情報取得APIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct Orders {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,

    /// レスポンスの`data`の部分。
    pub data: Data,
}

impl RestResponse<Orders> {
    /// 注文情報取得が格納された配列を取得する。
    pub fn orders(&self) -> &Vec<Order> {
        &self.body.data.list
    }
}

/// 注文情報取得APIを呼び出す。
pub async fn get_orders(
    http_client: &impl HttpClient,
    api_key: &str,
    secret_key: &str,
    order_ids: &[&str],
) -> Result<RestResponse<Orders>, Error> {
    let url = format!(
        "{}{}?orderId={}",
        PRIVATE_ENDPOINT,
        ORDERS_API_PATH,
        order_ids.join(",")
    );
    let headers =
        Headers::create_get_headers(&api_key, &secret_key, &ORDERS_API_METHOD, &ORDERS_API_PATH);
    let response = http_client.get(url, &headers).await?;
    parse_from_http_response::<Orders>(&response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_client::tests::InmemClient;
    use chrono::SecondsFormat;

    const SAMPLE_RESPONSE: &str = r#"
    {
        "status": 0,
        "data": {
          "list": [
            {
              "orderId": "223456789",
              "rootOrderId": "223456789",
              "symbol": "BTC_JPY",
              "side": "BUY",
              "orderType": "NORMAL",
              "executionType": "LIMIT",
              "settleType": "OPEN",
              "size": "0.02",
              "executedSize": "0.02",
              "price": "1430001",
              "losscutPrice": "0",
              "status": "EXECUTED",
              "timeInForce": "FAS",
              "timestamp": "2020-10-14T20:18:59.343Z"
            },
            {
              "rootOrderId": 123456789,
              "orderId": 123456789,
              "symbol": "BTC",
              "side": "BUY",
              "orderType": "NORMAL",
              "executionType": "LIMIT",
              "settleType": "OPEN",
              "size": "1",
              "executedSize": "0",
              "price": "900000",
              "losscutPrice": "0",
              "status": "CANCELED",
              "cancelType": "USER",
              "timeInForce": "FAS",
              "timestamp": "2019-03-19T02:15:06.059Z"
            }
          ]
        },
        "responsetime": "2019-03-19T02:15:06.059Z"
    }
          "#;

    #[tokio::test]
    async fn should_return_ok_when_http_client_returns_correct_response() {
        let body = SAMPLE_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = get_orders(&http_client, "apikey", "seckey", &Vec::<&str>::new())
            .await
            .unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2019-03-19T02:15:06.059Z"
        );
        assert_eq!(resp.orders().len(), 2);
    }
}
