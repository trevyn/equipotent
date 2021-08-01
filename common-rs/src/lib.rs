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
