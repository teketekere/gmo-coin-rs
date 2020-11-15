//! 最新の約定一覧APIを実装する。

use crate::dto::{Execution, Pagination};
use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use crate::symbol::{to_string, Symbol};
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 最新の約定一覧APIのパス。
const LATEST_EXECUTIONS_API_PATH: &str = "/v1/latestExecutions";

/// 最新の約定一覧APIを呼び出すときのメソッド。
const LATEST_EXECUTIONS_API_METHOD: &str = "GET";

/// 最新の約定一覧APIから返ってくるレスポンスのうち`list`の部分を格納数する構造体。
#[derive(Deserialize)]
pub struct Data {
    /// 最新の約定一覧の取得ページに関する情報。
    pub pagination: Pagination,

    /// 約定情報の配列。
    pub list: Vec<Execution>,
}

/// 最新の約定一覧APIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct LatestExecutions {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,

    /// レスポンスの`data`の部分。
    pub data: Data,
}

impl RestResponse<LatestExecutions> {
    /// 最新の約定一覧が格納された配列を取得する。
    pub fn latest_executions(&self) -> &Vec<Execution> {
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

/// 最新の約定一覧APIを呼び出す。
pub async fn request_latest_executions(
    http_client: &impl HttpClient,
    api_key: &str,
    secret_key: &str,
    symbol: &Symbol,
    page: i32,
    count: i32,
) -> Result<RestResponse<LatestExecutions>, Error> {
    let url = format!(
        "{}{}?symbol={}&page={}&count={}",
        PRIVATE_ENDPOINT,
        LATEST_EXECUTIONS_API_PATH,
        to_string(&symbol),
        page,
        count
    );
    let headers = Headers::create_get_headers(
        &api_key,
        &secret_key,
        &LATEST_EXECUTIONS_API_METHOD,
        &LATEST_EXECUTIONS_API_PATH,
    );
    let response = http_client.get(url, &headers).await?;
    parse_from_http_response::<LatestExecutions>(&response)
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
              "executionId": 72123911,
              "orderId": 123456789,
              "symbol": "BTC",
              "side": "BUY",
              "settleType": "OPEN",
              "size": "0.7361",
              "price": "877404",
              "lossGain": "0",
              "fee": "323",
              "timestamp": "2019-03-19T02:15:06.086Z"
            }
          ]
        },
        "responsetime": "2019-03-19T02:15:06.086Z"
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
        let resp =
            request_latest_executions(&http_client, "apikey", "seckey", &Symbol::Btc, 1, 100)
                .await
                .unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2019-03-19T02:15:06.086Z"
        );
        assert_eq!(resp.latest_executions().len(), 1);
        assert_eq!(resp.current_page(), 1);
        assert_eq!(resp.count(), 30);
    }
}
