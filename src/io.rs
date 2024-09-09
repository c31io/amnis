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
                let s = match String::from_utf8(buf.filled().to_vec()) {
                    Ok(s) => s,
                    Err(_) => return Poll::Ready(Err(std::io::ErrorKind::InvalidInput.into())),
                };
                if let Some(n) = statement_size(&s) {
                    let statement = Statement::new(&s[n..]);
                    statement.to_bytes();
                    Poll::Ready(Ok(()))
                } else {
                    return Poll::Pending;
                }
            }
        }
    }
}

/// Non-ascii might fuck up. No one uses unicode for names anyway.
fn statement_size(s: &str) -> Option<usize> {
    let full = s.len();
    let mut string = s.trim_start();
    let mut trimmed = string.len();
    let mut first_space = loop {
        match string.find(' ') {
            Some(n) => {
                // Indexing will not panic, because string is trimmed from the start.
                if string.as_bytes()[n - 1] != b'\\' {
                    break n;
                } else {
                    string = &string[n + 1..];
                    trimmed += n + 1;
                }
            }
            None => return None,
        }
    };
    first_space += full - trimmed;
    let mut first_parenthesis = loop {
        match string.find('(') {
            Some(n) => {
                // Indexing will not panic, because string is trimmed from the start.
                if string.as_bytes()[n - 1] != b'\\' {
                    break n;
                } else {
                    string = &string[n + 1..];
                    trimmed += n + 1;
                }
            }
            None => return None,
        }
    };
    first_parenthesis += full - trimmed;
    let function_name = &s[first_space + 1..first_parenthesis].trim();
    let first_lf = s.find('\n');
    let second_lf =
        first_lf.and_then(|fst| s[fst + 1..].find('\n').and_then(|snd| Some(snd + fst + 1)));
    let bytes_attached = is_attached_fn(function_name);
    if bytes_attached {
        return second_lf;
    } else {
        return first_lf;
    };
}

fn is_attached_fn(_s: &str) -> bool {
    return false;
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
