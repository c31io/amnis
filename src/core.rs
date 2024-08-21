use bytes::Bytes;
use futures::stream::BoxStream;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::mpsc::{self, Sender};

use crate::{
    channel::Channel,
    gas::{Gas, GasPlan},
    io::OutputChunk,
    Amnis, Variable,
};

pub struct AmnisState {
    gas_plan: GasPlan,
    gas_used: Gas,
    variables: HashMap<String, Variable>,
    channels: HashMap<i32, Channel>,
}

pub struct AmnisCore {
    state: Arc<Mutex<AmnisState>>,
    input: Arc<Mutex<BoxStream<'static, Bytes>>>,
}

#[async_trait::async_trait]
impl Amnis for AmnisCore {
    fn new(gas_plan: GasPlan, input: BoxStream<'static, Bytes>) -> Self {
        AmnisCore {
            state: Arc::new(Mutex::new(AmnisState {
                gas_plan,
                gas_used: Gas::zero(),
                variables: HashMap::new(),
                channels: HashMap::from([(1, Channel::new())]),
            })),
            input: Arc::new(Mutex::new(input)),
        }
    }

    /// Handle a bytes stream.
    async fn handle(&self) -> BoxStream<OutputChunk> {
        // Channel for muxing outputs
        // TODO config channel size somewhere, some compile flag I guess
        let (tx, mut rx) = mpsc::channel::<OutputChunk>(32);
        // Feed inputs to channels
        let state = self.state.clone();
        let input = self.input.clone();
        tokio::spawn((|| async move {
            drop(tx);
            drop(state);
            drop(input);
        })());
        // Early return the output stream
        Box::pin(async_stream::stream! {
            while let Some(item) = rx.recv().await {
                yield item;
            }
        })
    }
}
