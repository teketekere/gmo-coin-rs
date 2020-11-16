use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::private::*;

/// ロスカットレート変更APIのExample
///
/// # Example
///
/// 実行前に環境変数`GMO_COIN_API_KEY`, `GMO_COIN_API_SECRET`にGMOコインのAPIキー、APIシークレットを設定します。
/// また`GMO_COIN_POSITION_ID`で建玉IDを、`GMO_COIN_LOSSCUT_PRICE`でロスカットレートを指定します。
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
    let position_id = std::env::var("GMO_COIN_POSITION_ID")?;
    let losscut_price: i64 = std::env::var("GMO_COIN_LOSSCUT_PRICE")?.parse().unwrap();

    let http_client = Reqwest;
    let private_api = PrivateAPI::<Reqwest> { http_client };
    let response = private_api
        .change_losscut_price(&position_id, losscut_price)
        .await?;

    println!("HTTPステータスコード: {}", response.http_status_code);
    println!("ステータスコード: {}", response.body.status);
    println!("APIを呼び出した時間: {}", response.body.responsetime,);
    Ok(())
}
