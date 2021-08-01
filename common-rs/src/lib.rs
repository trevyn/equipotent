use serde::{Deserialize, Serialize};
#[cfg(feature = "turbosql")]
use turbosql::Turbosql;
#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "turbosql", derive(Turbosql))]
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
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

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(getter_with_clone))]
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

#[cfg(feature = "turbosql")]
#[derive(Turbosql, Clone, Default, Debug)]
pub struct HostAffection {
 pub rowid: Option<i64>,
 pub host: Option<String>,
 pub affection: Option<i32>,
}

#[cfg(feature = "turbosql")]
#[derive(Turbosql, Clone, Default, Debug)]
pub struct Bookmark {
 pub rowid: Option<i64>,
 pub url: Option<String>,
 pub timestamp: Option<f64>,
}
