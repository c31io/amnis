use std::collections::HashMap;

use bytes::Bytes;

use crate::Result;
use crate::{channel::Channel, gas::Gas, variable::Variable};

pub trait Amnis {
    fn new(gas_plan: Gas) -> Self
    where
        Self: Sized;

    fn evaluate(&mut self, input: Bytes) -> Result<Bytes> {
        loop {}
        todo!()
    }

    fn execute_ext(
        &mut self,
        channel: Bytes,
        name: Bytes,
        inputs: Box<[Bytes]>,
        input_bytes: Bytes,
        outputs: Box<[Bytes]>,
    ) -> Result<Bytes>;

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
    gas_plan: Gas,
    variables: HashMap<String, Variable>,
    channels: HashMap<String, Channel>,
}

impl Amnis for AmnisCore {
    fn new(gas_plan: Gas) -> Self {
        AmnisCore {
            gas_plan,
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
