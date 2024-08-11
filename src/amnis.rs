use bytes::Bytes;
use futures::stream::BoxStream;

use crate::gas::GasPlan;
use crate::io::OutputChunk;

#[async_trait::async_trait]
pub trait Amnis<'a> {
    fn new(gas_plan: GasPlan, input: BoxStream<'a, Bytes>) -> Self
    where
        Self: Sized;

    async fn handle(&mut self) -> BoxStream<OutputChunk>;
}
