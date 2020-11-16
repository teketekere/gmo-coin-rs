use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::private::*;

/// 複数の注文キャンセルAPIのExample
///
/// # Example
///
/// 実行前に環境変数`GMO_COIN_API_KEY`, `GMO_COIN_API_SECRET`にGMOコインのAPIキー、APIシークレットを設定します。
/// また`GMO_COIN_ORDER_IDS`で注文IDを指定します。
///
/// Private APIは実際に注文などが行われます。実行する際は十分気を付けてください。
/// いかなる損害が発生しても当方は何ら責任を負いません。
/// 全て自己責任でお願いします。
///
/// ```
/// cargo build --examples
/// cargo run --example cancel_orders
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let order_ids_str = std::env::var("GMO_COIN_ORDER_IDS")?;
    let order_ids = order_ids_str.split(",").collect::<Vec<&str>>();

    let http_client = Reqwest;
    let private_api = PrivateAPI::<Reqwest> { http_client };
    let response = private_api.cancel_orders(&order_ids).await?;

    println!("キャンセルに成功した注文:");
    for id in response.success() {
        println!("注文Id: {}", id);
    }

    println!("\nキャンセルに失敗した注文:");
    for failed_order in response.failed() {
        println!("注文Id: {}", failed_order.order_id);
        println!("エラーコード: {}", failed_order.message_code);
        println!("エラーメッセージ: {}\n", failed_order.message_string);
    }

    println!("HTTPステータスコード: {}", response.http_status_code);
    println!("ステータスコード: {}", response.body.status);
    println!("APIを呼び出した時間: {}", response.body.responsetime,);
    Ok(())
}
