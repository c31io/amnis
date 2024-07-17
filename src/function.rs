use bytes::Bytes;

use crate::variable::Variable;

pub trait Function {
    fn call(args: Box<[Variable]>) -> Bytes where Self: Sized;
}
