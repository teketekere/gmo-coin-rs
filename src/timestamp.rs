//! タイムスタンプを計算する処理を実装する。

use chrono::{DateTime, Utc};

/// Unixエポックからの経過秒数を取得する。
pub(crate) fn get_timestamp() -> u64 {
    let now: DateTime<Utc> = Utc::now();
    now.timestamp_nanos() as u64 / 1_000_000
}
