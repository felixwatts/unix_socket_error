use async_trait;

use crate::err::T2Result;

#[async_trait]
pub trait Transport<TSend, TRecv>: Send {
    async fn recv(&mut self) -> T2Result<TRecv>;
    async fn send(&mut self, msg: TSend) -> T2Result<()>;
}