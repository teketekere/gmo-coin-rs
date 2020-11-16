use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::private::*;
use gmo_coin_rs::symbol::Symbol;

/// 建玉一覧を取得するAPIのExample
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
/// cargo run --example open_positions
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let http_client = Reqwest;
    let private_api = PrivateAPI::<Reqwest> { http_client };
    let response = private_api.open_positions(&Symbol::BtcJpy).await?;

    // 取得対象ページ、1ページ当たりの取得件数を指定する場合。
    // let page = 2;
    // let count = 30;
    // let response = private_api.open_positions_with_options(&Symbol::Btc, page, count).await?;

    println!("取得対象ページ: {}", response.current_page());
    println!("1ページ当たりの取得件数: {}\n", response.count());

    for position in response.open_positions() {
        println!("建玉ID: {}", position.position_id);
        println!("銘柄名: {}", position.symbol);
        println!("売買区分: {}", position.side);
        println!("建玉数量: {}", position.size);
        println!("発注中数量: {}", position.orderd_size);
        println!("建玉レート: {}", position.price);
        println!("評価損益 : {}", position.loss_gain);
        println!("レバレッジ : {}", position.leverage);
        println!("ロスカットレート : {}", position.losscut_price);
        println!("約定日時 : {}\n", position.timestamp);
    }

    println!("HTTPステータスコード: {}", response.http_status_code);
    println!("ステータスコード: {}", response.body.status);
    println!("APIを呼び出した時間: {}", response.body.responsetime,);
    Ok(())
}
