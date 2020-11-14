use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::public::*;
use gmo_coin_rs::symbol::BTC;

/// 取引履歴を取得するAPIのExample
///
/// # Example
/// ```
/// cargo build --examples
/// cargo run --example trades
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let http_client = Reqwest;
    let public_api = PublicAPI::<Reqwest> { http_client };
    let response = public_api.trades(BTC).await?;

    // 取得対象ページ、1ページ当たりの取得件数を取得する場合は次のようにする。
    // let page = 2;
    // let count = 30;
    // let response = public_api.trades_with_options(BTC, page, count).await?;

    println!("取引履歴:");
    let trades = response.trades();
    for trade in trades {
        println!(
            "約定価格: {}, 売買区分: {}, 約定数量: {}, 約定日時: {}",
            trade.price, trade.side, trade.size, trade.timestamp
        );
    }

    println!("取得対象ページ: {}", response.pagination().currentPage);
    println!("取得件数: {}", response.pagination().count);

    println!(
        "HTTPステータスコード: {}\nステータスコード: {}\nAPIを呼び出した時間: {}",
        response.http_status_code, response.body.status, response.body.responsetime,
    );

    Ok(())
}
