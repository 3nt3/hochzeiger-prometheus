use models::ApiData;

mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://winter.intermaps.com/hochzeiger/data?lang=en";
    let api_data: ApiData = reqwest::get(url).await?.json().await?;
    dbg!(api_data);

    Ok(())
}
