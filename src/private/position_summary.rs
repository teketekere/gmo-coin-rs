//! 建玉サマリーAPIを実装する。

use crate::dto::{get_vector_default_value, Summary};
use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use crate::symbol::Symbol;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 建玉サマリーAPIのパス。
const POSITION_SUMMARY_API_PATH: &str = "/v1/positionSummary";

/// 建玉サマリーAPIから返ってくるレスポンスのうち`list`の部分を格納数する構造体。
#[derive(Deserialize)]
pub struct Data {
    /// 建玉情報の配列。
    #[serde(default = "get_vector_default_value::<Summary>")]
    pub list: Vec<Summary>,
}

/// 建玉サマリーAPIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct PositionSummary {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,

    /// レスポンスの`data`の部分。
    pub data: Data,
}

impl RestResponse<PositionSummary> {
    /// 建玉サマリーが格納された配列を取得する。
    pub fn position_summaries(&self) -> &Vec<Summary> {
        &self.body.data.list
    }
}

/// 建玉サマリーAPIを呼び出す。
pub async fn request_position_summary(
    http_client: &impl HttpClient,
    symbol: &Symbol,
) -> Result<RestResponse<PositionSummary>, Error> {
    let url = format!(
        "{}{}?symbol={}",
        PRIVATE_ENDPOINT,
        POSITION_SUMMARY_API_PATH,
        symbol.to_string(),
    );
    let headers = Headers::create_get_headers(POSITION_SUMMARY_API_PATH)?;
    let response = http_client.get(url, &headers).await?;
    parse_from_http_response::<PositionSummary>(&response)
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
          "list": [
            {
              "averagePositionRate": "715656",
              "positionLossGain": "250675",
              "side": "BUY",
              "sumOrderQuantity": "2",
              "sumPositionQuantity": "11.6999",
              "symbol": "BTC_JPY"
            }
          ]
        },
        "responsetime": "2019-03-19T02:15:06.102Z"
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
    async fn test_position_summary() {
        let body = SAMPLE_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_position_summary(&http_client, &Symbol::BtcJpy)
            .await
            .unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2019-03-19T02:15:06.102Z"
        );
        assert_eq!(resp.position_summaries().len(), 1);
    }

    #[tokio::test]
    async fn test_position_summary_when_empty_response() {
        let body = SAMPLE_EMPTY_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_position_summary(&http_client, &Symbol::BtcJpy)
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
        assert_eq!(resp.position_summaries().len(), 0);
    }
}
