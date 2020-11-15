use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::private::*;
use gmo_coin_rs::symbol::Symbol;

/// 最新の約定一覧を取得するAPIのExample
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
/// cargo run --example latest_executions
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = std::env::var("GMO_API_KEY")?;
    let secret_key = std::env::var("GMO_API_SECRET")?;

    let http_client = Reqwest;
    let private_api = PrivateAPI::<Reqwest> { http_client };
    let response = private_api
        .latest_executions(&api_key, &secret_key, &Symbol::BtcJpy)
        .await?;

    // 取得対象ページ、1ページ当たりの取得件数を取得する場合。
    // let page = 2;
    // let count = 30;
    // let response = private_api.latest_executions_with_options(&api_key, &secret_key, &Symbol::Btc, page, count).await?;

    println!("取得対象ページ: {}", response.current_page());
    println!("取得件数: {}\n", response.count());

    for execution in response.latest_executions() {
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
