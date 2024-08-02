use std::collections::HashMap;

use bytes::Bytes;
use tokio_stream::{Stream, StreamExt};

use crate::Result;
use crate::{channel::Channel, gas::Gas, variable::Variable};

#[async_trait::async_trait]
pub trait AmnisIO {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    async fn write(&mut self, buf: &[u8]) -> Result<usize>;
}

pub trait Amnis {
    fn new(gas_plan: Gas) -> Self
    where
        Self: Sized;

    fn evaluate(&mut self, input: AmnisIO) -> Result<AmnisIO> {
        todo!()
    }

    fn execute_ext(
        &mut self,
        channel: u32,
        function: i32,
        body: AmnisIO,
    ) -> Result<AmnisIO>;

    fn execute(
        &mut self,
        channel: Bytes,
        name: Bytes,
        inputs: Box<[Bytes]>,
        input_bytes: Bytes,
        outputs: Box<[Bytes]>,
    ) -> Result<Bytes>;
}

pub struct AmnisCore {
    gas_plan: GasPlan,
    gas_used: Gas,
    variables: HashMap<String, Variable>,
    channels: HashMap<String, Channel>,
}

impl Amnis for AmnisCore {
    fn new(gas_plan: GasPlan) -> Self {
        AmnisCore {
            gas_plan,
            gas_used: Gas::zero(),
            variables: HashMap::new(),
            channels: HashMap::from([("_".into(), Channel::new())]),
        }
    }

    fn execute_ext(
        &mut self,
        channel: Bytes,
        name: Bytes,
        inputs: Box<[Bytes]>,
        input_bytes: Bytes,
        outs: Box<[Bytes]>,
    ) -> Result<Bytes> {
        Err(crate::Error::FnNotFound)
    }

    fn execute(
        &mut self,
        channel: Bytes,
        name: Bytes,
        inputs: Box<[Bytes]>,
        input_bytes: Bytes,
        outputs: Box<[Bytes]>,
    ) -> Result<Bytes> {
        todo!()
    }
}
