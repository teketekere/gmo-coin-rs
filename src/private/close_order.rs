//! 決済注文APIを実装する。

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

/// 決済注文APIのパス。
const CLOSE_ORDER_API_PATH: &str = "/v1/closeOrder";

/// 決済注文APIから返ってくるレスポンスを格納する構造体。
#[derive(Deserialize)]
pub struct CloseOrder {
    /// ステータスコード。
    pub status: i16,

    /// APIが呼び出された時間。
    #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
    pub responsetime: DateTime<Utc>,

    /// 注文ID。
    pub data: String,
}

impl RestResponse<CloseOrder> {
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
    price: Option<i64>,
    position_id: &str,
    time_in_force: &TimeInForce,
) -> Result<Value, Error> {
    Ok(match execution_type {
        ExecutionType::Market => build_market_parameters(
            execution_type,
            symbol,
            side,
            size,
            position_id,
            time_in_force,
        )?,
        _ => match price {
            Some(p) => build_limit_or_stop_paramters(
                execution_type,
                symbol,
                side,
                size,
                p,
                position_id,
                time_in_force,
            )?,
            None => return Err(Error::PriceNotSpecifiedError()),
        },
    })
}

fn build_market_parameters(
    execution_type: &ExecutionType,
    symbol: &Symbol,
    side: &Side,
    size: f64,
    position_id: &str,
    time_in_force: &TimeInForce,
) -> Result<Value, Error> {
    Ok(json!({
        "executionType": execution_type.to_string(),
        "symbol": symbol.to_string(),
        "side": side.to_string(),
        "timeInForce": time_in_force.to_string(),
        "settlePosition": [
            {
                "positionId": id_to_num(position_id)?,
                "size": size.to_string(),
            }
        ]
    }))
}

fn build_limit_or_stop_paramters(
    execution_type: &ExecutionType,
    symbol: &Symbol,
    side: &Side,
    size: f64,
    price: i64,
    position_id: &str,
    time_in_force: &TimeInForce,
) -> Result<Value, Error> {
    Ok(json!({
        "executionType": execution_type.to_string(),
        "symbol": symbol.to_string(),
        "side": side.to_string(),
        "timeInForce": time_in_force.to_string(),
        "price": price.to_string(),
        "settlePosition": [
            {
                "positionId": id_to_num(position_id)?,
                "size": size.to_string(),
            }
        ]
    }))
}

/// 決済注文APIを呼び出す。
pub async fn request_close_order(
    http_client: &impl HttpClient,
    execution_type: &ExecutionType,
    symbol: &Symbol,
    side: &Side,
    size: f64,
    price: Option<i64>,
    position_id: &str,
    time_in_force: &TimeInForce,
) -> Result<RestResponse<CloseOrder>, Error> {
    let url = format!("{}{}", PRIVATE_ENDPOINT, CLOSE_ORDER_API_PATH,);
    let parameters = build_parameters(
        execution_type,
        symbol,
        side,
        size,
        price,
        position_id,
        time_in_force,
    )?;
    let headers = Headers::create_post_headers(CLOSE_ORDER_API_PATH, &parameters)?;
    let response = http_client.post(url, &headers, &parameters).await?;
    parse_from_http_response::<CloseOrder>(&response)
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
        "responsetime": "2019-03-19T01:07:24.557Z"
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
        let resp = request_close_order(
            &http_client,
            &ExecutionType::Market,
            &Symbol::BtcJpy,
            &Side::Buy,
            0.1,
            None,
            "110",
            &TimeInForce::Fak,
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
        assert_eq!(resp.order_id(), "637000");
    }
}
