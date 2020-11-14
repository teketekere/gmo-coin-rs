//! 有効注文一覧APIを実装する。

use crate::dto::{Order, Pagination};
use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use crate::symbol::{to_string, Symbol};
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 有効注文一覧APIのパス。
const ACTIVE_ORDERS_API_PATH: &str = "/v1/activeOrders";

/// 有効注文一覧APIを呼び出すときのメソッド。
const ACTIVE_ORDERS_API_METHOD: &str = "GET";

/// 有効注文一覧APIから返ってくるレスポンスのうち`list`の部分を格納数する構造体。
#[derive(Deserialize)]
pub struct Data {
    /// 有効注文の取得ページに関する情報。
    pub pagination: Pagination,

    /// 注文情報の配列。
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
    pub fn page(&self) -> i64 {
        self.body.data.pagination.currentPage
    }

    /// 1ページ当たりの取得件数。
    pub fn count(&self) -> i64 {
        self.body.data.pagination.count
    }
}

/// 有効注文一覧APIを呼び出す。オプショナルなパラメーターを明示的に指定する場合こちらを呼ぶ。
pub async fn get_active_orders_with_options(
    http_client: &impl HttpClient,
    api_key: &str,
    secret_key: &str,
    symbol: &Symbol,
    page: i32,
    count: i32,
) -> Result<RestResponse<ActiveOrders>, Error> {
    let url = format!(
        "{}{}?symbol={}&page={}&count={}",
        PRIVATE_ENDPOINT,
        ACTIVE_ORDERS_API_PATH,
        to_string(&symbol),
        page,
        count,
    );
    let headers = Headers::create_get_headers(
        &api_key,
        &secret_key,
        &ACTIVE_ORDERS_API_METHOD,
        &ACTIVE_ORDERS_API_PATH,
    );
    let response = http_client.get(url, &headers).await?;
    parse_from_http_response::<ActiveOrders>(&response)
}

/// 有効注文一覧APIを呼び出す。
pub async fn get_active_orders(
    http_client: &impl HttpClient,
    api_key: &str,
    secret_key: &str,
    symbol: &Symbol,
) -> Result<RestResponse<ActiveOrders>, Error> {
    get_active_orders_with_options(http_client, &api_key, &secret_key, &symbol, 1, 100).await
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

    #[tokio::test]
    async fn should_return_ok_when_http_client_returns_correct_response() {
        let body = SAMPLE_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = get_active_orders(&http_client, "apikey", "seckey", &Symbol::Bch)
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
        assert_eq!(resp.page(), 1);
        assert_eq!(resp.count(), 30);
    }
}
