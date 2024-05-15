use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use super::parser::parse_message;

#[derive(Clone, Debug)]
pub enum Response {
    NullBulkString(),
    NullArray(),
    SimpleString(String),
    BulkString(String),
    Array(Vec<Response>),
}

impl Response {
    // https://redis.io/docs/latest/develop/reference/protocol-spec/
    pub fn serialize(self) -> String {
        match self {
            Response::NullBulkString() => format!("$-1\r\n"),
            Response::NullArray() => format!("*-1\r\n"),
            Response::SimpleString(s) => format!("+{}\r\n", s),
            Response::BulkString(s) => format!("${}\r\n{}\r\n", s.len(), s),
            _ => panic!("Unsupported value for serialize!"),
        }
    }
}

pub struct ResponseHandler {
    stream: TcpStream,
    buffer: BytesMut,
}

impl ResponseHandler {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(512),
        }
    }

    pub async fn read_value(&mut self) -> Result<Option<Response>, String> {
        match self.stream.read_buf(&mut self.buffer).await {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return Ok(None);
                }
                match parse_message(self.buffer.split()) {
                    Ok((val, _)) => {
                        return Ok(Some(val));
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            Err(err) => Err(format!("Failed to read from stream: {}", err)),
        }
    }

    pub async fn write_value(&mut self, response: Response) -> Result<(), String> {
        let _ = self.stream.write(response.serialize().as_bytes()).await;
        Ok(())
    }
}
