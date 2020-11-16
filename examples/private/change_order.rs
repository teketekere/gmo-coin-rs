use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::private::*;

/// 注文変更APIのExample
///
/// # Example
///
/// 実行前に環境変数`GMO_COIN_API_KEY`, `GMO_COIN_API_SECRET`にGMOコインのAPIキー、APIシークレットを設定します。
/// また`GMO_COIN_ORDER_ID`で注文IDを、`GMO_COIN_CHANGE_PRICE`で指値価格を指定します。
///
/// Private APIは実際に注文などが行われます。実行する際は十分気を付けてください。
/// いかなる損害が発生しても当方は何ら責任を負いません。
/// 全て自己責任でお願いします。
///
/// ```
/// cargo build --examples
/// cargo run --example change_order
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let order_id = std::env::var("GMO_COIN_ORDER_ID")?;
    let price: i64 = std::env::var("GMO_COIN_CHANGE_PRICE")?.parse().unwrap();

    let http_client = Reqwest;
    let private_api = PrivateAPI::<Reqwest> { http_client };
    let response = private_api.change_order(&order_id, price).await?;

    // ロスカットレートを指定する場合。
    // let losscut_rate = ...;
    // let response = private_api.change_order_with_options(&order_id, price, losscut_rate).await?;

    println!("HTTPステータスコード: {}", response.http_status_code);
    println!("ステータスコード: {}", response.body.status);
    println!("APIを呼び出した時間: {}", response.body.responsetime,);
    Ok(())
}
