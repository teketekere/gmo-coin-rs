use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::private::*;
use gmo_coin_rs::side::Side;
use gmo_coin_rs::symbol::Symbol;

/// 新規成行注文APIのExample
///
/// # Example
///
/// 実行前に環境変数`GMO_API_KEY`, `GMO_API_SECRET`にGMOコインのAPIキー、APIシークレットを設定します。
///
/// Private APIは実際に注文などが行われます。実行する際は十分気を付けてください。
/// いかなる損害が発生しても当方は何ら責任を負いません。
/// 全て自己責任でお願いします。
///
/// ```
/// cargo build --examples
/// cargo run --example market_order
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = std::env::var("GMO_API_KEY")?;
    let secret_key = std::env::var("GMO_API_SECRET")?;
    let size = 0.01; // !!! 最小サイズ !!!

    let http_client = Reqwest;
    let private_api = PrivateAPI::<Reqwest> { http_client };
    let response = private_api
        .market_order(&api_key, &secret_key, &Symbol::BtcJpy, &Side::Buy, size)
        .await?;

    // 執行数量条件を指定する場合。
    // use gmo_coin_rs::time_in_force::TimeInForce;
    // let response = private_api.market_order_with_options(&api_key, &secret_key, &Symbol::BtcJpy, &Side::Buy, size, &TimeInForce::Fak).await?;

    println!("注文ID: {}", response.order_id());
    println!("HTTPステータスコード: {}", response.http_status_code);
    println!("ステータスコード: {}", response.body.status);
    println!("APIを呼び出した時間: {}", response.body.responsetime,);
    Ok(())
}
