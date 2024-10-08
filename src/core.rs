use std::{
    collections::HashMap, pin::Pin, sync::{Arc, Mutex}
};
use tokio::{io::AsyncRead, sync::mpsc};

use crate::{
    channel::Channel,
    gas::{Gas, GasPlan},
    io::{Output, OutputFrame},
    Amnis, Variable,
};

pub struct AmnisState {
    gas_plan: GasPlan,
    gas_used: Gas,
    variables: HashMap<i32, Variable>,
    channels: HashMap<i32, Channel>,
}

pub struct AmnisCore {
    state: Arc<Mutex<AmnisState>>,
}

#[async_trait::async_trait]
impl Amnis for AmnisCore {
    fn new(gas_plan: GasPlan) -> Self {
        AmnisCore {
            state: Arc::new(Mutex::new(AmnisState {
                gas_plan,
                gas_used: Gas::zero(),
                variables: HashMap::new(),
                channels: HashMap::from([(1, Channel::new())]),
            })),
        }
    }

    async fn handle(&self, input: Pin<Box<dyn AsyncRead + Send>>) -> Output {
        // Channel for muxing outputs
        // TODO config channel size somewhere, some compile flag I guess
        let (tx, mut rx) = mpsc::channel::<OutputFrame>(32);
        // Feed inputs to channels
        let state = self.state.clone();
        tokio::spawn(async move {
            drop(tx);
            drop(state);
            drop(input);
        });
        // Early return the output stream
        Output::new(Box::pin(async_stream::stream! {
            while let Some(item) = rx.recv().await {
                yield item;
            }
        }))
    }
}
