mod utils;

use wasm_bindgen::prelude::*;

use common_rs::ExportedNamedStruct;

#[wasm_bindgen]
pub fn return_named_struct(inner: u32) -> ExportedNamedStruct {
 ExportedNamedStruct { inner, ..Default::default() }
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
