use common_rs::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

thread_local! {
 static WS: RefCell<Option<WebSocket>> = RefCell::new(None);
}

macro_rules! console_log {
 ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
 #[wasm_bindgen(js_namespace = console)]
 fn log(s: &str);
}

#[wasm_bindgen(raw_module = "../../AppContents.svelte.js")]
extern "C" {
 fn set_json(json: String);
}

#[wasm_bindgen]
pub fn return_named_struct(inner: String) -> ResultItem {
 ResultItem { url: Some("jk!!".to_string()), title: Some(inner), ..Default::default() }
}

#[wasm_bindgen]
pub fn set_search(query: String) {
 console_log!("set_search: {:?}", query);
 WS.with(|ws| {
  if let Some(ws) = ws.borrow().as_ref() {
   if let Err(e) = ws.send_with_str(&query) {
    console_log!("websocket send err: {:?}", e);
   }
  }
 });
}

fn init_ws(ws: &WebSocket) {
 // For small binary messages, Arraybuffer is more efficient than Blob
 ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

 // onmessage callback
 let cloned_ws = ws.clone();
 let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
  if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
   console_log!("message event, received arraybuffer: {:?}", abuf);
   let array = js_sys::Uint8Array::new(&abuf);
   let len = array.byte_length() as usize;
   console_log!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());
   cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
   match cloned_ws.send_with_u8_array(&vec![5, 6, 7, 8]) {
    Ok(_) => console_log!("binary message successfully sent"),
    Err(err) => console_log!("error sending message: {:?}", err),
   }
  } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
   console_log!("message event, received blob: {:?}", blob);
   // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
   let fr = web_sys::FileReader::new().unwrap();
   let fr_c = fr.clone();
   let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
    let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
    let len = array.byte_length() as usize;
    console_log!("Blob received {} bytes: {:?}", len, array.to_vec());
    // here you can for example use the received image/png data
   }) as Box<dyn FnMut(web_sys::ProgressEvent)>);
   fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
   fr.read_as_array_buffer(&blob).expect("blob not readable");
   onloadend_cb.forget();
  } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
   // console_log!("message event, received Text: {:?}", txt);
   let json: String = txt.into();
   // let items: Vec<ResultItem> = serde_json::from_str(&json).unwrap();
   // console_log!("{:?}", items);
   set_json(json);
  } else {
   console_log!("message event, received Unknown: {:?}", e.data());
  }
 }) as Box<dyn FnMut(MessageEvent)>);
 ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
 onmessage_callback.forget();

 // onerror callback
 let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
  console_log!("error event: {:?}", e);
 }) as Box<dyn FnMut(ErrorEvent)>);
 ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
 onerror_callback.forget();

 // onopen callback
 // let cloned_ws = ws.clone();
 let onopen_callback = Closure::wrap(Box::new(move |_| {
  console_log!("socket opened");
  // cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]).unwrap();
 }) as Box<dyn FnMut(JsValue)>);
 ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
 onopen_callback.forget();
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
 console_error_panic_hook::set_once();
 let new_ws = WebSocket::new("ws://127.0.0.1:8080")?;

 WS.with(|ws| {
  init_ws(&new_ws);
  ws.replace(Some(new_ws));
 });

 Ok(())
}
