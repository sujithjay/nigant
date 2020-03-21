extern crate reqwest;
use reqwest::Error;

pub async fn request(word: &String) -> Result<String, Error> {
    let app_id = "***REMOVED***";
    let app_key = "***REMOVED***";
    let url = format!("https://od-api.oxforddictionaries.com/api/v2/entries/en-us/{entry}",
        entry = word);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("app_id", app_id)
        .header("app_key", app_key)
        .send()
        .await?;

    Ok(response.text().await?)
}