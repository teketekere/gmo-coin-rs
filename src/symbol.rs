//! 取引銘柄名を定義する。

/// 取引可能な銘柄の候補。
pub enum Symbol {
    Btc,
    Eth,
    Bch,
    Ltc,
    Xrp,
    BtcJpy,
    EthJpy,
    BchJpy,
    LtcJpy,
    XprJpy,
}

/// ビットコイン（現物取引）の銘柄名
pub const BTC: &str = "BTC";

/// イーサリアム（現物取引）の銘柄名
pub const ETH: &str = "ETH";

/// ビットコインキャッシュ（現物取引）の銘柄名
pub const BCH: &str = "BCH";

/// ライトコイン（現物取引）の銘柄名
pub const LTC: &str = "LTC";

/// リップル（現物取引）の銘柄名
pub const XRP: &str = "XRP";

/// ビットコイン/円（レバレッジ取引）の銘柄名
pub const BTC_JPY: &str = "BTC_JPY";

/// イーサリアム/円（レバレッジ取引）の銘柄名
pub const ETH_JPY: &str = "ETH_JPY";

/// ビットコインキャッシュ/円（レバレッジ取引）の銘柄名
pub const BCH_JPY: &str = "BCH_JPY";

/// ライトコイン/円（レバレッジ取引）の銘柄名
pub const LTC_JPY: &str = "LTC_JPY";

/// リップル/円（レバレッジ取引）の銘柄名
pub const XRP_JPY: &str = "XRP_JPY";

/// 取引銘柄を文字列に変換する。
pub fn to_string(symbol: &Symbol) -> &str {
    match symbol {
        Symbol::Btc => BTC,
        Symbol::Eth => ETH,
        Symbol::Bch => BCH,
        Symbol::Ltc => LTC,
        Symbol::Xrp => XRP,
        Symbol::BtcJpy => BTC_JPY,
        Symbol::EthJpy => ETH_JPY,
        Symbol::BchJpy => BCH_JPY,
        Symbol::LtcJpy => LTC_JPY,
        Symbol::XprJpy => XRP_JPY,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(to_string(&Symbol::Btc), BTC);
        assert_eq!(to_string(&Symbol::Eth), ETH);
        assert_eq!(to_string(&Symbol::Bch), BCH);
        assert_eq!(to_string(&Symbol::Ltc), LTC);
        assert_eq!(to_string(&Symbol::Xrp), XRP);
        assert_eq!(to_string(&Symbol::BtcJpy), BTC_JPY);
        assert_eq!(to_string(&Symbol::EthJpy), ETH_JPY);
        assert_eq!(to_string(&Symbol::BchJpy), BCH_JPY);
        assert_eq!(to_string(&Symbol::LtcJpy), LTC_JPY);
        assert_eq!(to_string(&Symbol::XprJpy), XRP_JPY);
    }
}
