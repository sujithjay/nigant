// Copyright 2020 Sujith Jay Nair
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::anyhow;
use anyhow::Result;
use config::Config;
use http::StatusCode;

pub async fn request(word: &String) -> Result<String> {
    let mut settings = Config::default();
    settings
        .merge(config::File::with_name("nigant.ini")).unwrap()
        .merge(config::Environment::with_prefix("NIGANT")).unwrap();
    let app_id = settings.get::<String>("NIGANT_APP_ID")?;
    let app_key = settings.get::<String>("NIGANT_APP_KEY")?;
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