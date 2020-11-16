//! 注文の複数キャンセルAPIを実装する。

use crate::dto::{get_vector_default_value, CancelFailedOrder};
use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{json, Value};

/// 注文の複数キャンセルAPIのパス。
const CANCEL_ORDERS_API_PATH: &str = "/v1/cancelOrders";

/// 資産残高APIから返ってくるレスポンスのうち`data`の部分を格納する構造体。
#[derive(Deserialize)]
pub struct Data {
    /// キャンセルに失敗した注文Idの配列。
    #[serde(default = "get_vector_default_value")]
    pub failed: Vec<CancelFailedOrder>,

    /// キャンセルに成功した注文Idの配列。
    #[serde(
        deserialize_with = "ids_to_strvec",
        default = "get_vector_default_value"
    )]
    pub success: Vec<String>,
}

/// 注文の複数キャンセルAPIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct CancelOrders {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,

    /// レスポンスの`data`の部分。
    pub data: Data,
}

impl RestResponse<CancelOrders> {
    /// キャンセルに失敗した注文が格納された配列を取得する。
    pub fn failed(&self) -> &Vec<CancelFailedOrder> {
        &self.body.data.failed
    }

    /// キャンセルに成功した注文の注文Idが格納された配列を取得する。
    pub fn success(&self) -> &Vec<String> {
        &self.body.data.success
    }
}

fn build_parameters(order_ids: &[&str]) -> Result<Value, Error> {
    let mut order_ids_as_numvec: Vec<i32> = Vec::<i32>::new();
    for id in order_ids {
        order_ids_as_numvec.push(id_to_num(id)?);
    }
    Ok(json!({"orderIds": order_ids_as_numvec,}))
}

/// 注文の複数キャンセルAPIを呼び出す。
pub async fn request_cancel_orders(
    http_client: &impl HttpClient,
    order_ids: &[&str],
) -> Result<RestResponse<CancelOrders>, Error> {
    let url = format!("{}{}", PRIVATE_ENDPOINT, CANCEL_ORDERS_API_PATH,);
    let parameters = build_parameters(&order_ids)?;
    let headers = Headers::create_post_headers(&CANCEL_ORDERS_API_PATH, &parameters)?;
    let response = http_client.post(url, &headers, &parameters).await?;
    parse_from_http_response::<CancelOrders>(&response)
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
            "failed": [
              {
                "message_code": "ERR-5122",
                "message_string": "The request is invalid due to the status of the specified order.",
                "orderId": 1
              },
              {
                "message_code": "ERR-5122",
                "message_string": "The request is invalid due to the status of the specified order.",
                "orderId": 2
              }
            ],
            "success": [3,4]
        },
        "responsetime": "2019-03-19T01:07:24.557Z"
    }
    "#;

    #[tokio::test]
    async fn test_cancel_orders() {
        let body = SAMPLE_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_cancel_orders(&http_client, &vec!["1", "2", "3", "4"])
            .await
            .unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2019-03-19T01:07:24.557Z"
        );
        assert_eq!(resp.failed().len(), 2);
        assert_eq!(resp.success().len(), 2);
    }
}
