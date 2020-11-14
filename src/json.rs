//! JSON文字列をパースするときに型変換行うための関数を定義する。

use crate::error::Error;
use crate::response::{ErrorResponse, RawResponse, RestResponse};
use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

/// strからf64への変換を行う。
pub fn str_to_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => {
            num.as_f64()
                .ok_or_else(|| de::Error::custom("Invalid number"))? as f64
        }
        _ => return Err(de::Error::custom("wrong type")),
    })
}

/// strからi64への変換を行う。
pub fn str_to_i64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => {
            num.as_i64()
                .ok_or_else(|| de::Error::custom("Invalid number"))? as i64
        }
        _ => return Err(de::Error::custom("wrong type")),
    })
}

/// GMOコインAPIから返ってくるタイムスタンプをchronoの日時に変換する。
/// GMOコインのタイムスタンプはUTC。この関数でもUTCの日時を返す。
pub fn gmo_timestamp_to_chrono_timestamp<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<DateTime<Utc>, D::Error> {
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(
        match chrono::naive::NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.%3fZ") {
            Ok(date) => DateTime::<Utc>::from_utc(date, Utc),
            Err(_) => return Err(de::Error::custom("wrong datetime format")),
        },
    )
}

/// GMOコインのAPIを呼び出して得られるHTTPレスポンスをええ感じに構造体RestResponse<T>に詰めなおす
pub fn parse_from_http_response<'a, T>(
    http_response: &'a RawResponse,
) -> Result<RestResponse<T>, Error>
where
    T: serde::de::Deserialize<'a>,
{
    let body: Result<T, serde_json::Error> = serde_json::from_str(&http_response.body_text);
    Ok(match body {
        Ok(b) => RestResponse {
            http_status_code: http_response.http_status_code,
            body: b,
        },
        Err(e) => {
            let err_resp: ErrorResponse = serde_json::from_str(&http_response.body_text)?;
            return Err(Error::APIError(err_resp));
        }
    })
}

/// GMOコインからのレスポンスでフィールドが無い場合のデフォルト値。
/// 使用箇所は注文情報取得APIなど。
pub fn get_string_default_value() -> String {
    "NONE".to_string()
}

#[cfg(test)]
mod tests {
    use crate::json::{gmo_timestamp_to_chrono_timestamp, str_to_f64, str_to_i64};
    use chrono::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Number {
        #[serde(deserialize_with = "str_to_i64")]
        i: i64,

        #[serde(deserialize_with = "str_to_f64")]
        f: f64,
    }

    #[derive(Deserialize)]
    struct Date {
        #[serde(deserialize_with = "gmo_timestamp_to_chrono_timestamp")]
        d: DateTime<Utc>,
    }

    #[test]
    fn test_str_to_numbers() {
        let json_str = r#"{"i": "100", "f": "-10.55"}"#;
        let json: Number = serde_json::from_str(&json_str).unwrap();
        assert_eq!(json.i, 100);
        assert_eq!(json.f, -10.55);
    }

    #[test]
    fn test_str_to_datetime() {
        let json_str = r#"{"d": "2019-03-19T02:15:06.001Z"}"#;
        let json: Date = serde_json::from_str(&json_str).unwrap();
        assert_eq!(json.d.year(), 2019);
        assert_eq!(json.d.month(), 3);
        assert_eq!(json.d.day(), 19);
        assert_eq!(json.d.hour(), 2);
        assert_eq!(json.d.minute(), 15);
        assert_eq!(json.d.second(), 6);
        assert_eq!(json.d.timestamp_subsec_millis(), 1);
    }
}
