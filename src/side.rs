//! 売買区分を定義する。

/// 売買区分。
pub enum Side {
    Buy,
    Sell,
}

pub const BUY: &str = "BUY";
pub const SELL: &str = "SELL";

impl Side {
    /// 売買区分を文字列に変換する。
    pub fn to_string(&self) -> &str {
        match self {
            Side::Buy => BUY,
            Side::Sell => SELL,
        }
    }
}
