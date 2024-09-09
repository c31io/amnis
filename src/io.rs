use std::{
    collections::HashMap,
    mem::MaybeUninit,
    pin::Pin,
    sync::{Arc, Mutex},
    task::Poll,
};

use bytes::{Buf, Bytes};
use futures::stream::BoxStream;
use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, ReadBuf, Stdin};

struct Namespace {
    name_from_id: HashMap<i32, String>,
    name_to_id: HashMap<String, i32>,
    last_id: i32,
}

impl Namespace {
    fn new() -> Self {
        Namespace {
            name_from_id: HashMap::new(),
            name_to_id: HashMap::new(),
            last_id: 0,
        }
    }

    fn add_name(&mut self, name: &str) -> Option<i32> {
        match self.name_to_id.get(name) {
            Some(_) => None,
            None => {
                self.last_id += 1;
                self.name_from_id.insert(self.last_id, name.to_owned());
                self.name_to_id.insert(name.to_owned(), self.last_id);
                Some(self.last_id)
            }
        }
    }

    fn get_name(&self, id: &i32) -> Option<String> {
        Some(self.name_from_id.get(id)?.clone())
    }

    fn get_id(&self, name: &str) -> Option<i32> {
        Some(self.name_to_id.get(name)?.clone())
    }

    fn remove_name(&mut self, name: &str) -> Option<i32> {
        let id = self.name_to_id.get(name)?;
        self.name_from_id.remove(id);
        self.name_to_id.remove(name)
    }

    fn remove_id(&mut self, id: &i32) -> Option<String> {
        let name= self.name_from_id.get(id)?;
        self.name_to_id.remove(name);
        self.name_from_id.remove(id)
    }
}

struct Statement {
    channel: i32,
    function: i32,
    input: Box<[i32]>,
    output: Box<[i32]>,
    binary: Box<[u8]>,
}

enum Token {
    Channel(i32),
    Function(i32),
    Input(i32),
    Output(i32),
    Binary(Box<[u8]>),
}

impl Token {
    fn take(text: &mut String, tokens: &mut Vec<Token>) {
        let mut slice = text.as_str();
        while let Some((token, end)) = Token::take_one(slice, tokens) {
            tokens.push(token);
            slice = &slice[end..];
        }
    }

    fn take_one(text: &str, tokens: &mut Vec<Token>) -> Option<(Self, usize)> {
        match tokens.last() {
            Some(Token::Binary(_)) | None => todo!(),
            Some(Token::Channel(_)) => todo!(),
            Some(Token::Function(_)) => todo!(),
            Some(Token::Input(_)) => todo!(),
            Some(Token::Output(_)) => todo!(),
        }
        todo!()
    }
}

pin_project! {
    /// Debug only, this is not a wire format.
    pub struct Utf8Input<T> {
        #[pin]
        inner: T,
        buf: [MaybeUninit<u8>; 1024],
        text: String,
        tokens: Vec<Token>,
    }
}

impl<T> Utf8Input<T> {
    pub fn new(inner: T) -> Self {
        Utf8Input {
            inner,
            buf: MaybeUninit::uninit_array(),
            text: String::new(),
            tokens: Vec::new(),
        }
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
        let mut read_buf = ReadBuf::uninit(&mut self.buf);
        match Pin::new(&mut self.inner).poll_read(cx, &mut read_buf) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(error)) => Poll::Ready(Err(error)),
            Poll::Ready(Ok(())) => {
                let s = match String::from_utf8(buf.filled().to_vec()) {
                    Ok(s) => s,
                    Err(_) => return Poll::Ready(Err(std::io::ErrorKind::InvalidInput.into())),
                };
                self.text.push_str(&s);
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
