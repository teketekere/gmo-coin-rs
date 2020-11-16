use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::private::*;
use gmo_coin_rs::symbol::Symbol;

/// 注文の一括キャンセルAPIのExample
///
/// # Example
///
/// 実行前に環境変数`GMO_COIN_API_KEY`, `GMO_COIN_API_SECRET`にGMOコインのAPIキー、APIシークレットを設定します。
///
/// Private APIは実際に注文などが行われます。実行する際は十分気を付けてください。
/// いかなる損害が発生しても当方は何ら責任を負いません。
/// 全て自己責任でお願いします。
///
/// ```
/// cargo build --examples
/// cargo run --example cancel_bulk_order
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let symbols = vec![&Symbol::Btc, &Symbol::Eth];

    let http_client = Reqwest;
    let private_api = PrivateAPI::<Reqwest> { http_client };
    let response = private_api.cancel_bulk_order(&symbols).await?;

    // オプションを指定する場合。
    // use gmo_coin_rs::settle_type::SettleType;
    // use gmo_coin_rs::side::Side;
    //
    // let response = private_api
    //     .cancel_bulk_order_with_options(
    //         &symbols,
    //         Some(&Side::Buy),
    //         Some(&SettleType::Open),
    //         Some(true),
    //     )
    //     .await?;

    for id in response.order_ids() {
        println!("注文Id: {}", id);
    }

    println!("HTTPステータスコード: {}", response.http_status_code);
    println!("ステータスコード: {}", response.body.status);
    println!("APIを呼び出した時間: {}", response.body.responsetime,);
    Ok(())
}
