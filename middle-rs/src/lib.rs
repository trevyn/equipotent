use serde::{Deserialize, Serialize};
use turbosql::Turbosql;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
mod fe;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Command {
 // SearchScrape { query: String },
 // SearchInstant { query: String },
 // OpenAi { query: String },
 GetCard { rowid: i64 },
 SetCardQuestion { rowid: i64, question: String },
 SetCardAnswer { rowid: i64, answer: String },
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Turbosql, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Card {
 pub rowid: Option<i64>,
 pub question: Option<String>,
 pub answer: Option<String>,
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
