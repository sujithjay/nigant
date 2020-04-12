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

use crate::parser::Payload;

use anyhow::Result;
use cacache;
use bincode;

pub async fn load(payload: &Payload) -> Result<()> {
    let home: String = std::env::var("HOME").unwrap();
    let dir: String = home + "/.nigant";
    let ser = bincode::serialize(&payload).unwrap();
    cacache::write(&dir, &payload.id, ser).await?;
    Ok(())
}


pub async fn fetch(id: &String) -> Result<Payload> {
    let home: String = std::env::var("HOME").unwrap();
    let dir: String = home + "/.nigant";
    let data = cacache::read(&dir, id).await?;
    let deser = bincode::deserialize::<Payload>(&data);
    Ok(deser.unwrap())
}
