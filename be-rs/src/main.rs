// #![allow(unused_imports)]
use clap::Clap;
// use common_rs::*;
use futures::{SinkExt, StreamExt, TryFutureExt};
use log::{debug, error, info, trace, warn};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

mod ddg;
mod queries;

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

 info!("running initial query to pool connection...");
 ddg::do_query("hello").await.unwrap();

 warn!("warn enabled");
 info!("info enabled");
 debug!("debug enabled");
 trace!("trace enabled");

 // let routes = warp::any().map(|| Ok(warp::reply::html("hello"))).with(warp::log("routes"));

 let routes = warp::path("socket")
  .and(warp::ws())
  .map(|ws: warp::ws::Ws| ws.on_upgrade(accept_connection))
  .with(warp::log("routes"));

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
 let (tx, rx) = mpsc::unbounded_channel();
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
  if let Ok(t) = msg.to_str() {
   info!("query: {:?} start", t);
   let items = ddg::do_query(t).await.unwrap();
   info!("query: {:?} scraped {} results", t, items.len());
   let search_results = queries::scrape_search(t, items).await.unwrap();
   info!("query: {:?} response sent", t);
   tx.send(Message::text(serde_json::to_string(&search_results).unwrap())).unwrap();
  } else {
   error!("received non-text message: {:?}", msg);
  };
 }

 info!("accept_connection completed")
}
