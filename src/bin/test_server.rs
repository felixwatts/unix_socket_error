use unix_socket_error::transport::{Transport};
use unix_socket_error::unix_socket_transport::UnixTransport;
use tokio::stream::StreamExt;
use tokio::{net::UnixListener};
use unix_socket_error::TestMsg;

#[tokio::main]
async fn main() {
    let _ = std::fs::remove_file(".test");
    let mut listener = UnixListener::bind(".test").unwrap();
    let client_stream = listener.next().await.unwrap().unwrap();
    let client_transport: Box<dyn Transport<TestMsg, TestMsg>> = Box::new(UnixTransport::from_socket(client_stream));

    unix_socket_error::test_peer_loop(client_transport).await;
}