use std::collections::HashMap;

use bytes::Bytes;

use crate::gas_plan::GasPlan;
use crate::variable::Variable;

pub struct Context {
    pub input: Bytes,
    pub output: Bytes,

    pub gas: GasPlan,
    pub variables: HashMap<String, Variable>,
}

impl Context {
    pub fn new(input: Bytes, output: Bytes, gas: GasPlan) -> Self {
        Self {
            input,
            output,
            gas,
            variables: HashMap::new(),
        }
    }
}
