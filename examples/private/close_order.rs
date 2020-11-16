use gmo_coin_rs::error::Error;
use gmo_coin_rs::execution_type::ExecutionType;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::private::*;
use gmo_coin_rs::side::Side;
use gmo_coin_rs::symbol::Symbol;

/// 決済注文APIのExample
///
/// # Example
///
/// 実行前に環境変数`GMO_COIN_API_KEY`, `GMO_COIN_API_SECRET`にGMOコインのAPIキー、APIシークレットを設定します。
/// また`GMO_COIN_POSITION_ID`で決済を行う建玉IDを指定します。
///
/// Private APIは実際に注文などが行われます。実行する際は十分気を付けてください。
/// いかなる損害が発生しても当方は何ら責任を負いません。
/// 全て自己責任でお願いします。
///
/// ```
/// cargo build --examples
/// cargo run --example close_order
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let position_id = std::env::var("GMO_COIN_POSITION_ID")?;
    let size = 0.1; // !!! EthJpyの最小サイズ !!!
    let price = None; // 成行の場合は価格はなし。指値、逆指値の場合は価格を指定する。

    let http_client = Reqwest;
    let private_api = PrivateAPI::<Reqwest> { http_client };
    let response = private_api
        .close_order(
            &ExecutionType::Market,
            &Symbol::EthJpy,
            &Side::Sell,
            size,
            price,
            &position_id,
        )
        .await?;

    // 執行数量条件を指定する場合。
    // use gmo_coin_rs::time_in_force::TimeInForce;
    // let response = private_api
    //     .close_order_with_options(
    //         &ExecutionType::Market,
    //         &Symbol::EthJpy,
    //         &Side::Buy,
    //         size,
    //         price,
    //         &position_id,
    //         &TimeInForce::Fak,
    //     )
    //     .await?;

    println!("注文ID: {}", response.order_id());
    println!("HTTPステータスコード: {}", response.http_status_code);
    println!("ステータスコード: {}", response.body.status);
    println!("APIを呼び出した時間: {}", response.body.responsetime,);
    Ok(())
}
