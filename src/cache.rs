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
