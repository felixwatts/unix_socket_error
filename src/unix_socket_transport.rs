use crate::err::T2Result;

use super::transport::Transport;
use async_trait;
use serde::de::DeserializeOwned;
use std::{path::PathBuf};
use tokio::{prelude::*};
use tokio::net::UnixStream;

pub struct UnixTransport {
    socket: UnixStream,
    rx_buffer: Vec<u8>
}

impl UnixTransport {
    pub async fn from_addr(addr: &PathBuf) -> T2Result<UnixTransport> {
        let socket = UnixStream::connect(addr).await?;
        Ok(UnixTransport {
            socket,
            rx_buffer: vec![]
        })
    }

    pub fn from_socket(socket: UnixStream) -> UnixTransport {
        UnixTransport {
            socket,
            rx_buffer: vec![]
        }
    }
}

#[async_trait]
impl<TSend, TRecv> Transport<TSend, TRecv> for UnixTransport
where
    TRecv: DeserializeOwned + 'static,
    TSend: serde::Serialize + 'static + Send,
{
    async fn recv(&mut self) -> T2Result<TRecv> {
        let msg_len = self.socket.read_u32().await?;
        self.rx_buffer.clear();
        (&mut self.socket)
            .take(msg_len.into())
            .read_to_end(&mut self.rx_buffer)
            .await?;
        let msg: TRecv = serde_cbor::from_slice(&self.rx_buffer[..msg_len as usize])?;
        Ok(msg)
    }

    async fn send(&mut self, msg: TSend) -> T2Result<()> {
        let msg_buffer = serde_cbor::to_vec(&msg)?;
        let msg_len = msg_buffer.len();
        self.socket.write_u32(msg_len as u32).await?;
        self.socket.write_all(&msg_buffer).await?;
        Ok(())
    }
}