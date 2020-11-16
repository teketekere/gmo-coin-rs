//! 新規注文APIを実装する。

#![allow(clippy::too_many_arguments)]

use crate::end_point::*;
use crate::error::Error;
use crate::execution_type::ExecutionType;
use crate::headers::Headers;
use crate::http_client::*;
use crate::json::*;
use crate::response::*;
use crate::side::Side;
use crate::symbol::Symbol;
use crate::time_in_force::TimeInForce;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{json, Value};

/// 新規注文APIのパス。
const ORDER_API_PATH: &str = "/v1/order";

/// 新規注文APIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct Order {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,

    /// 注文ID。
    pub data: String,
}

impl RestResponse<Order> {
    /// 注文IDを取得する。
    pub fn order_id(&self) -> &str {
        &self.body.data
    }
}

fn build_parameters(
    execution_type: &ExecutionType,
    symbol: &Symbol,
    side: &Side,
    size: f64,
    time_in_force: &TimeInForce,
    price: Option<i64>,
    losscut_price: Option<i64>,
) -> Value {
    match execution_type {
        ExecutionType::Market => {
            build_market_parameters(&execution_type, &symbol, &side, size, &time_in_force)
        }
        _ => match price {
            Some(p) => build_limit_or_stop_paramters(
                &execution_type,
                &symbol,
                &side,
                size,
                &time_in_force,
                p,
                losscut_price,
            ),
            None => panic!("limit/stop order need price"),
        },
    }
}

fn build_market_parameters(
    execution_type: &ExecutionType,
    symbol: &Symbol,
    side: &Side,
    size: f64,
    time_in_force: &TimeInForce,
) -> Value {
    json!({
        "executionType": execution_type.to_string(),
        "symbol": symbol.to_string(),
        "side": side.to_string(),
        "size": size.to_string(),
        "timeInForce": time_in_force.to_string(),
    })
}

fn build_limit_or_stop_paramters(
    execution_type: &ExecutionType,
    symbol: &Symbol,
    side: &Side,
    size: f64,
    time_in_force: &TimeInForce,
    price: i64,
    losscut_price: Option<i64>,
) -> Value {
    match losscut_price {
        Some(lp) => json!({
            "symbol": symbol.to_string(),
            "side": side.to_string(),
            "executionType": execution_type.to_string(),
            "size": size.to_string(),
            "price": price.to_string(),
            "timeInForce": time_in_force.to_string(),
            "losscutPrice": lp,
        }),
        None => json!({
            "symbol": symbol.to_string(),
            "side": side.to_string(),
            "executionType": execution_type.to_string(),
            "size": size.to_string(),
            "price": price.to_string(),
            "timeInForce": time_in_force.to_string(),
        }),
    }
}

/// 新規注文APIを呼び出す。
pub async fn request_order(
    http_client: &impl HttpClient,
    execution_type: &ExecutionType,
    symbol: &Symbol,
    side: &Side,
    size: f64,
    time_in_force: &TimeInForce,
    price: Option<i64>,
    losscut_price: Option<i64>,
) -> Result<RestResponse<Order>, Error> {
    let url = format!("{}{}", PRIVATE_ENDPOINT, ORDER_API_PATH,);
    let parameters = build_parameters(
        &execution_type,
        &symbol,
        &side,
        size,
        &time_in_force,
        price,
        losscut_price,
    );
    let headers = Headers::create_post_headers(&ORDER_API_PATH, &parameters)?;
    let response = http_client.post(url, &headers, &parameters).await?;
    parse_from_http_response::<Order>(&response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_client::tests::InmemClient;
    use chrono::SecondsFormat;

    const SAMPLE_RESPONSE: &str = r#"
    {
        "status": 0,
        "data": "637000",
        "responsetime": "2019-03-19T02:15:06.108Z"
    }
    "#;

    #[tokio::test]
    async fn test_market_order() {
        let body = SAMPLE_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_order(
            &http_client,
            &ExecutionType::Market,
            &Symbol::BtcJpy,
            &Side::Buy,
            0.1,
            &TimeInForce::Fak,
            None,
            None,
        )
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
        assert_eq!(resp.order_id(), "637000");
    }

    #[tokio::test]
    async fn test_limit_order() {
        let body = SAMPLE_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_order(
            &http_client,
            &ExecutionType::Limit,
            &Symbol::BtcJpy,
            &Side::Buy,
            0.1,
            &TimeInForce::Fas,
            Some(100),
            Some(100),
        )
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
        assert_eq!(resp.order_id(), "637000");
    }

    #[tokio::test]
    async fn test_stop_order() {
        let body = SAMPLE_RESPONSE;
        let http_client = InmemClient {
            http_status_code: 200,
            body_text: body.to_string(),
            return_error: false,
        };
        let resp = request_order(
            &http_client,
            &ExecutionType::Stop,
            &Symbol::BtcJpy,
            &Side::Buy,
            0.1,
            &TimeInForce::Fas,
            Some(100),
            None,
        )
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
        assert_eq!(resp.order_id(), "637000");
    }
}
