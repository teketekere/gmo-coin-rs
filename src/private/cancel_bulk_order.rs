//! 注文の一括キャンセルAPIを実装する。

use crate::dto::get_vector_default_value;
use crate::end_point::*;
use crate::error::Error;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use crate::settle_type::SettleType;
use crate::side::Side;
use crate::symbol::Symbol;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{json, Value};

/// 注文の一括キャンセルAPIのパス。
const CANCEL_BULK_ORDERS_API_PATH: &str = "/v1/cancelBulkOrder";

/// 注文の一括キャンセルAPIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct CancelBulkOrder {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,

    /// レスポンスの`data`の部分。
    #[serde(
        deserialize_with = "ids_to_strvec",
        default = "get_vector_default_value"
    )]
    pub data: Vec<String>,
}

impl RestResponse<CancelBulkOrder> {
    /// 注文Idが格納された配列を取得する。
    pub fn order_ids(&self) -> &Vec<String> {
        &self.body.data
    }
}

fn build_parameters(
    symbols: &[&Symbol],
    side: Option<&Side>,
    settle_type: Option<&SettleType>,
    desc: bool,
) -> Result<Value, Error> {
    let mut symbols_strvec = Vec::<String>::new();
    for symbol in symbols {
        symbols_strvec.push(symbol.to_string().to_string());
    }

    match (side, settle_type) {
        (Some(s), Some(st)) => Ok(json!({
            "symbols": symbols_strvec,
            "side": s.to_string(),
            "settleType": st.to_string(),
            "desc": desc,
        })),
        (Some(s), None) => Ok(json!({
            "symbols": symbols_strvec,
            "side": s.to_string(),
            "desc": desc,
        })),
        (None, Some(st)) => Ok(json!({
            "symbols": symbols_strvec,
            "settleType": st.to_string(),
            "desc": desc,
        })),
        (None, None) => Ok(json!({
            "symbols": symbols_strvec,
            "desc": desc,
        })),
    }
}

/// 注文の一括キャンセルAPIを呼び出す。
pub async fn request_cancel_bulk_order(
    http_client: &impl HttpClient,
    symbols: &[&Symbol],
    side: Option<&Side>,
    settle_type: Option<&SettleType>,
    desc: bool,
) -> Result<RestResponse<CancelBulkOrder>, Error> {
    let url = format!("{}{}", PRIVATE_ENDPOINT, CANCEL_BULK_ORDERS_API_PATH,);
    let parameters = build_parameters(&symbols, side, settle_type, desc)?;
    let headers = Headers::create_post_headers(&CANCEL_BULK_ORDERS_API_PATH, &parameters)?;
    let response = http_client.post(url, &headers, &parameters).await?;
    parse_from_http_response::<CancelBulkOrder>(&response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_client::tests::InmemClient;
    use chrono::SecondsFormat;

    const SAMPLE_RESPONSE: &str = r#"
    {
        "status": 0,
        "data": [637000,637002],
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
        let resp = request_cancel_bulk_order(
            &http_client,
            &vec![&Symbol::Btc, &Symbol::EthJpy],
            Some(&Side::Sell),
            None,
            false,
        )
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
        assert_eq!(resp.order_ids().len(), 2);
    }
}
