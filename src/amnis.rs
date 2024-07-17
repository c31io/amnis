use std::collections::HashMap;

use bytes::Bytes;

use crate::{
    channel::Channel, function::Function, gas_plan::GasPlan, variable::Variable,
};

pub struct Amnis {
    gas_plan: GasPlan,
    variables: HashMap<String, Variable>,
    channels: HashMap<String, Channel>,
    functions: HashMap<String, Box<dyn Function>>,
}

impl Amnis {
    pub fn new(gas_plan: GasPlan) -> Self {
        Amnis {
            gas_plan,
            variables: HashMap::new(),
            channels: HashMap::from([("_".into(), Channel::new())]),
            functions: HashMap::new(),
        }
    }

    pub fn exec(input: Bytes) -> Bytes {
        Bytes::new()
    }

    pub fn register(name: String, function: impl Function) {}
}
