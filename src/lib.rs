#[macro_use]
extern crate async_trait;

use transport::Transport;
use serde::{Serialize, Deserialize};
use tokio::stream::StreamExt;

pub mod err;
pub mod transport;
pub mod unix_socket_transport;

#[derive(Serialize, Deserialize)]
pub struct TestMsg {
    id: u32,
    payload: Vec<i32>
}

pub async fn test_peer_loop(mut transport: Box<dyn Transport<TestMsg, TestMsg>>) {
    let mut short_period = tokio::time::interval(tokio::time::Duration::from_millis(5));

    let mut next_rx_msg_id = 0u32;
    let mut next_tx_msg_id = 0u32;

    let mut payload = vec![];
    for i in 0..2048 {
        payload.push(i);
    }

    loop {
        tokio::select!{
            msg = transport.recv() => {
                let msg = msg.unwrap();
                assert_eq!(next_rx_msg_id, msg.id);
                next_rx_msg_id += 1;
            },
            _ = short_period.next() => {
                let msg = TestMsg{
                    id: next_tx_msg_id,
                    payload: payload.clone()
                };
                transport.send(msg).await.unwrap();
                next_tx_msg_id += 1;
            },
        }
    }
}