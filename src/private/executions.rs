//! 約定情報取得APIを実装する。

use crate::dto::{get_vector_default_value, Execution};
use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 約定情報取得APIのパス。
const EXECUTIONS_API_PATH: &str = "/v1/executions";

/// 約定情報取得APIから返ってくるレスポンスのうち`list`の部分を格納数する構造体。
#[derive(Deserialize)]
pub struct Data {
    /// 約定情報の配列。
    #[serde(default = "get_vector_default_value::<Execution>")]
    pub list: Vec<Execution>,
}

/// 約定情報取得APIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct Executions {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,

    /// レスポンスの`data`の部分。
    pub data: Data,
}

impl RestResponse<Executions> {
    /// 約定情報取得が格納された配列を取得する。
    pub fn executions(&self) -> &Vec<Execution> {
        &self.body.data.list
    }
}

/// 約定情報取得APIを呼び出す。注文IDを指定してAPIを呼び出す。
pub async fn request_executions_with_order_id(
    http_client: &impl HttpClient,
    order_id: &str,
) -> Result<RestResponse<Executions>, Error> {
    let url = format!(
        "{}{}?orderId={}",
        PRIVATE_ENDPOINT, EXECUTIONS_API_PATH, order_id,
    );
    let headers = Headers::create_get_headers(&EXECUTIONS_API_PATH)?;
    let response = http_client.get(url, &headers).await?;
    parse_from_http_response::<Executions>(&response)
}

/// 約定情報取得APIを呼び出す。約定IDを指定してAPIを呼び出す。
pub async fn request_executions_with_execution_id(
    http_client: &impl HttpClient,
    execution_id: &str,
) -> Result<RestResponse<Executions>, Error> {
    let url = format!(
        "{}{}?executionId={}",
        PRIVATE_ENDPOINT, EXECUTIONS_API_PATH, execution_id,
    );
    let headers = Headers::create_get_headers(&EXECUTIONS_API_PATH)?;
    let response = http_client.get(url, &headers).await?;
    parse_from_http_response::<Executions>(&response)
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
              "executionId": 72123911,
              "orderId": 123456789,
              "symbol": "BTC",
              "side": "BUY",
              "settleType": "OPEN",
              "size": "0.7361",
              "price": "877404",
              "lossGain": "0",
              "fee": "323",
              "timestamp": "2019-03-19T02:15:06.081Z"
            }
          ]
        },
        "responsetime": "2019-03-19T02:15:06.081Z"
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
        let resp = request_executions_with_execution_id(&http_client, "execid")
            .await
            .unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2019-03-19T02:15:06.081Z"
        );
        assert_eq!(resp.executions().len(), 1);
    }

    #[tokio::test]
    async fn should_not_return_err_when_http_client_returns_empty_response() {
        let body = SAMPLE_EMPTY_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_executions_with_order_id(&http_client, "orderid")
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
        assert_eq!(resp.executions().len(), 0);
    }
}
