//! 注文変更APIを実装する。

use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{json, Value};

/// 注文変更APIのパス。
const CHANGE_ORDER_API_PATH: &str = "/v1/changeOrder";

/// 注文変更APIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct ChangeOrder {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,
}

fn build_parameters(order_id: &str, price: i64, losscut_price: Option<i64>) -> Value {
    match losscut_price {
        Some(lp) => json!({
            "orderId": order_id.to_string(),
            "price": price.to_string(),
            "losscutPrice": lp.to_string(),
        }),
        None => json!({
            "orderId": order_id.to_string(),
            "price": price.to_string(),
        }),
    }
}

/// 注文変更APIを呼び出す。
pub async fn request_change_order(
    http_client: &impl HttpClient,
    order_id: &str,
    price: i64,
    losscut_price: Option<i64>,
) -> Result<RestResponse<ChangeOrder>, Error> {
    let url = format!("{}{}", PRIVATE_ENDPOINT, CHANGE_ORDER_API_PATH,);
    let parameters = build_parameters(order_id, price, losscut_price);
    let headers = Headers::create_post_headers(&CHANGE_ORDER_API_PATH, &parameters)?;
    let response = http_client.post(url, &headers, &parameters).await?;
    parse_from_http_response::<ChangeOrder>(&response)
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
    async fn test_change_order() {
        let body = SAMPLE_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_change_order(&http_client, "orderid", 100, None)
            .await
            .unwrap();
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
