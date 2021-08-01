// #![allow(unused_imports)]
use clap::Clap;
use common_rs::*;
use futures::{SinkExt, StreamExt, TryFutureExt};
use log::{debug, error, info, trace, warn};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

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
 do_query("hello").await.unwrap();

 warn!("warn enabled");
 info!("info enabled");
 debug!("debug enabled");
 trace!("trace enabled");

 // let routes = warp::any().map(|| Ok(warp::reply::html("hello"))).with(warp::log("routes"));

 let routes = warp::path("socket")
  .and(warp::ws())
  .map(|ws: warp::ws::Ws| ws.on_upgrade(move |socket| accept_connection(socket)))
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
   let items = do_query(&t).await.unwrap();
   info!("query: {:?} response sent", t);
   tx.send(Message::text(serde_json::to_string(&items).unwrap())).unwrap();
  } else {
   error!("received non-text message: {:?}", msg);
  };
 }

 info!("accept_connection completed")
}

async fn do_query(query: &str) -> anyhow::Result<Vec<ResultItem>> {
 let req_url = format!("https://html.duckduckgo.com/html?q={}", query);
 let agent = "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:73.0) Gecko/20100101 Firefox/73.0";

 let client = reqwest::Client::new();
 let req = client.get(&req_url).header(reqwest::header::USER_AGENT, agent);
 let res = req.send().await?;
 let bytes = res.bytes().await?;

 let text_buffer = Rc::new(RefCell::new(String::new()));
 let items = Rc::new(RefCell::new(HashMap::<String, ResultItem>::new()));
 let result_pos = Rc::new(RefCell::new(0i32));
 let scrapetime = std::time::SystemTime::now()
  .duration_since(std::time::SystemTime::UNIX_EPOCH)?
  .as_millis() as f64;

 let mut rewriter = lol_html::HtmlRewriter::new(
  lol_html::Settings {
   element_content_handlers: vec![
    lol_html::text!(".links_main a[href]", |t| {
     text_buffer.borrow_mut().push_str(t.as_str());
     Ok(())
    }),
    lol_html::element!(".links_main a[href]", |el| {
     text_buffer.borrow_mut().clear();
     let text = text_buffer.clone();
     let attrs: HashMap<_, _> = el.attributes().iter().map(|a| (a.name(), a.value())).collect();
     let items = items.clone();
     let result_pos = result_pos.clone();
     let req_url = req_url.clone();
     let query = query.to_owned();

     el.on_end_tag(move |_end_tag| {
      let href = &attrs["href"];
      let mut items = items.borrow_mut();

      let item = items.entry(href.to_string()).or_insert_with(|| {
       let parsed_href = url::Url::parse(href).unwrap();
       let host = parsed_href.host_str().unwrap();
       let mut result_pos = result_pos.borrow_mut();
       *result_pos += 1;
       ResultItem {
        url: Some(href.to_string()),
        host: Some(host.to_string()),
        source_query: Some(query.clone()),
        source_query_url: Some(req_url.clone()),
        source_result_pos: Some(*result_pos),
        last_scraped: Some(scrapetime),
        ..Default::default()
       }
      });

      if let Some(c) = attrs.get("class") {
       let text = Some(html_escape::decode_html_entities(text.borrow().as_str()).to_string());
       match c.as_str() {
        "result__a" => item.title = text,
        "result__snippet" => item.snippet = text,
        _ => (),
       }
      };

      Ok(())
     })?;

     Ok(())
    }),
   ],
   ..lol_html::Settings::default()
  },
  |_: &[u8]| {},
 );

 rewriter.write(&bytes)?;
 rewriter.end()?;

 Ok(Rc::try_unwrap(items).unwrap().into_inner().into_values().collect())
}
