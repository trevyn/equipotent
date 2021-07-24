#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
#[derive(Default)]
pub struct ExportedNamedStruct {
 pub inner: u32,
 pub bigunsigned: u64,
 pub bigsigned: i64,
}
