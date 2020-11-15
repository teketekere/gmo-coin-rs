use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::private::*;

/// 約定情報を取得するAPIのExample
///
/// # Example
///
/// 実行前に環境変数`GMO_API_KEY`, `GMO_API_SECRET`にGMOコインのAPIキー、APIシークレットを設定します。
/// また`GMO_ORDER_ID`に"1234567..."という形式で取得する注文のIDを設定します。
///
/// Private APIは実際に注文などが行われます。実行する際は十分気を付けてください。
/// いかなる損害が発生しても当方は何ら責任を負いません。
/// 全て自己責任でお願いします。
///
/// ```
/// cargo build --examples
/// cargo run --example executions
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = std::env::var("GMO_API_KEY")?;
    let secret_key = std::env::var("GMO_API_SECRET")?;
    let order_id = std::env::var("GMO_ORDER_ID")?;

    let http_client = Reqwest;
    let private_api = PrivateAPI::<Reqwest> { http_client };
    let response = private_api
        .executions_with_order_id(&api_key, &secret_key, &order_id)
        .await?;

    // 約定IDを指定して約定情報を取得する場合。
    // let execition_id = "1234...";
    // let response = private_api.executions_with_execution_id(&api_key, &secret_key, &execution_id).await?;

    for execution in response.executions() {
        println!("約定ID: {}", execution.execution_id);
        println!("注文ID: {}", execution.order_id);
        println!("銘柄名: {}", execution.symbol);
        println!("売買区分: {}", execution.side);
        println!("決済区分: {}", execution.settle_type);
        println!("約定数量: {}", execution.size);
        println!("約定レート: {}", execution.price);
        println!("決済損益 : {}", execution.loss_gain);
        println!("取引手数料 : {}", execution.fee);
        println!("約定日時 : {}\n", execution.timestamp);
    }

    println!("HTTPステータスコード: {}", response.http_status_code);
    println!("ステータスコード: {}", response.body.status);
    println!("APIを呼び出した時間: {}", response.body.responsetime,);
    Ok(())
}
