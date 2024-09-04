use std::{pin::Pin, task::Poll};

use bytes::{Buf, Bytes};
use futures::stream::BoxStream;
use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, ReadBuf, Stdin};

pin_project! {
    /// Debug only, this is not a wire format.
    pub struct Utf8Input<T> {
        #[pin]
        inner: T,
    }
}

impl<T> Utf8Input<T> {
    pub fn new(inner: T) -> Self {
        Utf8Input { inner }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl AsyncRead for Utf8Input<Stdin> {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let mut slice = [0_u8; 1024];
        let mut read_buf = ReadBuf::new(&mut slice);
        match Pin::new(&mut self.inner).poll_read(cx, &mut read_buf) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(error)) => Poll::Ready(Err(error)),
            Poll::Ready(Ok(())) => {
                if let Some(n) = statement_size(buf.filled()) {
                    // get channel and function in i32
                    // call the function parser
                    // put the crap in buf
                    todo!()
                } else {
                    return Poll::Pending;
                }
            }
        }
    }
}

fn statement_size(slice: &[u8]) -> Option<usize> {
    todo!()
}

pub struct OutputFrame {
    channel: i32,
    line: i32,
    size: u64,
    payload: Bytes,
}

pub struct Output {
    inner: BoxStream<'static, OutputFrame>,
}

impl Output {
    pub fn new(stream: BoxStream<'static, OutputFrame>) -> Self {
        Output { inner: stream }
    }

    pub fn to_utf8(self) -> BoxStream<'static, String> {
        todo!()
    }
}
