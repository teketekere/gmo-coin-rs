use gmo_coin_rs::error::Error;
use gmo_coin_rs::http_client::Reqwest;
use gmo_coin_rs::public::status::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let http_client = Reqwest;
    let response = get_status(http_client).await?;

    println!(
        "http_status_code: {}, status_code: {}, resptime: {}, status: {}",
        response.http_status_code,
        response.body.status,
        response.body.responsetime,
        response.status(),
    );
    Ok(())
}
