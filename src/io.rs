use std::{collections::HashMap, mem::MaybeUninit, pin::Pin, task::Poll};

use bytes::{BufMut, Bytes};
use futures::stream::BoxStream;
use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, ReadBuf, Stdin};

use crate::{Error, Function, Result};

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

    fn add_name(&mut self, name: &str) -> i32 {
        self.last_id += 1;
        self.name_from_id.insert(self.last_id, name.to_owned());
        self.name_to_id.insert(name.to_owned(), self.last_id);
        self.last_id
    }

    fn get_name(&self, id: &i32) -> Option<String> {
        Some(self.name_from_id.get(id)?.to_string())
    }

    fn get_id(&self, name: &str) -> Option<i32> {
        Some(*self.name_to_id.get(name)?)
    }

    fn try_get_id(&self, name: &str) -> Result<i32> {
        self.get_id(name).ok_or(Error::InvalidInput)
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
    inputs: Vec<i32>,
    outputs: Vec<i32>,
    body: Option<Box<[u8]>>,
}

impl Statement {
    fn take_tokens(tokens: &mut Vec<Token>, namespace: &mut Namespace) -> Result<Vec<Self>> {
        let mut statements = Vec::new();
        while let Some(end) = tokens.iter().position(|t| *t == Token::EndOfStatement) {
            let mut inputs = Vec::new();
            let mut input_index = 3;
            while let Token::Input(input) = &tokens[input_index] {
                let input = namespace.try_get_id(input)?;
                inputs.push(input);
                input_index += 1;
            }

            let mut outputs = Vec::new();
            let mut output_index = 1 + match tokens.iter().position(|t| *t == Token::InputEnd) {
                Some(i) => i,
                None => return Err(Error::InvalidInput),
            };
            while let Token::Output(output) = &tokens[output_index] {
                let output = namespace.add_name(output);
                outputs.push(output);
                output_index += 1;
            }

            statements.push(Statement {
                channel: match &tokens[0] {
                    Token::Channel(c) => namespace.try_get_id(c)?,
                    _ => return Err(Error::InvalidInput),
                },
                function: match tokens[1] {
                    Token::Function(f) => f,
                    _ => return Err(Error::InvalidInput),
                },
                inputs,
                outputs,
                body: match &tokens[end - 1] {
                    Token::Body(b) => Some(b.clone()),
                    Token::LineFeed => None,
                    _ => return Err(Error::InvalidInput),
                },
            });
            *tokens = tokens[end + 1..].to_vec();
        }
        Ok(statements)
    }

    fn write(&self, buf: &mut ReadBuf) {
        buf.put_i32(self.channel);
        buf.put_i32(self.function);
        self.inputs.iter().for_each(|&i| buf.put_i32(i));
        self.outputs.iter().for_each(|&i| buf.put_i32(i));
        if let Some(b) = &self.body {
            buf.put_slice(b);
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Token {
    Channel(String),
    Function(i32),
    InputStart,
    Input(String),
    InputEnd,
    Output(String),
    LineFeed,
    Body(Box<[u8]>),
    EndOfStatement,
}

fn first_non_whitespace_position(s: &str) -> Option<usize> {
    let mut i = 0;
    while i < s.len() {
        // safe since
        if !s.as_bytes()[i].is_ascii_whitespace() {
            break;
        }
        i += 1;
    }
    if i == s.len() {
        None
    } else {
        Some(i)
    }
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

    fn take_one(text: &str, tokens: &mut [Token]) -> Result<Option<(Self, usize)>> {
        // The last token as the parsing context.
        match tokens.last() {
            // Get Token::Channel
            Some(Token::EndOfStatement) | None => {
                let i = match first_non_whitespace_position(text) {
                    Some(i) => i,
                    None => return Ok(None),
                };
                match text[i..].find(char::is_whitespace) {
                    Some(j) => Ok(Some((Token::Channel(text[i..j].to_owned()), j))),
                    None => Ok(None),
                }
            }
            // Get Token::Function
            Some(Token::Channel(_)) => {
                let i = match first_non_whitespace_position(text) {
                    Some(i) => i,
                    None => return Ok(None),
                };
                match text[i..].find(char::is_whitespace) {
                    Some(j) => Ok(Some((
                        Token::Function(Function::name_to_i32(&text[i..j])?),
                        j,
                    ))),
                    None => Ok(None),
                }
            }
            // Get Token::InputStart
            Some(Token::Function(_)) => {
                let i = match first_non_whitespace_position(text) {
                    Some(i) => i,
                    None => return Ok(None),
                };
                match text.as_bytes()[i] {
                    b'(' => Ok(Some((Token::InputStart, i + 1))),
                    _ => Err(Error::InvalidInput),
                }
            }
            // Get Input or InputEnd
            Some(Token::InputStart | Token::Input(_)) => {
                let i = match first_non_whitespace_position(text) {
                    Some(i) => i,
                    None => return Ok(None),
                };
                match text.as_bytes()[i] {
                    b')' => Ok(Some((Token::InputEnd, i + 1))),
                    _ => match text[i..].find(char::is_whitespace) {
                        Some(j) => Ok(Some((Token::Input(text[i..j].to_owned()), j))),
                        None => Ok(None),
                    },
                }
            }
            // Get Output or LineFeed
            Some(Token::InputEnd | Token::Output(_)) => {
                let i = match first_non_whitespace_position(text) {
                    Some(i) => i,
                    None => return Ok(None),
                };
                match text.as_bytes()[i] {
                    b'\n' => Ok(Some((Token::LineFeed, i + 1))),
                    _ => match text[i..].find(char::is_whitespace) {
                        Some(j) => Ok(Some((Token::Output(text[i..j].to_owned()), j))),
                        None => Ok(None),
                    },
                }
            }
            Some(Token::LineFeed) => {
                // look back to function
                let mut i = tokens.len();
                let has_bin = loop {
                    // SAFETY: Function must exist in tokens.
                    i -= 1;
                    if let Token::Function(f) = tokens[i] {
                        break true; //TODO function.has_bin()
                    }
                };
                match has_bin {
                    false => Ok(Some((Token::EndOfStatement, 0))), //TODO length
                    true => {
                        todo!()
                    }
                }
                // if no bin, return eos
                // else get base64
            }
            Some(Token::Body(..)) => {
                // get eos
                todo!()
            }
        }
    }
}

pin_project! {
    /// Debug only, this is not the wire format.
    pub struct Utf8Input<T> {
        #[pin]
        inner: T,
        text: String,
        tokens: Vec<Token>,
        namespace: Namespace,
    }
}

impl<T> Utf8Input<T> {
    pub fn new(inner: T) -> Self {
        Utf8Input {
            inner,
            text: String::new(),
            tokens: Vec::new(),
            namespace: Namespace::new(),
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

    fn borrow3(&mut self) -> (&mut String, &mut Vec<Token>, &mut Namespace) {
        (&mut self.text, &mut self.tokens, &mut self.namespace)
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
                let (text, tokens, namespace) = self.borrow3();
                if Token::take(text, tokens).is_err() {
                    return Poll::Ready(Err(std::io::ErrorKind::InvalidInput.into()));
                }
                // write statements
                let statements = match Statement::take_tokens(tokens, namespace) {
                    Ok(s) => s,
                    Err(_) => return Poll::Ready(Err(std::io::ErrorKind::InvalidInput.into())),
                };
                if statements.is_empty() {
                    Poll::Pending
                } else {
                    statements.iter().for_each(|s| s.write(buf));
                    Poll::Ready(Ok(()))
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

impl OutputFrame {
    pub fn new(channel: i32, line: i32, size: u64, payload: Bytes) -> Self {
        OutputFrame {
            channel,
            line,
            size,
            payload,
        }
    }
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
