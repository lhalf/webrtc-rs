use std::net::TcpListener;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let address = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:6969".to_string());
    let socket = TcpListener::bind(&address).await;
    let listener = socket.expect("Failed to bind");
    println!("Listening on: {}", address);
    while let Ok((stream, address)) = listener.accept().await {
        tokio::spawn(handle_connection)
    }
}
