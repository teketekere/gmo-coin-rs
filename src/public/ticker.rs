//! 最新レートAPIを実装する。

use crate::end_point::*;
use crate::error::Error;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 取引所ステータスAPIのパス。
const TICKER_API_PATH: &str = "/v1/ticker";

/// 最新レートAPIから返ってくるレスポンスのうち`data`の部分を格納する構造体。
#[derive(Deserialize)]
pub struct Data {
    #[serde(deserialize_with = "str_to_i64")]
    pub ask: i64,
    #[serde(deserialize_with = "str_to_i64")]
    pub bid: i64,
    #[serde(deserialize_with = "str_to_i64")]
    pub high: i64,
    #[serde(deserialize_with = "str_to_i64")]
    pub last: i64,
    #[serde(deserialize_with = "str_to_i64")]
    pub low: i64,
    pub symbol: String,
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(deserialize_with = "str_to_f64")]
    pub volume: f64,
}

/// 最新レートAPIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct Ticker {
    pub status: i16,
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,
    pub data: Vec<Data>,
}

impl RestResponse<Ticker> {
    pub fn ask(&self) -> Result<i64, Error> {
        let d = self.body.data.get(0).ok_or(Error::DeserializeError {})?;
        Ok(d.ask)
    }

    pub fn bid(&self) -> Result<i64, Error> {
        let d = self.body.data.get(0).ok_or(Error::DeserializeError {})?;
        Ok(d.bid)
    }

    pub fn high(&self) -> Result<i64, Error> {
        let d = self.body.data.get(0).ok_or(Error::DeserializeError {})?;
        Ok(d.high)
    }

    pub fn last(&self) -> Result<i64, Error> {
        let d = self.body.data.get(0).ok_or(Error::DeserializeError {})?;
        Ok(d.last)
    }

    pub fn low(&self) -> Result<i64, Error> {
        let d = self.body.data.get(0).ok_or(Error::DeserializeError {})?;
        Ok(d.low)
    }

    pub fn symbol(&self) -> Result<&String, Error> {
        let d = self.body.data.get(0).ok_or(Error::DeserializeError {})?;
        Ok(&d.symbol)
    }

    pub fn timestamp(&self) -> Result<&DateTime<Utc>, Error> {
        let d = self.body.data.get(0).ok_or(Error::DeserializeError {})?;
        Ok(&d.timestamp)
    }

    pub fn volume(&self) -> Result<f64, Error> {
        let d = self.body.data.get(0).ok_or(Error::DeserializeError {})?;
        Ok(d.volume)
    }
}

/// 最新レートAPIを呼び出す。
pub async fn get_ticker(
    http_client: &impl HttpClient,
    symbol: &str,
) -> Result<RestResponse<Ticker>, Error> {
    let response = http_client
        .get(format!(
            "{}{}?symbol={}",
            PUBLIC_ENDPOINT, TICKER_API_PATH, symbol
        ))
        .await?;
    let body: Ticker = serde_json::from_str(&response.body_text)?;
    Ok(RestResponse::<Ticker> {
        http_status_code: (response.http_status_code),
        body: (body),
    })
}

#[cfg(test)]
mod tests {
    use crate::http_client::tests::InmemClient;
    use crate::public::ticker::*;
    use chrono::SecondsFormat;

    const TICKER_RESPONSE_SAMPLE: &str = r#"{
        "status": 0,
        "data": [
          {
            "ask": "750760",
            "bid": "750600",
            "high": "762302",
            "last": "756662",
            "low": "704874",
            "symbol": "BTC",
            "timestamp": "2018-03-30T12:34:56.789Z",
            "volume": "194785.8484"
          }
        ],
        "responsetime": "2019-03-19T02:15:06.014Z"
      }"#;

    #[tokio::test]
    async fn should_return_ok_when_http_client_returns_correct_response() {
        let body = TICKER_RESPONSE_SAMPLE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = get_ticker(&http_client, "BTC").await.unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2019-03-19T02:15:06.014Z"
        );
        assert_eq!(resp.ask().unwrap(), 750760);
        assert_eq!(resp.bid().unwrap(), 750600);
        assert_eq!(resp.high().unwrap(), 762302);
        assert_eq!(resp.last().unwrap(), 756662);
        assert_eq!(resp.low().unwrap(), 704874);
        assert_eq!(resp.symbol().unwrap(), "BTC");
        assert_eq!(
            resp.timestamp()
                .unwrap()
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2018-03-30T12:34:56.789Z"
        );
        assert_eq!(resp.volume().unwrap(), 194785.8484);
    }
}
