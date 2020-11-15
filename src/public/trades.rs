//! 取引履歴APIを実装する。

use crate::dto::{Pagination, Trade};
use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use crate::symbol::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 取引履歴APIのパス。
const TRADES_API_PATH: &str = "/v1/trades";

/// 取引履歴APIから返ってくるレスポンスのうち`data`の部分を格納する構造体。
#[derive(Deserialize)]
pub struct Data {
    /// 取引履歴の配列。
    pub list: Vec<Trade>,

    /// 取引履歴の取得ページに関する情報。
    pub pagination: Pagination,
}

/// 取引履歴APIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct Trades {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,

    /// レスポンスの`data`の部分。
    pub data: Data,
}

impl RestResponse<Trades> {
    /// 取引履歴の配列を取得する。
    pub fn trades(&self) -> &Vec<Trade> {
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

/// 取引履歴APIを呼び出す。引数で取得対象ページと1ページ当たりの取得件数を指定する。
pub async fn request_trades_with_options(
    http_client: &impl HttpClient,
    symbol: &Symbol,
    page: i32,
    count: i32,
) -> Result<RestResponse<Trades>, Error> {
    let url = format!(
        "{}{}?symbol={}&page={}&count={}",
        PUBLIC_ENDPOINT,
        TRADES_API_PATH,
        to_string(&symbol),
        page,
        count,
    );
    let headers = Headers::create_empty_headers();
    let response = http_client.get(url, &headers).await?;
    parse_from_http_response::<Trades>(&response)
}

/// 取引履歴APIを呼び出す。
pub async fn request_trades(
    http_client: &impl HttpClient,
    symbol: &Symbol,
) -> Result<RestResponse<Trades>, Error> {
    request_trades_with_options(http_client, &symbol, 1, 100).await
}

#[cfg(test)]
mod tests {
    use crate::http_client::tests::InmemClient;
    use crate::public::trades::*;
    use crate::symbol::Symbol;
    use chrono::SecondsFormat;

    const TRADES_RESPONSE_SAMPLE: &str = r#"
          {
            "status": 0,
            "data": {
              "pagination": {
                "currentPage": 1,
                "count": 30
              },
              "list": [
                {
                  "price": "750760",
                  "side": "BUY",
                  "size": "0.1",
                  "timestamp": "2018-03-30T12:34:56.789Z"
                },
                {
                    "price": "750760",
                    "side": "BUY",
                    "size": "0.1",
                    "timestamp": "2018-03-30T12:34:56.789Z"
                }
              ]
            },
            "responsetime": "2019-03-28T09:28:07.980Z"
          }
          "#;

    #[tokio::test]
    async fn should_return_ok_when_http_client_returns_correct_response() {
        let body = TRADES_RESPONSE_SAMPLE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_trades(&http_client, &Symbol::Btc).await.unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2019-03-28T09:28:07.980Z"
        );
        assert_eq!(resp.count(), 30);
        assert_eq!(resp.page(), 1);
        let trades = resp.trades();
        assert_eq!(trades.len(), 2);
    }
}
