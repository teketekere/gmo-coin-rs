//! HTTPのGET, POSTリクエスト時のヘッダーを作る処理を実装する。

use crate::error::Error;
use crate::timestamp::get_timestamp;
use hex::encode;
use ring::hmac;
use serde_json::Value;
use std::collections::{hash_map::Iter, HashMap};

const GMO_COM_API_KEY_ENVNAME: &str = "GMO_COIN_API_KEY";
const GMO_COM_API_SECRET_ENVNAME: &str = "GMO_COIN_API_SECRET";

pub struct Headers(HashMap<String, String>);

impl<'a> IntoIterator for &'a Headers {
    type Item = (&'a String, &'a String);
    type IntoIter = Iter<'a, String, String>;

    #[inline]
    fn into_iter(self) -> Iter<'a, String, String> {
        self.0.iter()
    }
}

impl Headers {
    /// 空のヘッダーを作る。
    pub fn create_empty_headers() -> Headers {
        let headers: HashMap<String, String> = HashMap::new();
        Headers(headers)
    }

    fn api_key() -> Result<String, Error> {
        Ok(std::env::var(GMO_COM_API_KEY_ENVNAME)?)
    }

    fn api_secret() -> Result<String, Error> {
        Ok(std::env::var(GMO_COM_API_SECRET_ENVNAME)?)
    }

    fn sign(text: &str) -> Result<String, Error> {
        let secret_key = Headers::api_secret()?;
        let signed_key = hmac::Key::new(hmac::HMAC_SHA256, secret_key.as_bytes());
        let sign = encode(hmac::sign(&signed_key, text.as_bytes()).as_ref());
        Ok(sign)
    }

    /// GETリクエスト時のヘッダーを作る。
    pub fn create_get_headers(path: &str) -> Result<Headers, Error> {
        let timestamp = get_timestamp();
        let text = format!("{}{}{}", timestamp, "GET", path);
        let api_key = Headers::api_key()?;
        let sign = Headers::sign(&text)?;

        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert(String::from("API-KEY"), api_key);
        headers.insert(String::from("API-TIMESTAMP"), timestamp.to_string());
        headers.insert(String::from("API-SIGN"), sign);
        Ok(Headers(headers))
    }

    /// POSTリクエスト時のヘッダーを作る。
    pub fn create_post_headers(path: &str, parameters: &Value) -> Result<Headers, Error> {
        let timestamp = get_timestamp();
        let text = format!("{}{}{}{}", timestamp, "POST", path, &parameters);
        let api_key = Headers::api_key()?;
        let sign = Headers::sign(&text)?;

        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert(String::from("content-type"), "application/json".to_string());
        headers.insert(String::from("API-KEY"), api_key);
        headers.insert(String::from("API-TIMESTAMP"), timestamp.to_string());
        headers.insert(String::from("API-SIGN"), sign);
        Ok(Headers(headers))
    }
}
