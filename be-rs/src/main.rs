#![allow(unused_imports)]
use common_rs::*;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use log::{debug, error, info, trace, warn};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite;
use tungstenite::Message;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
 // let serialized = bincode::serialize(&mystruct).unwrap();
 // let deserialized: ResultItem = bincode::deserialize(&serialized).unwrap();

 pretty_env_logger::init_timed();
 let _warplog = warp::log("equipotent");

 info!("running initial query to pool connection...");
 do_query("hello").await.unwrap();

 warn!("warn enabled");
 info!("info enabled");
 debug!("debug enabled");
 trace!("trace enabled");

 let addr = "127.0.0.1:8080";

 let listener = TcpListener::bind(addr).await?;
 info!("Listening on: {}", addr);

 while let Ok((stream, _)) = listener.accept().await {
  tokio::spawn(accept_connection(stream));
 }

 Ok(())
}

async fn accept_connection(stream: TcpStream) {
 let addr = stream.peer_addr().unwrap();
 let (write, read) = tokio_tungstenite::accept_async(stream).await.unwrap().split();
 info!("New WebSocket connection: {}", addr);

 let (tx, rx) = futures_channel::mpsc::unbounded();

 let read_future = read.for_each(|msg| async {
  match msg.unwrap() {
   Message::Text(t) => {
    info!("query: {:?} start", t);
    let json = do_query(&t).await.unwrap();
    info!("query: {:?} response sent", t);

    tx.unbounded_send(Message::Text(json)).unwrap();
   }
   msg => {
    dbg!(msg);
   }
  };

  // let data = msg.unwrap().into_data();
 });

 let receive = rx.map(Ok).forward(write);
 pin_mut!(read_future, receive);
 future::select(read_future, receive).await;

 // read_future.await;
}

async fn do_query(query: &str) -> anyhow::Result<String> {
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

 let items = items.borrow();
 let items: Vec<_> = items.values().collect();
 Ok(serde_json::to_string(&items)?)
}
