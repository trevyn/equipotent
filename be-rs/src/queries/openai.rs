use common_rs::*;
use futures::StreamExt;
use reqwest::header;

pub async fn openai(query: String) -> anyhow::Result<Vec<SearchQueryResultItem>> {
 let request = serde_json::to_string(&serde_json::json!({
     "prompt": query,
     "max_tokens": 100,
 }))?;

 let mut stream = reqwest::Client::new()
  .post("https://api.openai.com/v1/engines/ada/completions")
  .header(header::AUTHORIZATION, concat!("Bearer ", include_str!("../../../credentials/openai")))
  .header(header::CONTENT_TYPE, "application/json")
  .body(request)
  .send()
  .await?
  .bytes_stream();

 while let Some(item) = stream.next().await {
  println!("Chunk: {:?}", item?);
 }

 Ok(vec![])
}
