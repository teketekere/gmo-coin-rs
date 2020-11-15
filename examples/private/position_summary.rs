use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::private::*;
use gmo_coin_rs::symbol::Symbol;

/// 建玉サマリーを取得するAPIのExample
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
/// cargo run --example position_summary
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let http_client = Reqwest;
    let private_api = PrivateAPI::<Reqwest> { http_client };
    let response = private_api.position_summary(&Symbol::BtcJpy).await?;

    for summary in response.position_summaries() {
        println!("平均建玉レート: {}", summary.average_position_rate);
        println!("評価損益: {}", summary.position_loss_gain);
        println!("売買区分: {}", summary.side);
        println!("発注中数量: {}", summary.sum_order_quantity);
        println!("建玉数量 : {}", summary.sum_position_quantity);
        println!("銘柄名 : {}\n", summary.symbol);
    }

    println!("HTTPステータスコード: {}", response.http_status_code);
    println!("ステータスコード: {}", response.body.status);
    println!("APIを呼び出した時間: {}", response.body.responsetime,);
    Ok(())
}
