#![allow(unused_imports)]
use common_rs::*;
use d_macro::*;
use futures_util::StreamExt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite;
use tungstenite::Message;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
 let mystruct = ResultItem { url: Some("imastring!!".to_string()), ..Default::default() };
 let serialized = bincode::serialize(&mystruct).unwrap();
 let deserialized: ResultItem = bincode::deserialize(&serialized).unwrap();
 // println!("{:?}", mystruct);
 // dbg!(serialized);
 // dbg!(&deserialized);
 assert_eq!(mystruct, deserialized);

 let _ = env_logger::try_init();
 let addr = "127.0.0.1:8080";

 let listener = TcpListener::bind(addr).await?;
 println!("Listening on: {}", addr);

 while let Ok((stream, _)) = listener.accept().await {
  tokio::spawn(accept_connection(stream));
 }

 Ok(())
}

async fn accept_connection(stream: TcpStream) {
 let addr = stream.peer_addr().unwrap();
 let (_write, read) = tokio_tungstenite::accept_async(stream).await.unwrap().split();
 println!("New WebSocket connection: {}", addr);

 let read_future = read.for_each(|msg| async {
  match msg.unwrap() {
   Message::Text(t) => {
    println!("query: {:?}", t);
    do_query(&t).await.unwrap();
    // write.send(Message::Text("{'event': 'ping'}\n".to_string())).await.unwrap();
   }
   msg => {
    dbg!(msg);
   }
  };

  // let data = msg.unwrap().into_data();
 });

 read_future.await;
}

async fn do_query(query: &str) -> anyhow::Result<()> {
 // let links_main = Selector::parse(r#".links_main"#).unwrap();
 // let result_snippet = Selector::parse(r#".result__snippet"#).unwrap();
 // let result_a = Selector::parse(r#".result__a"#).unwrap();

 let req_url = format!("https://html.duckduckgo.com/html?q={}", query);
 let agent = "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:73.0) Gecko/20100101 Firefox/73.0";

 let client = reqwest::Client::new();
 let req = client.get(&req_url).header(reqwest::header::USER_AGENT, agent);
 let res = req.send().await?;
 let bytes = res.bytes().await?;

 let text_buffer = Rc::new(RefCell::new(String::new()));
 let items = Rc::new(RefCell::new(HashMap::<String, ResultItem>::new()));
 let result_pos = Rc::new(RefCell::new(1i32));
 let scrapetime = std::time::SystemTime::now()
  .duration_since(std::time::SystemTime::UNIX_EPOCH)?
  .as_millis() as f64;

 let mut rewriter = lol_html::HtmlRewriter::new(
  lol_html::Settings {
   element_content_handlers: vec![
    lol_html::element!(".links_main a[href]", |el| {
     text_buffer.borrow_mut().clear();
     let text = text_buffer.clone();
     let attrs: HashMap<_, _> = el.attributes().iter().map(|a| (a.name(), a.value())).collect();
     let items = items.clone();
     let result_pos = result_pos.clone();
     let req_url = req_url.clone();
     let query = query.to_owned();

     el.on_end_tag(move |_end_tag| {
      let href = attrs["href"].as_str();
      if items.borrow().contains_key(href) == false {
       let parsed_href = url::Url::parse(href).unwrap();
       let host = parsed_href.host_str().unwrap();
       items.borrow_mut().insert(
        href.to_string(),
        ResultItem {
         url: Some(href.to_string()),
         host: Some(host.to_string()),
         source_query: Some(query.clone()),
         source_query_url: Some(req_url.clone()),
         source_result_pos: Some(*result_pos.borrow()),
         last_scraped: Some(scrapetime),
         ..Default::default()
        },
       );
       *result_pos.borrow_mut() += 1;
      }

      if let Some(c) = attrs.get("class") {
       let mut items = items.borrow_mut();
       let item = items.get_mut(href).unwrap();
       let text = Some(text.borrow().clone());
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
    lol_html::text!(".links_main a[href]", |t| {
     text_buffer.borrow_mut().push_str(t.as_str());
     Ok(())
    }),
   ],
   ..lol_html::Settings::default()
  },
  |_: &[u8]| {},
 );

 rewriter.write(&bytes)?;
 rewriter.end()?;

 dbg!(items);

 Ok(())
}
