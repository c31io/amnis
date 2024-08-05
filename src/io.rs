use bytes::Bytes;

use crate::{channel, Result};

#[async_trait::async_trait]
pub trait AmnisIO {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    async fn write(&mut self, buf: &[u8]) -> Result<usize>;
}

pub struct OutputChunk {
    channel: i32,
    line: i32,
    size: u64,
    payload: Bytes,
}

pub struct Output {
    inner: tokio::sync::mpsc::Receiver<OutputChunk>,
}

#[async_trait::async_trait]
impl AmnisIO for Output {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        unreachable!()
    }

    async fn write(&mut self, buf: &[u8]) -> Result<usize> {
        unreachable!()
    }
}
