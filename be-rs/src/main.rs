use futures_util::StreamExt;
// use tokio::io::AsyncWriteExt;
// use log::info;
use common_rs::*;
use d_macro::*;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
 // let links_main = Selector::parse(r#".links_main"#).unwrap();
 // let result_snippet = Selector::parse(r#".result__snippet"#).unwrap();
 // let result_a = Selector::parse(r#".result__a"#).unwrap();

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
 let addr = stream.peer_addr().expect("connected streams should have a peer address");
 println!("Peer address: {}", addr);

 let ws_stream = tokio_tungstenite::accept_async(stream)
  .await
  .expect("Error during the websocket handshake occurred");

 println!("New WebSocket connection: {}", addr);

 let (_write, read) = ws_stream.split();

 //  write
 //   .send(tokio_tungstenite::tungstenite::protocol::Message::Text(
 //    r#"{
 //   "event": "ping",
 //   "reqid": 42
 // }"#
 //     .to_string()
 //     + "\n",
 //   ))
 //   .await
 //   .unwrap();

 //  println!("sent");

 let read_future = read.for_each(|msg| async {
  match msg.unwrap() {
   Message::Text(t) => {
    println!("query: {:?}", t);
    do_query(&t).await.unwrap();
   }
   msg => {
    dbg!(msg);
   }
  };

  // let data = msg.unwrap().into_data();
  // println!("received {} bytes: {:?}", data.len(), data);
  // tokio::io::stdout().write(&data).await.unwrap();
 });

 read_future.await;

 // read.forward(write).await.expect("Failed to forward message")
}

async fn do_query(query: &str) -> anyhow::Result<()> {
 let req_url = format!("https://html.duckduckgo.com/html?q={}", query);
 let agent = "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:73.0) Gecko/20100101 Firefox/73.0";

 let client = reqwest::Client::new();
 let req = client.get(&req_url).header(reqwest::header::USER_AGENT, agent);
 let res = req.send().await?;
 let bytes = res.bytes().await?;

 let buffer = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

 let mut rewriter = lol_html::HtmlRewriter::new(
  lol_html::Settings {
   element_content_handlers: vec![
    lol_html::element!(".links_main a[href]", |el| {
     buffer.borrow_mut().clear();

     dbg!(el.attributes());

     let buffer = buffer.clone();
     el.on_end_tag(move |end| {
      let s = buffer.borrow();
      d!(s);
      dbg!(end);
      Ok(())
     })?;

     // let href = el.get_attribute("href").expect("href was required");
     // println!("");
     // println!("----- {}", &href);
     Ok(())
    }),
    lol_html::text!(".links_main a[href]", |t| {
     buffer.borrow_mut().push_str(t.as_str());
     Ok(())
    }),
   ],
   ..lol_html::Settings::default()
  },
  |_: &[u8]| {},
 );

 rewriter.write(bytes.as_ref())?;
 rewriter.end()?;

 Ok(())
}
