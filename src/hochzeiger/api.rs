use super::models::ApiData;
use std::fs;

pub async fn get_info() -> anyhow::Result<ApiData> {
    // NOTE: just because I don't have an internat connection in the fucking Austrian train
    // fetch_data_from_disk().await
    fetch_data_from_api().await
}

async fn fetch_data_from_api() -> anyhow::Result<ApiData> {
    println!("INFO: using data from disk because ÖBB can't give me fucking WiFi");
    let url = "https://winter.intermaps.com/hochzeiger/data?lang=en";
    let api_response = reqwest::get(url).await?;
    let api_data = api_response.json::<ApiData>().await?;
    Ok(api_data)
}

async fn fetch_data_from_disk() -> anyhow::Result<ApiData> {
    let content = fs::read_to_string("./data.json")?;
    let data = serde_json::from_str::<ApiData>(&content)?;
    Ok(data)
}
