use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::public::*;

/// 取引所ステータスを取得するAPIのExample
///
/// # Example
/// ```
/// cargo build --examples
/// cargo run --example status
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    let http_client = Reqwest;
    let public_api = PublicAPI::<Reqwest> { http_client };
    let response = public_api.status().await?;

    println!("取引所ステータス: {}", response.status());

    println!("HTTPステータスコード: {}", response.http_status_code);
    println!("ステータスコード: {}", response.body.status);
    println!("APIを呼び出した時間: {}", response.body.responsetime,);
    Ok(())
}
