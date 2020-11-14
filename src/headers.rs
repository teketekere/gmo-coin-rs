//! HTTPのGET, POSTリクエスト時のヘッダーを作る処理を実装する。

use crate::secret::*;
use crate::timestamp::get_timestamp;
use std::collections::{hash_map::Iter, HashMap};

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

    /// GETリクエスト時のヘッダーを作る。
    pub fn create_get_headers(
        api_key: &str,
        secret_key: &str,
        method: &str,
        path: &str,
        parameters: &str,
    ) -> Headers {
        let timestamp = get_timestamp();
        let text = format!("{}{}{}{}", timestamp, method, path, parameters);
        let secret = Secret::create(&api_key, &secret_key, &text);

        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert(String::from("API-KEY"), secret.api_key.clone());
        headers.insert(String::from("API-TIMESTAMP"), timestamp.to_string());
        headers.insert(String::from("API-SIGN"), secret.sign.clone());
        Headers(headers)
    }
}
