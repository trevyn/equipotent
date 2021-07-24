mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ExportedNamedStruct {
 pub inner: u32,
}

#[wasm_bindgen]
pub fn return_named_struct(inner: u32) -> ExportedNamedStruct {
 ExportedNamedStruct { inner }
}

#[wasm_bindgen]
extern "C" {
 fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
 a + b
}

#[wasm_bindgen]
pub fn greet() {
 alert("Hello, mywasm!");
}
