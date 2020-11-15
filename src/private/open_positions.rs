//! 建玉一覧APIを実装する。

use crate::dto::{get_pagination_default_value, get_vector_default_value, Pagination, Position};
use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use crate::symbol::Symbol;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 建玉一覧APIのパス。
const OPEN_POSITIONS_API_PATH: &str = "/v1/openPositions";

/// 建玉一覧APIから返ってくるレスポンスのうち`list`の部分を格納数する構造体。
#[derive(Deserialize)]
pub struct Data {
    /// 建玉一覧の取得ページに関する情報。
    #[serde(default = "get_pagination_default_value")]
    pub pagination: Pagination,

    /// 建玉情報の配列。
    #[serde(default = "get_vector_default_value::<Position>")]
    pub list: Vec<Position>,
}

/// 建玉一覧APIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct OpenPositions {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,

    /// レスポンスの`data`の部分。
    pub data: Data,
}

impl RestResponse<OpenPositions> {
    /// 建玉一覧が格納された配列を取得する。
    pub fn open_positions(&self) -> &Vec<Position> {
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

/// 建玉一覧APIを呼び出す。
pub async fn request_open_positions(
    http_client: &impl HttpClient,
    symbol: &Symbol,
    page: i32,
    count: i32,
) -> Result<RestResponse<OpenPositions>, Error> {
    let url = format!(
        "{}{}?symbol={}&page={}&count={}",
        PRIVATE_ENDPOINT,
        OPEN_POSITIONS_API_PATH,
        symbol.to_string(),
        page,
        count,
    );
    let headers = Headers::create_get_headers(&OPEN_POSITIONS_API_PATH)?;
    let response = http_client.get(url, &headers).await?;
    parse_from_http_response::<OpenPositions>(&response)
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
              "positionId": 1234567,
              "symbol": "BTC_JPY",
              "side": "BUY",
              "size": "0.22",
              "orderdSize": "0",
              "price": "876045",
              "lossGain": "14",
              "leverage": "4",
              "losscutPrice": "766540",
              "timestamp": "2019-03-19T02:15:06.094Z"
            }
          ]
        },
        "responsetime": "2019-03-19T02:15:06.095Z"
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
    async fn should_return_ok_when_http_client_returns_correct_response() {
        let body = SAMPLE_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_open_positions(&http_client, &Symbol::Bch, 1, 100)
            .await
            .unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2019-03-19T02:15:06.095Z"
        );
        assert_eq!(resp.open_positions().len(), 1);
        assert_eq!(resp.current_page(), 1);
        assert_eq!(resp.count(), 30);
    }

    #[tokio::test]
    async fn should_not_return_err_when_http_client_returns_empty_response() {
        let body = SAMPLE_EMPTY_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_open_positions(&http_client, &Symbol::BtcJpy, 1, 100)
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
        assert_eq!(resp.open_positions().len(), 0);
        assert_eq!(resp.current_page(), 0);
        assert_eq!(resp.count(), 0);
    }
}
