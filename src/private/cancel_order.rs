//! 注文キャンセルAPIを実装する。

use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{json, Value};

/// 注文キャンセルAPIのパス。
const CANCEL_ORDER_API_PATH: &str = "/v1/cancelOrder";

/// 注文キャンセルAPIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct CancelOrder {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,
}

fn build_parameters(order_id: &str) -> Result<Value, Error> {
    let order_id_num = id_to_num(order_id)?;
    Ok(json!({"orderId": order_id_num,}))
}

/// 注文キャンセルAPIを呼び出す。
pub async fn request_cancel_order(
    http_client: &impl HttpClient,
    order_id: &str,
) -> Result<RestResponse<CancelOrder>, Error> {
    let url = format!("{}{}", PRIVATE_ENDPOINT, CANCEL_ORDER_API_PATH,);
    let parameters = build_parameters(order_id)?;
    let headers = Headers::create_post_headers(CANCEL_ORDER_API_PATH, &parameters)?;
    let response = http_client.post(url, &headers, &parameters).await?;
    parse_from_http_response::<CancelOrder>(&response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_client::tests::InmemClient;
    use chrono::SecondsFormat;

    const SAMPLE_RESPONSE: &str = r#"
    {
        "status": 0,
        "responsetime": "2019-03-19T02:15:06.108Z"
    }
    "#;

    #[tokio::test]
    async fn test_cancel_order() {
        let body = SAMPLE_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_cancel_order(&http_client, "200").await.unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2019-03-19T02:15:06.108Z"
        );
    }
}
