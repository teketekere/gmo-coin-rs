use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::public::status::*;

use std::time::Instant;

/// 取引所ステータスを取得するAPIのExample
///
/// # Example
/// ```
/// cargo build --examples
/// cargo run --example status
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let http_client = Reqwest;
    let start = Instant::now();
    let response = get_status(http_client).await?;
    let end = start.elapsed();

    println!(
        "かかった時間: {}.{:05} [seconds]",
        end.as_secs(),
        end.subsec_micros() / 1000000
    );
    println!("取引所ステータス: {}\n", response.status());
    println!(
        "HTTPステータスコード: {}\nステータスコード: {}\nAPIを呼び出した時間: {}",
        response.http_status_code, response.body.status, response.body.responsetime,
    );
    Ok(())
}
