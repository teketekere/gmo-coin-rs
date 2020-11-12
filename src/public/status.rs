//! 取引所ステータスAPIを実装する。

use crate::end_point::*;
use crate::error::Error;
use crate::http_client::*;
use crate::json::gmo_timestamp_to_chrono_timestamp;
use crate::response::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 取引所ステータスAPIのパス。
const STATUS_API_PATH: &str = "/v1/status";

/// 取引所ステータス OPEN。
const EXCHANGE_STATUS_OPEN: &str = "OPEN";

/// 取引所ステータス PREOPEN。
const EXCHANGE_STATUS_PREOPEN: &str = "PREOPEN";

/// 取引所ステータス MAINTENANCE。
const EXCHANGE_STATUS_MAINTENANCE: &str = "MAINTENANCE";

/// 取引所ステータスAPIから返ってくるレスポンスのうち`data`の部分を格納する構造体。
#[derive(Deserialize)]
pub struct Data {
    pub status: String,
}

/// 取引所ステータスAPIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct Status {
    pub status: i16,
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,
    pub data: Data,
}

impl RestResponse<Status> {
    /// 取引所が開いているか？
    pub fn is_open(&self) -> bool {
        self.body.data.status == EXCHANGE_STATUS_OPEN
    }

    /// 取引所がプレオープン中か？
    /// プレオープンは定時メンテナンスの前後30分の間。
    pub fn is_pre_open(&self) -> bool {
        self.body.data.status == EXCHANGE_STATUS_PREOPEN
    }

    /// 取引所がメンテナンス中か？
    /// 定時メンテナンスは日本時間で毎週水曜15:00 - 16:00。
    pub fn is_maintenance(&self) -> bool {
        self.body.data.status == EXCHANGE_STATUS_MAINTENANCE
    }

    /// 取引所のステータスを返す。
    pub fn status(&self) -> &String {
        &self.body.data.status
    }
}

/// 取引所ステータスAPIを呼び出す。
pub async fn get_status(http_client: &impl HttpClient) -> Result<RestResponse<Status>, Error> {
    let response = http_client
        .get(format!("{}{}", PUBLIC_ENDPOINT, STATUS_API_PATH))
        .await?;
    let body: Status = serde_json::from_str(&response.body_text)?;
    Ok(RestResponse::<Status> {
        http_status_code: (response.http_status_code),
        body: (body),
    })
}

#[cfg(test)]
mod tests {
    use crate::http_client::tests::InmemClient;
    use crate::public::status::*;
    use chrono::SecondsFormat;

    const STATUS_RESPONSE_SAMPLE: &str = r#"{
        "status": 0,
        "data": {
          "status": "OPEN"
        },
        "responsetime": "2019-03-19T02:15:06.001Z"
      }"#;

    #[tokio::test]
    async fn should_return_ok_when_http_client_returns_correct_response() {
        let body = STATUS_RESPONSE_SAMPLE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = get_status(&http_client).await.unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2019-03-19T02:15:06.001Z"
        );
        assert_eq!(resp.status(), "OPEN");
        assert_eq!(resp.is_open(), true);
    }

    #[tokio::test]
    async fn should_return_ng_when_body_cannot_be_parsed() {
        let body = "json parse dekinaiyo";
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = get_status(&http_client).await;
        assert_eq!(resp.is_err(), true);
    }

    #[tokio::test]
    async fn should_return_ng_when_http_client_returns_ng() {
        let body = STATUS_RESPONSE_SAMPLE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: true,
        };
        let resp = get_status(&http_client).await;
        assert_eq!(resp.is_err(), true);
    }
}
