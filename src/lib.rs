use std::collections::VecDeque;
use bytes::Bytes;
use tokio_stream::StreamExt;

struct Interpreter {
    in_buffer: StreamExt,
    out_buffer: StreamExt,
    lines: VecDeque<Bytes>,
}
