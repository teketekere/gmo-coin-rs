//! 板情報APIを実装する。

use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use crate::symbol::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// 板情報APIのパス。
const ORDERBOOKS_API_PATH: &str = "/v1/orderbooks";

/// 価格と注文数量を格納する構造体。
#[derive(Deserialize)]
pub struct PriceAndSize {
    /// 価格。
    #[serde(deserialize_with = "str_to_i64")]
    pub price: i64,

    /// 注文数量。
    #[serde(deserialize_with = "str_to_f64")]
    pub size: f64,
}

/// 板情報APIから返ってくるレスポンスのうち`data`の部分を格納する構造体。
#[derive(Deserialize)]
pub struct Data {
    /// 売り注文の情報。
    pub asks: Vec<PriceAndSize>,

    /// 買い注文の情報。
    pub bids: Vec<PriceAndSize>,

    /// 銘柄名。
    pub symbol: String,
}

/// 板情報APIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct Orderbooks {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,

    /// レスポンスの`data`の部分。
    pub data: Data,
}

impl RestResponse<Orderbooks> {
    /// 売り注文の情報を取得する。
    pub fn asks(&self) -> &Vec<PriceAndSize> {
        &self.body.data.asks
    }

    /// 買い注文の情報を取得する。
    pub fn bids(&self) -> &Vec<PriceAndSize> {
        &self.body.data.bids
    }

    /// 銘柄名を取得する。
    pub fn symbol(&self) -> &String {
        &self.body.data.symbol
    }
}

/// 板情報APIを呼び出す。
pub async fn request_orderbooks(
    http_client: &impl HttpClient,
    symbol: &Symbol,
) -> Result<RestResponse<Orderbooks>, Error> {
    let url = format!(
        "{}{}?symbol={}",
        PUBLIC_ENDPOINT,
        ORDERBOOKS_API_PATH,
        to_string(&symbol)
    );
    let headers = Headers::create_empty_headers();
    let response = http_client.get(url, &headers).await?;
    parse_from_http_response::<Orderbooks>(&response)
}

#[cfg(test)]
mod tests {
    use crate::http_client::tests::InmemClient;
    use crate::public::orderbooks::*;
    use crate::symbol::Symbol;
    use chrono::SecondsFormat;

    const ORDERBOOKS_RESPONSE_SAMPLE: &str = r#"{
            "status": 0,
            "data": {
              "asks": [
                {
                  "price": "455659",
                  "size": "0.1"
                },
                {
                    "price": "455659",
                    "size": "0.1"
                }
              ],
              "bids": [
                {
                  "price": "455659",
                  "size": "0.1"
                }
              ],
              "symbol": "BTC"
            },
            "responsetime": "2019-03-19T02:15:06.026Z"
          }"#;

    #[tokio::test]
    async fn should_return_ok_when_http_client_returns_correct_response() {
        let body = ORDERBOOKS_RESPONSE_SAMPLE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_orderbooks(&http_client, &Symbol::Btc)
            .await
            .unwrap();
        assert_eq!(resp.http_status_code, 200);
        assert_eq!(resp.body.status, 0);
        assert_eq!(
            resp.body
                .responsetime
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2019-03-19T02:15:06.026Z"
        );
        assert_eq!(resp.symbol(), "BTC");

        let asks = resp.asks();
        assert_eq!(asks.len(), 2);

        let bids = resp.bids();
        assert_eq!(bids.len(), 1);
    }
}
