//! JSON文字列をパースするときに型変換行うための関数を定義する。

use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

/// strからf64への変換を行う。
pub fn str_to_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num.as_f64().ok_or(de::Error::custom("Invalid number"))? as f64,
        _ => return Err(de::Error::custom("wrong type")),
    })
}

/// strからi64への変換を行う。
pub fn str_to_i64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num.as_i64().ok_or(de::Error::custom("Invalid number"))? as i64,
        _ => return Err(de::Error::custom("wrong type")),
    })
}

#[cfg(test)]
mod tests {
    use crate::json::{str_to_f64, str_to_i64};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Number {
        #[serde(deserialize_with = "str_to_i64")]
        i: i64,

        #[serde(deserialize_with = "str_to_f64")]
        f: f64,
    }

    #[test]
    fn test_str_to_numbers() {
        let json_str = r#"{"i": "100", "f": "-10.55"}"#;
        let json: Number = serde_json::from_str(&json_str).unwrap();
        assert_eq!(json.i, 100);
        assert_eq!(json.f, -10.55);
    }
}
