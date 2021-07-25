#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[cfg_attr(feature = "turbosql", derive(turbosql::Turbosql))]
#[derive(Default)]
pub struct ExportedNamedStruct {
 pub rowid: Option<i64>,
 pub inner: Option<u32>,
 pub bigsigned: Option<i64>,
}
