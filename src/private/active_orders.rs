//! 有効注文一覧APIを実装する。

use crate::dto::{get_pagination_default_value, get_vector_default_value, Order, Pagination};
use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use crate::symbol::Symbol;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 有効注文一覧APIのパス。
const ACTIVE_ORDERS_API_PATH: &str = "/v1/activeOrders";

/// 有効注文一覧APIから返ってくるレスポンスのうち`list`の部分を格納数する構造体。
#[derive(Deserialize)]
pub struct Data {
    /// 有効注文の取得ページに関する情報。
    #[serde(default = "get_pagination_default_value")]
    pub pagination: Pagination,

    /// 注文情報の配列。
    #[serde(default = "get_vector_default_value::<Order>")]
    pub list: Vec<Order>,
}

/// 有効注文一覧APIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct ActiveOrders {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,

    /// レスポンスの`data`の部分。
    pub data: Data,
}

impl RestResponse<ActiveOrders> {
    /// 有効注文一覧が格納された配列を取得する。
    pub fn active_orders(&self) -> &Vec<Order> {
        &self.body.data.list
    }

    /// 取得対象ページ。
    pub fn current_page(&self) -> i64 {
        self.body.data.pagination.current_page
    }

    /// 1ページ当たりの取得件数。
    pub fn count(&self) -> i64 {
        self.body.data.pagination.count
    }
}

/// 有効注文一覧APIを呼び出す。オプショナルなパラメーターを明示的に指定する場合こちらを呼ぶ。
pub async fn request_active_orders(
    http_client: &impl HttpClient,
    symbol: &Symbol,
    page: i32,
    count: i32,
) -> Result<RestResponse<ActiveOrders>, Error> {
    let url = format!(
        "{}{}?symbol={}&page={}&count={}",
        PRIVATE_ENDPOINT,
        ACTIVE_ORDERS_API_PATH,
        symbol.to_string(),
        page,
        count,
    );
    let headers = Headers::create_get_headers(&ACTIVE_ORDERS_API_PATH)?;
    let response = http_client.get(url, &headers).await?;
    parse_from_http_response::<ActiveOrders>(&response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_client::tests::InmemClient;
    use crate::symbol::Symbol;
    use chrono::SecondsFormat;

    const SAMPLE_RESPONSE: &str = r#"
    {
        "status": 0,
        "data": {
          "pagination": {
            "currentPage": 1,
            "count": 30
          },
          "list": [
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
              "price": "840000",
              "losscutPrice": "0",
              "status": "ORDERED",
              "timeInForce": "FAS",
              "timestamp": "2019-03-19T01:07:24.217Z"
            }
          ]
        },
        "responsetime": "2019-03-19T01:07:24.217Z"
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
    async fn test_active_orders() {
        let body = SAMPLE_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_active_orders(&http_client, &Symbol::Bch, 1, 100)
            .await
            .unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2019-03-19T01:07:24.217Z"
        );
        assert_eq!(resp.active_orders().len(), 1);
        assert_eq!(resp.current_page(), 1);
        assert_eq!(resp.count(), 30);
    }

    #[tokio::test]
    async fn test_active_orders_when_empty_response() {
        let body = SAMPLE_EMPTY_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_active_orders(&http_client, &Symbol::BtcJpy, 1, 100)
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
        assert_eq!(resp.active_orders().len(), 0);
        assert_eq!(resp.current_page(), 0);
        assert_eq!(resp.count(), 0);
    }
}
