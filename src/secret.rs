//! GMOコインのAPIキーを扱う処理を実装する。

use hex::encode;
use ring::hmac;

pub struct Secret {
    pub api_key: String,
    pub sign: String,
}

impl Secret {
    pub fn create(api_key: &str, secret_key: &str, text: &str) -> Secret {
        let signed_key = hmac::Key::new(hmac::HMAC_SHA256, secret_key.as_bytes());
        let sign = encode(hmac::sign(&signed_key, text.as_bytes()).as_ref());
        Secret {
            api_key: String::from(api_key),
            sign: sign,
        }
    }
}
