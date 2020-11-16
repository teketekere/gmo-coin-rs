use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::public::*;
use gmo_coin_rs::symbol::Symbol;

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
    let response = public_api.trades(&Symbol::Btc).await?;

    // 取得対象ページ、1ページ当たりの取得件数を指定する場合。
    // let page = 2;
    // let count = 30;
    // let response = public_api.trades_with_options(BTC, page, count).await?;

    println!("取得対象ページ: {}", response.current_page());
    println!("取得件数: {}\n", response.count());

    let trades = response.trades();
    for trade in trades {
        println!("約定価格: {}", trade.price);
        println!("売買区分: {}", trade.side);
        println!("約定数量: {}", trade.size);
        println!("約定日時: {}\n", trade.timestamp);
    }

    println!("HTTPステータスコード: {}", response.http_status_code);
    println!("ステータスコード: {}", response.body.status);
    println!("APIを呼び出した時間: {}", response.body.responsetime,);
    Ok(())
}
