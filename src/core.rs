use bytes::Bytes;
use futures::stream::BoxStream;
use std::collections::HashMap;
use tokio::sync::mpsc::{self, Sender};

use crate::{
    channel::Channel,
    gas::{Gas, GasPlan},
    io::OutputChunk,
    Amnis, Variable,
};

pub struct AmnisCore<'a> {
    gas_plan: GasPlan,
    gas_used: Gas,
    variables: HashMap<String, Variable>,
    channels: HashMap<i32, Channel>,
    input: BoxStream<'a, Bytes>,
}

#[async_trait::async_trait]
impl<'a> Amnis<'a> for AmnisCore<'a> {
    fn new(gas_plan: GasPlan, input: BoxStream<'a, Bytes>) -> Self {
        AmnisCore {
            gas_plan,
            gas_used: Gas::zero(),
            variables: HashMap::new(),
            channels: HashMap::from([(0, Channel::new())]),
            input,
        }
    }

    /// Handle a bytes stream.
    async fn handle(&mut self) -> BoxStream<OutputChunk> {
        // Channel for muxing outputs
        // TODO config channel size somewhere, some compile flag I guess
        let (tx, mut rx) = mpsc::channel::<OutputChunk>(128);
        // Fire channels and give them tx.clone()
        tokio::spawn(AmnisCore::run(tx));
        // Early return the output stream
        Box::pin(async_stream::stream! {
            while let Some(item) = rx.recv().await {
                yield item;
            }
        })
    }
}

impl AmnisCore<'_> {
    async fn run(tx: Sender<OutputChunk>) {}
}
