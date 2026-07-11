use std::error::Error;

use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://google.com/";

    let body = reqwest::get(url).await?.text().await?;

    println!("body = {body:?}");

    Ok(())
}
