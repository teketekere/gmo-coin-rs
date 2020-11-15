//! 最新レートAPIを実装する。

use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use crate::symbol::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 取引所ステータスAPIのパス。
const TICKER_API_PATH: &str = "/v1/ticker";

/// 最新レートAPIから返ってくるレスポンスのうち`data`の部分を格納する構造体。
#[derive(Deserialize)]
pub struct Data {
    /// ASK。
    #[serde(deserialize_with = "str_to_i64")]
    pub ask: i64,

    /// BID。
    #[serde(deserialize_with = "str_to_i64")]
    pub bid: i64,

    /// 高値。
    #[serde(deserialize_with = "str_to_i64")]
    pub high: i64,

    /// 終値。
    #[serde(deserialize_with = "str_to_i64")]
    pub last: i64,

    /// 安値。
    #[serde(deserialize_with = "str_to_i64")]
    pub low: i64,

    /// 銘柄名。
    pub symbol: String,
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]

    /// 時刻。
    pub timestamp: DateTime<Utc>,

    /// 24時間の取引量。
    #[serde(deserialize_with = "str_to_f64")]
    pub volume: f64,
}

/// 最新レートAPIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct Ticker {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,

    /// レスポンスの`data`の部分。
    pub data: Vec<Data>,
}

impl RestResponse<Ticker> {
    /// ASKを取得する。
    pub fn ask(&self) -> Result<i64, Error> {
        let d = self.body.data.get(0).ok_or(Error::EmptyResponseError {})?;
        Ok(d.ask)
    }

    /// BIDを取得する。
    pub fn bid(&self) -> Result<i64, Error> {
        let d = self.body.data.get(0).ok_or(Error::EmptyResponseError {})?;
        Ok(d.bid)
    }

    /// 高値を取得する。
    pub fn high(&self) -> Result<i64, Error> {
        let d = self.body.data.get(0).ok_or(Error::EmptyResponseError {})?;
        Ok(d.high)
    }

    /// 終値を取得する。
    pub fn last(&self) -> Result<i64, Error> {
        let d = self.body.data.get(0).ok_or(Error::EmptyResponseError {})?;
        Ok(d.last)
    }

    /// 安値を取得する。
    pub fn low(&self) -> Result<i64, Error> {
        let d = self.body.data.get(0).ok_or(Error::EmptyResponseError {})?;
        Ok(d.low)
    }

    /// 銘柄名を取得する。
    pub fn symbol(&self) -> Result<&String, Error> {
        let d = self.body.data.get(0).ok_or(Error::EmptyResponseError {})?;
        Ok(&d.symbol)
    }

    /// 時刻を取得する。
    pub fn timestamp(&self) -> Result<&DateTime<Utc>, Error> {
        let d = self.body.data.get(0).ok_or(Error::EmptyResponseError {})?;
        Ok(&d.timestamp)
    }

    /// 取引量を取得する。
    pub fn volume(&self) -> Result<f64, Error> {
        let d = self.body.data.get(0).ok_or(Error::EmptyResponseError {})?;
        Ok(d.volume)
    }
}

/// 最新レートAPIを呼び出す。
pub async fn request_ticker(
    http_client: &impl HttpClient,
    symbol: &Symbol,
) -> Result<RestResponse<Ticker>, Error> {
    let url = format!(
        "{}{}?symbol={}",
        PUBLIC_ENDPOINT,
        TICKER_API_PATH,
        to_string(&symbol)
    );
    let headers = Headers::create_empty_headers();
    let response = http_client.get(url, &headers).await?;
    parse_from_http_response::<Ticker>(&response)
}

#[cfg(test)]
mod tests {
    use crate::http_client::tests::InmemClient;
    use crate::public::ticker::*;
    use crate::symbol::Symbol;
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
        let resp = request_ticker(&http_client, &Symbol::Btc).await.unwrap();
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
