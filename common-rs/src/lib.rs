use serde::{Deserialize, Serialize};
use turbosql::Turbosql;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Turbosql, Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct ResultItem {
 pub rowid: Option<i64>,
 pub url: Option<String>,
 pub host: Option<String>,
 pub title: Option<String>,
 pub snippet: Option<String>,
 pub source_query: Option<String>,
 pub source_query_url: Option<String>,
 pub source_result_pos: Option<i32>,
 pub last_scraped: Option<f64>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct SearchQueryResultItem {
 pub search_highlighted_url: String,
 pub title: String,
 pub snippet: String,
 pub url: String,
 pub host: String,
 pub bookmarked: bool,
 pub hostaffection: i32,
 pub rank: f64,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CommandType {
 SearchScrape,
 SearchInstant,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Command {
 pub command: CommandType,
 pub param: String,
}

#[wasm_bindgen]
impl Command {
 pub fn new(command: CommandType, param: String) -> Command {
  Command { command, param }
 }
}

#[derive(Turbosql, Clone, Default, Debug)]
pub struct HostAffection {
 pub rowid: Option<i64>,
 pub host: Option<String>,
 pub affection: Option<i32>,
}

#[derive(Turbosql, Clone, Default, Debug)]
pub struct Bookmark {
 pub rowid: Option<i64>,
 pub url: Option<String>,
 pub timestamp: Option<f64>,
}
