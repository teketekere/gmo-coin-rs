//! 注文方法を定義する。

/// 注文方法
pub enum ExecutionType {
    /// 成行注文
    Market,

    /// 指値注文
    Limit,

    /// 逆指値注文
    Stop,
}

/// 成行注文
pub const MARKET_ORDER: &str = "MARKET";

/// 指値注文
pub const LIMIT_ORDER: &str = "LIMIT";

/// 逆指値注文
pub const STOP_ORDER: &str = "STOP";

impl ExecutionType {
    /// 注文方法を文字列に変換する。
    pub fn to_string(&self) -> &str {
        match self {
            ExecutionType::Market => MARKET_ORDER,
            ExecutionType::Limit => LIMIT_ORDER,
            ExecutionType::Stop => STOP_ORDER,
        }
    }
}
