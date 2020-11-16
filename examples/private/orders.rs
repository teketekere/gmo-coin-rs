use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::private::*;

/// 注文情報を取得するAPIのExample
///
/// # Example
///
/// 実行前に環境変数`GMO_COIN_API_KEY`, `GMO_COIN_API_SECRET`にGMOコインのAPIキー、APIシークレットを設定します。
/// また`GMO_COIN_ORDER_IDS`に"orderid1,orderid2,..."という形式で取得する注文のIDを設定します。
///
/// Private APIは実際に注文などが行われます。実行する際は十分気を付けてください。
/// いかなる損害が発生しても当方は何ら責任を負いません。
/// 全て自己責任でお願いします。
///
/// ```
/// cargo build --examples
/// cargo run --example orders
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let order_id: String = std::env::var("GMO_COIN_ORDER_IDS")?;
    let order_ids: Vec<&str> = order_id.split(',').collect();

    let http_client = Reqwest;
    let private_api = PrivateAPI::<Reqwest> { http_client };
    let response = private_api.orders(&order_ids).await?;

    for order in response.orders() {
        println!("親注文ID: {}", order.root_order_id);
        println!("注文ID: {}", order.order_id);
        println!("銘柄名: {}", order.symbol);
        println!("売買区分: {}", order.side);
        println!("取引区分: {}", order.order_type);
        println!("注文タイプ: {}", order.execution_type);
        println!("決済区分: {}", order.settle_type);
        println!("発注数量: {}", order.size);
        println!("約定数量: {}", order.executed_size);
        println!("注文価格: {}", order.price);
        println!("ロスカットレート : {}", order.losscut_price);
        println!("注文ステータス : {}", order.status);
        println!("取消区分 : {}", order.cancel_type);
        println!("執行数量条件 : {}", order.time_in_force);
        println!("注文日時 : {}\n", order.timestamp);
    }

    println!("HTTPステータスコード: {}", response.http_status_code);
    println!("ステータスコード: {}", response.body.status);
    println!("APIを呼び出した時間: {}", response.body.responsetime,);
    Ok(())
}
