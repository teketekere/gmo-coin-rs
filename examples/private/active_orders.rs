use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::private::*;
use gmo_coin_rs::symbol::Symbol;

/// 有効注文一覧を取得するAPIのExample
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
/// cargo run --example orders
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = std::env::var("GMO_API_KEY")?;
    let secret_key = std::env::var("GMO_API_SECRET")?;

    let http_client = Reqwest;
    let private_api = PrivateAPI::<Reqwest> { http_client };
    let response = private_api
        .active_orders(&api_key, &secret_key, &Symbol::Btc)
        .await?;

    // 取得対象ページ、1ページ当たりの取得件数を取得する場合は次のようにします。
    // let page = 2;
    // let count = 30;
    // let response = private_api.active_orders_with_options(&api_key, &secret_key, &Symbol::Btc, page, count).await?;

    println!("取得対象ページ: {}", response.current_page());
    println!("1ページ当たりの取得件数: {}\n", response.count());

    for order in response.active_orders() {
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
