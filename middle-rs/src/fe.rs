use super::*;
use futures::StreamExt;
use futures::{channel::mpsc::UnboundedSender, SinkExt};
use std::{cell::RefCell, collections::HashMap};
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use ws_stream_wasm::WsMessage;

#[derive(Default)]
struct Globals {
 channel_tx: Option<UnboundedSender<String>>,
 next_txid: i64,
 senders: HashMap<i64, UnboundedSender<Card>>,
}

thread_local! {
 static G: RefCell<Globals> = RefCell::new(Globals::default());
}

macro_rules! console_log {
 ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
 #[wasm_bindgen(js_namespace = console)]
 fn log(s: &str);
}

#[wasm_bindgen]
impl Card {
 // pub fn new() -> Card {
 //  Card { rowid: None, question: Some("q".to_string()), answer: Some("a".to_string()) }
 // }
 pub async fn get(rowid: i64) -> Card {
  send_ws(Command::GetCard { rowid }).await
 }
 pub async fn set_question(rowid: i64, question: String) {
  //execute!("UPDATE card SET question = ? WHERE rowid = ?", question, rowid).unwrap();
  send_ws(Command::SetCardQuestion { rowid, question }).await;
 }
 pub async fn set_answer(rowid: i64, answer: String) {
  send_ws(Command::SetCardAnswer { rowid, answer }).await;
 }
 // pub fn save(&self) {}
 // pub fn delete(rowid: i64) {
 //  let _ = rowid;
 // }
 // pub fn list() -> Vec<i64> {
 //  Vec::new()
 // }
}

async fn send_ws(cmd: Command) -> Card {
 console_log!("send_ws: {:?}", cmd);

 let (resp_tx, mut resp_rx) = futures::channel::mpsc::unbounded();

 let (mut channel_tx, txid) = G.with(|g| -> (_, _) {
  let mut g = g.borrow_mut();
  let txid = g.next_txid;
  g.senders.insert(txid, resp_tx);
  g.next_txid += 1;
  (g.channel_tx.clone().unwrap(), txid)
 });

 let wrapped_cmd = WrappedCommand { txid, cmd };
 channel_tx.send(serde_json::to_string(&wrapped_cmd).unwrap()).await.unwrap();
 resp_rx.next().await.unwrap()
}

#[allow(dead_code)]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
 console_error_panic_hook::set_once();

 turbosql::set_db_path(std::path::Path::new(":memory:")).unwrap();
 console_log!(
  "insert: {:?}",
  ResultItem { url: Some("myurl".to_string()), ..Default::default() }.insert()
 );
 console_log!("select: {:?}", turbosql::select!(Vec<ResultItem>));

 console_log!("connecting");

 let (_ws, wsio) =
  ws_stream_wasm::WsMeta::connect("ws://127.0.0.1:8080/socket", None).await.unwrap();

 console_log!("connected");

 let (mut ws_tx, mut ws_rx) = wsio.split();
 let (channel_tx, mut channel_rx) = futures::channel::mpsc::unbounded();

 G.with(|g| {
  g.borrow_mut().channel_tx = Some(channel_tx);
 });

 spawn_local(async move {
  while let Some(msg) = ws_rx.next().await {
   if let WsMessage::Text(msg) = msg {
    let Response { txid, resp } = serde_json::from_str(&msg).unwrap();
    let mut sender = G.with(|g| -> _ { g.borrow().senders.get(&txid).unwrap().clone() });
    sender.send(resp).await.unwrap();
   }
  }
  console_log!("ws_rx ENDED");
 });

 spawn_local(async move {
  while let Some(msg) = channel_rx.next().await {
   ws_tx.send(WsMessage::Text(msg)).await.unwrap();
  }
  console_log!("rx ENDED");
 });

 Ok(())
}
