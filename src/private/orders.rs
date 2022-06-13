//! 注文情報取得APIを実装する。

use crate::dto::{get_vector_default_value, Order};
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

/// 注文情報取得APIから返ってくるレスポンスのうち`list`の部分を格納数する構造体。
#[derive(Deserialize)]
pub struct Data {
    /// 注文情報の配列。
    #[serde(default = "get_vector_default_value::<Order>")]
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
pub async fn request_orders(
    http_client: &impl HttpClient,
    order_ids: &[&str],
) -> Result<RestResponse<Orders>, Error> {
    let url = format!(
        "{}{}?orderId={}",
        PRIVATE_ENDPOINT,
        ORDERS_API_PATH,
        order_ids.join(",")
    );
    let headers = Headers::create_get_headers(ORDERS_API_PATH)?;
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

    const SAMPLE_EMPTY_RESPONSE: &str = r#"
    {
        "status": 0,
        "data":{},
        "responsetime":"2020-11-15T06:32:13.747Z"
    }
    "#;

    #[tokio::test]
    async fn test_orders() {
        let body = SAMPLE_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_orders(&http_client, &Vec::<&str>::new())
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

    #[tokio::test]
    async fn test_orders_when_empty_response() {
        let body = SAMPLE_EMPTY_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_orders(&http_client, &Vec::<&str>::new())
            .await
            .unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2020-11-15T06:32:13.747Z"
        );
        assert_eq!(resp.orders().len(), 0);
    }
}
