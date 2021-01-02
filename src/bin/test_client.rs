use unix_socket_error::{TestMsg, transport::Transport, unix_socket_transport::UnixTransport, test_peer_loop};

#[tokio::main]
async fn main() {
    let server_transport: Box<dyn Transport<TestMsg, TestMsg>> = Box::new(UnixTransport::from_addr(&".test".into()).await.unwrap());

    test_peer_loop(server_transport).await;    
}