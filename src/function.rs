use bytes::Bytes;

use crate::io::OutputFrame;
use crate::variable::Variable;
use crate::{Error, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Function {
    Null = 0,
    Echo = 1,
}

impl Function {
    pub fn new(id: i32) -> Result<Self> {
        let id: u8 = id.try_into().map_err(|_| Error::FnIdInvalid)?;
        unsafe { Ok(std::mem::transmute::<_, Function>(id)) }
    }

    pub fn get_id(&self) -> i32 {
        let id: i8 = unsafe { std::mem::transmute(*self) };
        id.into()
    }

    pub fn name_from_i32(i: i32) -> Result<String> {
        Ok("TODO".to_string()) //TODO impl
    }

    pub fn name_to_i32(n: &str) -> Result<i32> {
        Ok(0) //TODO impl
    }

    pub fn call(&self, args: Box<[Variable]>, channel: i32, line: i32, size: u64) -> OutputFrame
    where
        Self: Sized,
    {
        match self {
            Function::Null => OutputFrame::new(channel, line, size, Bytes::new()),
            Function::Echo => OutputFrame::new(channel, line, size, Bytes::new()), //TODO echo bin
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Function;

    #[test]
    fn test_transmute() {
        assert_eq!(Function::new(0).unwrap(), Function::Null);
        assert_eq!(Function::new(1).unwrap(), Function::Echo);
        assert_eq!(Function::Null.get_id(), 0);
        assert_eq!(Function::Echo.get_id(), 1);
    }
}
