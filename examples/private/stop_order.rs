use gmo_coin_rs::error::Error;
use gmo_coin_rs::execution_type::ExecutionType;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::private::*;
use gmo_coin_rs::side::Side;
use gmo_coin_rs::symbol::Symbol;

/// 新規逆指値注文APIのExample
///
/// # Example
///
/// 実行前に環境変数`GMO_COIN_API_KEY`, `GMO_COIN_API_SECRET`にGMOコインのAPIキー、APIシークレットを設定します。
/// また`GMO_COIN_STOP_PRICE`で指値価格を指定します。
///
/// Private APIは実際に注文などが行われます。実行する際は十分気を付けてください。
/// いかなる損害が発生しても当方は何ら責任を負いません。
/// 全て自己責任でお願いします。
///
/// ```
/// cargo build --examples
/// cargo run --example stop_order
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let price: i64 = std::env::var("GMO_COIN_STOP_PRICE")?.parse().unwrap();
    let size = 0.0001; // !!! 最小サイズ !!!

    let http_client = Reqwest;
    let private_api = PrivateAPI::<Reqwest> { http_client };
    let response = private_api
        .order(
            &ExecutionType::Stop,
            &Symbol::Btc,
            &Side::Buy,
            size,
            Some(price),
        )
        .await?;

    // 執行数量条件, ロスカットレートを指定する場合。
    // use gmo_coin_rs::time_in_force::TimeInForce;
    // let losscut_rate = ...;
    // let response = private_api.order_with_options(&ExecutionType::Stop, &Symbol::Btc, &Side::Buy, size, Some(price), &TimeInForce::Fas, Some(losscut_rate))
    //                .await?;

    println!("注文ID: {}", response.order_id());
    println!("HTTPステータスコード: {}", response.http_status_code);
    println!("ステータスコード: {}", response.body.status);
    println!("APIを呼び出した時間: {}", response.body.responsetime,);
    Ok(())
}
