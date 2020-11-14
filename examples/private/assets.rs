use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::private::*;

/// 資産残高を取得するAPIのExample
///
/// # Example
///
/// 実行前に環境変数`GMO_API_KEY`, `GMO_API_SECRET`にGMOコインのAPIキー、APIシークレットを設定します。
/// Private APIは実際に注文などが行われます。実行する際は十分気を付けてください。
/// いかなる損害が発生しても当方は何ら責任を負いません。
/// 全て自己責任でお願いします。
///
/// ```
/// cargo build --examples
/// cargo run --example assets
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = std::env::var("GMO_API_KEY")?;
    let secret_key = std::env::var("GMO_API_SECRET")?;

    let http_client = Reqwest;
    let private_api = PrivateAPI::<Reqwest> { http_client };
    let response = private_api.assets(&api_key, &secret_key).await?;

    for asset in response.assets() {
        println!("銘柄名: {}", asset.symbol);
        println!("資産残高: {}", asset.amount);
        println!("資産残高[円]: {}円", asset.amount_as_jpy());
        println!("利用可能金額: {}", asset.available);
        println!("利用可能金額[円]: {}円", asset.available_as_jpy());
        println!("円転レート: {}\n", asset.conversionRate);
    }

    println!("HTTPステータスコード: {}", response.http_status_code);
    println!("ステータスコード: {}", response.body.status);
    println!("APIを呼び出した時間: {}", response.body.responsetime,);
    Ok(())
}
