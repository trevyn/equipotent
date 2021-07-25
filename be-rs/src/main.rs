use futures_util::StreamExt;
// use tokio::io::AsyncWriteExt;
// use log::info;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

 let (write, read) = ws_stream.split();

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
