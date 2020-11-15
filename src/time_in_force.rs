//! 執行数量条件を定義する。

/// 執行数量条件
#[derive(Copy, Clone)]
pub enum TimeInForce {
    /// 注文が一部約定後に未執行数量が残った場合、その残数量を失効とする条件。
    Fak,

    /// 注文が一部約定後に未執行数量が残った場合、その残数量を有効とする条件。
    Fas,

    /// 注文の全数量が直ちに約定しない場合、その全数量を失効とする条件。
    Fok,

    /// 指値注文においてMakerにならない場合、その全数量を失効とする条件(Post-only)。
    Sok,
}

pub const FAK: &str = "FAK";
pub const FAS: &str = "FAS";
pub const FOK: &str = "FOK";
pub const SOK: &str = "SOK";

/// 執行数量条件を文字列にする。
pub fn tif_to_string(tif: &TimeInForce) -> &str {
    match tif {
        TimeInForce::Fak => FAK,
        TimeInForce::Fas => FAS,
        TimeInForce::Fok => FOK,
        TimeInForce::Sok => SOK,
    }
}
