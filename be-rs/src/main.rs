// #![allow(unused_imports)]
use anyhow::Context;
use clap::Clap;
use futures::{SinkExt, StreamExt, TryFutureExt};
use log::{debug, error, info, trace, warn};
use middle_rs::*;
use tokio_stream::wrappers::UnboundedReceiverStream;
use turbosql::{execute, select};
use warp::ws::{Message, WebSocket};
use warp::Filter;

mod ddg;
mod queries;

#[derive(rust_embed::RustEmbed)]
#[folder = "../fe-svelte/build"]
struct Asset;

#[derive(Clap, Debug)]
struct Opts {
 #[clap(short, long)]
 cert_path: Option<String>,
 #[clap(short, long)]
 key_path: Option<String>,
 #[clap(short, long, default_value = "8080")]
 port: u16,
 // #[clap(long)]
 // password: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
 // let serialized = bincode::serialize(&mystruct).unwrap();
 // let deserialized: ResultItem = bincode::deserialize(&serialized).unwrap();

 pretty_env_logger::init_timed();
 let opts = Opts::parse();

 tokio::task::spawn(async {
  info!("running initial query to pool connection...");
  ddg::do_query("hello").await.unwrap();
 });

 warn!("warn enabled");
 info!("info enabled");
 debug!("debug enabled");
 trace!("trace enabled");

 // socket ----------------
 let routes = warp::path("socket")
  .and(warp::ws())
  .map(|ws: warp::ws::Ws| ws.on_upgrade(accept_connection))
  // asset -----------------
  .or(warp::any().and(warp::path::full()).map(|path: warp::path::FullPath| {
   match (|| -> anyhow::Result<_> {
    let path = match path.as_str() {
     "/" => "index.html",
     "/index_bg.wasm" => "dist/middle-rs/index_bg.wasm",
     p => p.trim_start_matches('/'),
    };
    let data = match path {
     "favicon.ico" => Vec::new(),
     p => Asset::get(p).context("Expected Asset")?.data.into_owned(),
    };
    let mime = mime_guess::from_path(path).first().unwrap();
    Ok(warp::reply::with_header(data, "content-type", mime.essence_str()))
   })() {
    Ok(body) => body,
    Err(e) => panic!("{}", e), //warp::reply::html("error!".to_string()),
   }
  }))
  .with(warp::log("be_rs::routes"));

 match (opts.key_path, opts.cert_path) {
  (Some(key_path), Some(cert_path)) => {
   eprintln!("Serving HTTPS on port {}", opts.port);
   warp::serve(routes)
    .tls()
    .cert_path(cert_path)
    .key_path(key_path)
    .run(([0, 0, 0, 0], opts.port))
    .await;
  }
  (None, None) => {
   eprintln!("Serving (unsecured) HTTP on port {}", opts.port);
   warp::serve(routes).run(([0, 0, 0, 0], opts.port)).await;
  }
  _ => panic!("Both key-path and cert-path must be specified for HTTPS."),
 }

 Ok(())
}

async fn accept_connection(ws: WebSocket) {
 let (mut ws_tx, mut ws_rx) = ws.split();
 let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
 let mut rx = UnboundedReceiverStream::new(rx);

 tokio::task::spawn(async move {
  while let Some(msg) = rx.next().await {
   ws_tx
    .send(msg)
    .unwrap_or_else(|e| {
     error!("websocket send error: {}", e);
    })
    .await;
  }
 });

 while let Some(result) = ws_rx.next().await {
  let msg = match result {
   Ok(msg) => msg,
   Err(e) => {
    error!("websocket error: {}", e);
    break;
   }
  };
  let WrappedCommand { txid, cmd } = bincode::deserialize(msg.as_bytes()).unwrap();
  info!("txid {}: {:?}", txid, cmd);
  match cmd {
   Command::GetCard { rowid } => {
    let card = match select!(Option<Card> "WHERE rowid = ?", rowid).unwrap() {
     Some(card) => card,
     None => Card { rowid: Some(rowid), ..Default::default() },
    };
    let resp = Response { txid, resp: card };
    tx.send(Message::binary(bincode::serialize(&resp).unwrap())).unwrap();
   }
   Command::SetCardQuestion { rowid, question } => {
    execute!(
      "INSERT INTO card(rowid, question) VALUES(?, ?) ON CONFLICT DO UPDATE SET question = excluded.question",
      rowid,
      question
     )
     .unwrap();
   }
   Command::SetCardAnswer { rowid, answer } => {
    execute!(
      "INSERT INTO card(rowid, answer) VALUES(?, ?) ON CONFLICT DO UPDATE SET answer = excluded.answer",
      rowid,
      answer
     )
     .unwrap();
   }
  }
  //  match command {
  //   Command { command: CommandType::SearchScrape, param: query } => {
  //    info!("SearchScrape: {:?} start", query);
  //    let items = ddg::do_query(&query).await.unwrap();
  //    info!("SearchScrape: {:?} scraped {} results", query, items.len());
  //    let search_results = queries::search_scrape(&query, items).await.unwrap();
  //    info!("SearchScrape: {:?} response sent", query);
  //    tx.send(Message::text(serde_json::to_string(&search_results).unwrap())).unwrap();
  //   }
  //   Command { command: CommandType::SearchInstant, param: query } => {
  //    info!("SearchInstant: {:?}", query);
  //    let search_results = queries::search_instant(query.clone()).await.unwrap();
  //    info!("SearchInstant: {:?} response sent", query);
  //    tx.send(Message::text(serde_json::to_string(&search_results).unwrap())).unwrap();
  //   }
  //   Command { command: CommandType::OpenAi, param: query } => {
  //    info!("OpenAI: {:?}", query);
  //    let _search_results = queries::openai(query.clone()).await.unwrap();
  //    // info!("SearchInstant: {:?} response sent", query);
  //    // tx.send(Message::text(serde_json::to_string(&search_results).unwrap())).unwrap();
  //   }
  //  }
 }

 info!("accept_connection completed")
}
