use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::public::*;
use gmo_coin_rs::symbol::Symbol;

/// 板情報を取得するAPIのExample
///
/// # Example
/// ```
/// cargo build --examples
/// cargo run --example orderbooks
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let http_client = Reqwest;
    let public_api = PublicAPI::<Reqwest> { http_client };
    let response = public_api.orderbooks(&Symbol::Btc).await?;

    println!("ask:");
    let asks = response.asks();
    for ask in asks {
        println!("price: {}, size: {}", ask.price, ask.size);
    }

    println!("bids:");
    let bids = response.bids();
    for bid in bids {
        println!("price: {}, size: {}", bid.price, bid.size);
    }

    println!("symbol: {}", response.symbol());

    println!(
        "HTTPステータスコード: {}\nステータスコード: {}\nAPIを呼び出した時間: {}",
        response.http_status_code, response.body.status, response.body.responsetime,
    );

    Ok(())
}
