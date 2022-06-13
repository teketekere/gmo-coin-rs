//! ロスカットレート変更APIを実装する。

use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{json, Value};

/// ロスカットレート変更APIのパス。
const CHANGE_LOSSCUT_PRICE_API_PATH: &str = "/v1/changeLosscutPrice";

/// ロスカットレート変更APIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct ChangeLosscutPrice {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,
}

fn build_parameters(position_id: &str, losscut_price: i64) -> Value {
    json!({
        "positionId": position_id,
        "losscutPrice": losscut_price.to_string(),
    })
}

/// ロスカットレート変更APIを呼び出す。
pub async fn request_change_losscut_price(
    http_client: &impl HttpClient,
    position_id: &str,
    losscut_price: i64,
) -> Result<RestResponse<ChangeLosscutPrice>, Error> {
    let url = format!("{}{}", PRIVATE_ENDPOINT, CHANGE_LOSSCUT_PRICE_API_PATH,);
    let parameters = build_parameters(position_id, losscut_price);
    let headers = Headers::create_post_headers(CHANGE_LOSSCUT_PRICE_API_PATH, &parameters)?;
    let response = http_client.post(url, &headers, &parameters).await?;
    parse_from_http_response::<ChangeLosscutPrice>(&response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_client::tests::InmemClient;
    use chrono::SecondsFormat;

    const SAMPLE_RESPONSE: &str = r#"
    {
        "status": 0,
        "responsetime": "2019-03-19T01:07:24.557Z"
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
        let resp = request_change_losscut_price(&http_client, "200", 100)
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
    }
}
