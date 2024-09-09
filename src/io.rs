use std::{
    collections::HashMap,
    mem::MaybeUninit,
    pin::Pin,
    task::Poll,
};

use bytes::Bytes;
use futures::stream::BoxStream;
use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, ReadBuf, Stdin};

use crate::Result;

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
        let name = self.name_from_id.get(id)?;
        self.name_to_id.remove(name);
        self.name_from_id.remove(id)
    }
}

struct Statement {
    channel: i32,
    function: i32,
    input: Box<[i32]>,
    output: Box<[i32]>,
    body: Option<Box<[u8]>>,
}

impl Statement {
    fn new(tokens: &mut Vec<Token>) -> Vec<Self> {
        todo!()
    }

    fn write(&self, dest: &mut ReadBuf) {
        // write self to buffer
        todo!()
    }
}

pub enum Token {
    Channel(i32),
    Function(i32),
    InputStart,
    Input(i32),
    InputEnd,
    Output(i32),
    LineFeed,
    Body(Box<[u8]>),
    EndOfStatement,
}

impl Token {
    fn take(text: &mut String, tokens: &mut Vec<Token>) -> Result<()> {
        let mut slice = text.as_str();
        let mut consumption = 0;
        while let Some((token, end)) = Token::take_one(slice, tokens)? {
            tokens.push(token);
            slice = &slice[end..];
            consumption += end;
        }
        *text = text[consumption..].to_owned();
        Ok(())
    }

    fn take_one(text: &str, tokens: &mut Vec<Token>) -> Result<Option<(Self, usize)>> {
        match tokens.last() {
            Some(Token::EndOfStatement) | None => {
                // get channel
                todo!()
            }
            Some(Token::Channel(_)) => {
                // get function
                todo!()
            }
            Some(Token::Function(_)) => {
                // get input start
                todo!()
            }
            Some(Token::InputStart | Token::Input(_)) => {
                // get input or input end
                todo!()
            }
            Some(Token::InputEnd | Token::Output(_)) => {
                // get output or lfao
                todo!()
            }
            Some(Token::LineFeed) => {
                // look back to function
                // if no bin, return eos
                // else get base64
                todo!()
            }
            Some(Token::Body(..)) => {
                // get eos
                todo!()
            },
        }
    }
}

pin_project! {
    /// Debug only, this is not a wire format.
    pub struct Utf8Input<T> {
        #[pin]
        inner: T,
        text: String,
        tokens: Vec<Token>,
    }
}

impl<T> Utf8Input<T> {
    pub fn new(inner: T) -> Self {
        Utf8Input {
            inner,
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

    pub fn borrow_two(&mut self) -> (&mut String, &mut Vec<Token>) {
        (&mut self.text, &mut self.tokens)
    }
}

impl AsyncRead for Utf8Input<Stdin> {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let mut inner_buf: [MaybeUninit<u8>; 1024] = MaybeUninit::uninit_array();
        let mut read_buf = ReadBuf::uninit(&mut inner_buf);
        match Pin::new(&mut self.inner).poll_read(cx, &mut read_buf) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(error)) => Poll::Ready(Err(error)),
            Poll::Ready(Ok(())) => {
                let s = match String::from_utf8(read_buf.filled().to_vec()) {
                    Ok(s) => s,
                    Err(_) => return Poll::Ready(Err(std::io::ErrorKind::InvalidInput.into())),
                };
                self.text.push_str(&s);
                // get tokens
                let (text, tokens) = self.borrow_two();
                if Token::take(text, tokens).is_err() {
                    return Poll::Ready(Err(std::io::ErrorKind::InvalidInput.into()));
                }
                // write statements
                let statements = Statement::new(tokens);
                if statements.is_empty() {
                    return Poll::Pending;
                } else {
                    statements.iter().for_each(|s| s.write(buf));
                    return Poll::Ready(Ok(()))
                }
            }
        }
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
