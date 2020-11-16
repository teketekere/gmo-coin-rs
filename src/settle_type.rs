//! 決済区分を定義する。

pub enum SettleType {
    Open,
    Close,
}

pub const OPEN: &str = "OPEN";
pub const CLOSE: &str = "CLOSE";

impl SettleType {
    /// 決済区分を文字列に変換する。
    pub fn to_string(&self) -> &str {
        match self {
            SettleType::Open => OPEN,
            SettleType::Close => CLOSE,
        }
    }
}
