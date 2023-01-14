use reqwest;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.rustinaction.com/";
    let mut response = reqwest::get(url).await?;

    let content = response.text().await?;
    println!("{}", content);

    Ok(())
}
