//! 資産残高APIを実装する。

use crate::dto::get_vector_default_value;
use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 資産残高APIのパス。
const ASSETS_API_PATH: &str = "/v1/account/assets";

/// 資産残高APIから返ってくるレスポンスのうち`data`の部分を格納する構造体。
#[derive(Deserialize)]
pub struct Data {
    /// 残高。
    #[serde(deserialize_with = "str_to_f64")]
    pub amount: f64,

    /// 利用可能金額。残高 - 出金予定額。
    #[serde(deserialize_with = "str_to_f64")]
    pub available: f64,

    /// 円転レート。
    #[serde(deserialize_with = "str_to_f64", rename = "conversionRate")]
    pub conversion_rate: f64,

    /// 銘柄名。
    pub symbol: String,
}

impl Data {
    /// 残高を円で取得する。
    pub fn amount_as_jpy(&self) -> f64 {
        self.amount * self.conversion_rate
    }

    /// 利用可能金額を円で取得する。
    pub fn available_as_jpy(&self) -> f64 {
        self.available * self.conversion_rate
    }
}

/// 資産残高APIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct Assets {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,

    /// レスポンスの`data`の部分。
    #[serde(default = "get_vector_default_value::<Data>")]
    pub data: Vec<Data>,
}

impl RestResponse<Assets> {
    /// 資産残高が格納された配列を取得する。
    pub fn assets(&self) -> &Vec<Data> {
        &self.body.data
    }
}

/// 資産残高APIを呼び出す。
pub async fn request_assets(http_client: &impl HttpClient) -> Result<RestResponse<Assets>, Error> {
    let url = format!("{}{}", PRIVATE_ENDPOINT, ASSETS_API_PATH,);
    let headers = Headers::create_get_headers(ASSETS_API_PATH)?;
    let response = http_client.get(url, &headers).await?;
    parse_from_http_response::<Assets>(&response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_client::tests::InmemClient;
    use chrono::SecondsFormat;

    const SAMPLE_RESPONSE: &str = r#"
    {
        "status": 0,
        "data": [
          {
            "amount": "993982448",
            "available": "993982448",
            "conversionRate": "1",
            "symbol": "JPY"
          },
          {
            "amount": "4.0002",
            "available": "4.0002",
            "conversionRate": "859614",
            "symbol": "BTC"
          }
        ],
        "responsetime": "2019-03-19T02:15:06.055Z"
    }
          "#;

    const SAMPLE_EMPTY_RESPONSE: &str = r#"
    {
        "status": 0,
        "data": [],
        "responsetime":"2020-11-15T06:32:13.747Z"
    }
    "#;

    #[tokio::test]
    async fn test_assets() {
        let body = SAMPLE_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_assets(&http_client).await.unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2019-03-19T02:15:06.055Z"
        );
        assert_eq!(resp.assets().len(), 2);
    }

    #[tokio::test]
    async fn test_assets_when_empty_response() {
        let body = SAMPLE_EMPTY_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_assets(&http_client).await.unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2020-11-15T06:32:13.747Z"
        );
        assert_eq!(resp.assets().len(), 0);
    }
}
