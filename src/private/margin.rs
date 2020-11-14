//! 余力情報APIを実装する。

use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use crate::secret::Secret;
use crate::timestamp::get_timestamp;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 余力APIのパス。
const MARGIN_API_PATH: &str = "/v1/account/margin";

/// 余力APIを呼び出すときのメソッド。
const MARGIN_API_METHOD: &str = "GET";

/// 余力APIから返ってくるレスポンスのうち`data`の部分を格納する構造体。
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Data {
    #[serde(deserialize_with = "str_to_i64")]
    actualProfitLoss: i64,
    #[serde(deserialize_with = "str_to_i64")]
    availableAmount: i64,
    #[serde(deserialize_with = "str_to_i64")]
    margin: i64,
    #[serde(deserialize_with = "str_to_i64")]
    profitLoss: i64,
}

/// 余力APIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct Margin {
    pub status: i16,
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,
    pub data: Data,
}

impl RestResponse<Margin> {
    pub fn actual_profitloss(&self) -> i64 {
        self.body.data.actualProfitLoss
    }

    pub fn availabel_amount(&self) -> i64 {
        self.body.data.availableAmount
    }

    pub fn margin(&self) -> i64 {
        self.body.data.margin
    }

    pub fn profit_loss(&self) -> i64 {
        self.body.data.profitLoss
    }
}

/// 余力情報APIを呼び出す。
pub async fn get_margin(
    http_client: &impl HttpClient,
    api_key: &str,
    secret_key: &str,
) -> Result<RestResponse<Margin>, Error> {
    let url = format!("{}{}", PRIVATE_ENDPOINT, MARGIN_API_PATH,);
    let timestamp = get_timestamp();
    let text = format!("{}{}{}", timestamp, MARGIN_API_METHOD, MARGIN_API_PATH);
    let secret = Secret::create(&api_key, &secret_key, &text);
    let headers = Headers::create_get_headers(&secret, timestamp);
    let response = http_client.get(url, &headers).await?;
    parse_from_http_response::<Margin>(&response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_client::tests::InmemClient;
    use chrono::SecondsFormat;

    const MARGIN_RESPONSE_SAMPLE: &str = r#"
    {
        "status": 0,
        "data": {
          "actualProfitLoss": "5204923",
          "availableAmount": "5189523",
          "margin": "7298",
          "profitLoss": "8019"
        },
        "responsetime": "2019-03-19T02:15:06.051Z"
    }
          "#;

    #[tokio::test]
    async fn should_return_ok_when_http_client_returns_correct_response() {
        let body = MARGIN_RESPONSE_SAMPLE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = get_margin(&http_client, "apikey", "seckey").await.unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2019-03-19T02:15:06.051Z"
        );
        assert_eq!(resp.actual_profitloss(), 5204923);
        assert_eq!(resp.availabel_amount(), 5189523);
        assert_eq!(resp.margin(), 7298);
        assert_eq!(resp.profit_loss(), 8019);
    }
}
