use std::collections::HashMap;
use tokio::sync::mpsc::channel;

use crate::{
    channel::Channel,
    gas::{Gas, GasPlan},
    io::AmnisIO,
    Amnis, Result, Variable,
};

pub struct AmnisCore {
    gas_plan: GasPlan,
    gas_used: Gas,
    variables: HashMap<String, Variable>,
    channels: HashMap<i32, Channel>,
}

#[async_trait::async_trait]
impl Amnis for AmnisCore {
    fn new(gas_plan: GasPlan) -> Self {
        AmnisCore {
            gas_plan,
            gas_used: Gas::zero(),
            variables: HashMap::new(),
            channels: HashMap::from([(0, Channel::new())]),
        }
    }

    /// Handle a connection.
    async fn handle(&mut self, input: Box<dyn AmnisIO>) -> Result<Box<dyn AmnisIO>> {
        // forward to channel thread
        let (sender, mut receiver) = channel(128); // TODO config
        todo!()
    }
}
