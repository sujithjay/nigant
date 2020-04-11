use anyhow::anyhow;
use anyhow::Result;
use dotenv_codegen::dotenv;
use http::StatusCode;

pub async fn request(word: &String) -> Result<String> {
    let app_id = dotenv!("APP_ID");
    let app_key = dotenv!("APP_KEY");
    let url = format!("https://od-api.oxforddictionaries.com/api/v2/entries/en-us/{entry}",
        entry = word);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("app_id", app_id)
        .header("app_key", app_key)
        .send()
        .await?;

    match response.status() {
        StatusCode::NOT_FOUND => Err(anyhow!("404")),
        _ => Ok(response.text().await?),
    }
}