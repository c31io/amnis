use bytes::Bytes;

use crate::Result;
use crate::variable::Variable;

pub struct Function {}

impl Function {
    pub fn name_from_i32(i: i32) -> Result<String> {
        Ok("TODO".to_string()) //TODO impl
    }

    pub fn name_to_i32(n: &str) -> Result<i32> {
        Ok(0) //TODO impl
    }

    pub fn call(args: Box<[Variable]>) -> Bytes where Self: Sized {
        Bytes::new()
    }
}
