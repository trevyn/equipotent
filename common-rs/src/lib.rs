#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
#[derive(Default)]
#[cfg(feature = "turbosql")]
#[derive(turbosql::Turbosql)]
pub struct ExportedNamedStruct {
 pub rowid: Option<i64>,
 pub inner: Option<u32>,
 pub bigsigned: Option<i64>,
}
