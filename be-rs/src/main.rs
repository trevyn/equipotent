use futures_util::StreamExt;
// use tokio::io::AsyncWriteExt;
// use log::info;
use common_rs::*;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
 let req_url = format!("https://html.duckduckgo.com/html?q={}", "banana");
 let agent = "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:73.0) Gecko/20100101 Firefox/73.0";

 let client = reqwest::Client::new();
 let req = client.get(&req_url).header(reqwest::header::USER_AGENT, agent);
 let res = req.send().await?;
 let bytes = res.bytes().await?;

 let mut rewriter = lol_html::HtmlRewriter::new(
  lol_html::Settings {
   element_content_handlers: vec![
    lol_html::element!("a[href]", |el| {
     let href = el.get_attribute("href").expect("href was required");
     println!("");
     println!("----- {}", &href);
     Ok(())
    }),
    lol_html::text!("a[href]", |el| {
     print!("{}", el.as_str());
     Ok(())
    }),
   ],
   ..lol_html::Settings::default()
  },
  |_: &[u8]| {},
 );

 rewriter.write(bytes.as_ref())?;
 rewriter.end()?;

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
 let addr = "127.0.0.1:8080".to_string();

 // Create the event loop and TCP listener we'll accept connections on.
 let try_socket = TcpListener::bind(&addr).await;
 let listener = try_socket.expect("Failed to bind");
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

 let read_future = read.for_each(|message| async {
  let data = message.unwrap().into_data();
  println!("received {} bytes: {:?}", data.len(), data);
  // tokio::io::stdout().write(&data).await.unwrap();
 });

 read_future.await;

 // read.forward(write).await.expect("Failed to forward message")
}
