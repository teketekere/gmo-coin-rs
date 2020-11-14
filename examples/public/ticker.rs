use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::public::*;
use gmo_coin_rs::symbol::Symbol;

/// 最新レートを取得するAPIのExample
///
/// # Example
/// ```
/// cargo build --examples
/// cargo run --example ticker
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let http_client = Reqwest;
    let public_api = PublicAPI::<Reqwest> { http_client };
    let response = public_api.ticker(&Symbol::Btc).await?;

    println!("ask: {}", response.ask().unwrap());
    println!("bid: {}", response.bid().unwrap());
    println!("high: {}", response.high().unwrap());
    println!("low: {}", response.low().unwrap());
    println!("last: {}", response.last().unwrap());
    println!("volume: {}", response.volume().unwrap());
    println!("timestamp: {}", response.timestamp().unwrap());
    println!("symbol: {}", response.symbol().unwrap());

    println!(
        "HTTPステータスコード: {}\nステータスコード: {}\nAPIを呼び出した時間: {}",
        response.http_status_code, response.body.status, response.body.responsetime,
    );

    Ok(())
}
