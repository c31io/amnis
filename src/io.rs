use bytes::Bytes;
use futures::stream::BoxStream;
use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, Stdin};

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
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        // read line
        // get channel and function
        // call the function parser
        todo!()
    }
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
