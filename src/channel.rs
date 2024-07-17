use std::collections::VecDeque;

pub struct Channel {
    inner: VecDeque<Box<[u8]>>
}

impl Channel {
    pub fn new() -> Self {
        Channel {
            inner: VecDeque::new()
        }
    }
}
